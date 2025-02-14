use std::ffi::CString;

use crate::{
    capabilities::set_capabilities, config::ContainerOpts, error::ErrorCode,
    hosthname::set_container_hostname, mount::set_mounts, syscall::set_syscalls, tools::set_tools,
    user_namespace::set_user_namespace,
};
use nix::{
    libc::c_int,
    sched::{clone, CloneFlags},
    sys::signal::Signal,
    unistd::{close, execve, Pid},
};

const STACK_SIZE: usize = 1024 * 1024; // 1MB stack of child process

fn setup_container_configuration(config: &ContainerOpts) -> Result<(), ErrorCode> {
    set_container_hostname(&config.hostname)?;
    set_mounts(&config.mount_dir, &config.root_path)?;
    set_user_namespace(config.fd, config.uid)?;
    set_capabilities()?;
    set_syscalls()?;
    set_tools()?;

    Ok(())
}

fn child(config: ContainerOpts) -> isize {
    match setup_container_configuration(&config) {
        Ok(_) => {
            log::info!("Container set up successfully");
        }
        Err(e) => {
            log::error!("Error while configuring container: {:?}", e);
            return -1;
        }
    }

    if let Err(_) = close(config.fd) {
        log::error!("Error while closing socket...");
    }

    log::info!(
        "Starting container with command `{}` and args {:?}",
        config.path.to_str().expect("command must be valid"),
        config.argv
    );

    let environments = [CString::new("TERM=xterm").expect("Must be valid")];
    match execve::<CString, CString>(&config.path, &config.argv, &environments) {
        Ok(_) => 0,
        Err(e) => {
            log::error!("Error while trying to perfoem execve: {:?}", e);
            -1
        }
    }
}

pub fn generate_child_process(config: &ContainerOpts) -> Result<Pid, ErrorCode> {
    log::debug!("Cloning child process");

    let mut tmp_stack: [u8; STACK_SIZE] = [0; STACK_SIZE];
    let mut flags = CloneFlags::empty();
    flags.insert(CloneFlags::CLONE_NEWNS);
    flags.insert(CloneFlags::CLONE_NEWCGROUP);
    flags.insert(CloneFlags::CLONE_NEWPID);
    flags.insert(CloneFlags::CLONE_NEWIPC);
    flags.insert(CloneFlags::CLONE_NEWNET);
    flags.insert(CloneFlags::CLONE_NEWUTS);

    unsafe {
        match clone(
            Box::new(|| child(config.clone())),
            &mut tmp_stack,
            flags,
            Some(Signal::SIGCHLD as c_int),
        ) {
            Ok(pid) => Ok(pid),
            Err(_) => Err(ErrorCode::ChildProcessError(0)),
        }
    }
}
