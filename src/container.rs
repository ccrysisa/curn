use crate::{
    cgroup::{clean_cgroups, restrict_resources},
    child::generate_child_process,
    cli::Args,
    config::ContainerOpts,
    ebpf::{clean_ebpf_program, generate_ebpf_program},
    error::ErrorCode,
    ipc::generate_socketpair,
    mount::clean_mounts,
    user_namespace::handle_child_uid_gid_map,
};
use nix::{
    sys::{utsname::uname, wait::waitpid},
    unistd::{close, Pid},
};
use rand::Rng;
use std::{os::fd::RawFd, path::PathBuf};

const MINIMAL_KERNEL_VERSION: f64 = 5.4; // kernel version of Ubuntu 20.04 LTS

pub struct Container {
    config: ContainerOpts,
    sockets: (RawFd, RawFd),
    child_pid: Option<Pid>,
    ebpf_pid: Option<Pid>,
}

impl Container {
    pub fn new(args: Args) -> Result<Self, ErrorCode> {
        let mut add_paths = Vec::new();
        for x in args.add_paths.iter() {
            let pair = x.to_str().unwrap().split(":").collect::<Vec<_>>();
            let from_path = PathBuf::from(pair[0])
                .canonicalize()
                .expect("Cannot canonicalize path");
            let mnt_path = PathBuf::from(
                pair[1]
                    .to_string()
                    .strip_prefix("/")
                    .expect("Cannot canonicalize path"),
            );
            add_paths.push((from_path, mnt_path));
        }

        let sockets = generate_socketpair()?;
        let config = ContainerOpts::new(
            args.command,
            args.uid,
            args.mount_dir,
            sockets.1,
            add_paths,
            args.tool_dir,
        )?;

        Ok(Self {
            config,
            sockets,
            child_pid: None,
            ebpf_pid: None,
        })
    }

    pub fn create(&mut self) -> Result<(), ErrorCode> {
        let pid = generate_child_process(&self.config)?;
        let ebpf_pid = generate_ebpf_program(self.config.container_id.clone(), pid.as_raw())?;
        restrict_resources(&self.config.hostname, pid)?;
        handle_child_uid_gid_map(pid, self.sockets.0)?;
        self.child_pid = Some(pid);
        self.ebpf_pid = Some(ebpf_pid);

        log::debug!("Creation finished");
        Ok(())
    }

    pub fn clean_exit(&mut self) -> Result<(), ErrorCode> {
        log::debug!("Cleaning container");

        if let Err(e) = close(self.sockets.0) {
            log::error!("Unable to close write socket of parent: {:?}", e);
            return Err(ErrorCode::SocketError(3));
        }
        if let Err(e) = close(self.sockets.1) {
            log::error!("Unable to close read socket of child: {:?}", e);
            return Err(ErrorCode::SocketError(4));
        }
        clean_mounts(&self.config.root_path)?;
        clean_cgroups(&self.config.hostname)?;
        clean_ebpf_program(self.ebpf_pid.expect("eBPF program pid must be valid"))?;

        log::debug!("Clean finished");
        Ok(())
    }
}

pub fn start(args: Args) -> Result<(), ErrorCode> {
    check_linux_version()?;

    let mut container = Container::new(args)?;

    if let Err(e) = container.create() {
        log::error!("Error while creating container: {:?}", e);
        return Err(e);
    }
    log::debug!("Container child process PID: {:?}", container.child_pid);
    wait_child(container.child_pid)?;

    log::debug!("Execution finished, now cleaning and exit");
    container.clean_exit()
}

fn check_linux_version() -> Result<(), ErrorCode> {
    log::debug!("Checking linux release");

    let host = uname().expect("Cannot get uname of host");
    let release = host.release().to_str().expect("Release must be valid");
    log::debug!("Linux release: {}", release);

    if let Ok(version) = scan_fmt!(release, "{f}.{}", f64) {
        if version < MINIMAL_KERNEL_VERSION {
            return Err(ErrorCode::NotSupported(0));
        }
    } else {
        return Err(ErrorCode::ContainerError(0));
    }

    if host.machine() != "x86_64" {
        return Err(ErrorCode::NotSupported(1));
    }

    Ok(())
}

fn wait_child(pid: Option<Pid>) -> Result<(), ErrorCode> {
    match pid {
        Some(pid) => {
            log::debug!("Waiting for child process (pid {}) to finish", pid);
            if let Err(e) = waitpid(pid, None) {
                log::error!("Error while waiting for pid to finish: {:?}", e);
                return Err(ErrorCode::ContainerError(1));
            }
            Ok(())
        }
        None => {
            log::error!("Invalid pid of waiting process");
            Err(ErrorCode::ContainerError(1))
        }
    }
}

pub fn generate_container_id() -> Result<String, ErrorCode> {
    Ok(format!("cunrc.{}", random_string(12)))
}

/// Generate a n-char String
fn random_string(n: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();

    let result: String = (0..n)
        .map(|_| {
            let i = rng.gen_range(0..CHARSET.len());
            CHARSET[i] as char
        })
        .collect();
    result
}
