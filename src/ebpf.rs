use crate::error::ErrorCode;
use libc::c_int;
use nix::{
    fcntl::{open, OFlag},
    sched::{clone, CloneFlags},
    sys::{
        signal::{kill, Signal},
        stat::Mode,
    },
    unistd::{dup2, execve, Pid},
};
use std::{ffi::CString, fs, path::PathBuf};

const STACK_SIZE: usize = 1024 * 1024;

fn ebpf_program(conatiner_id: String, pid: i32) -> isize {
    log::info!(
        "Starting container with command `{}` and args {:?}",
        "./ecli",
        vec!["run", "package.json"]
    );

    let path = PathBuf::from(format!("./logs/{}", conatiner_id));
    if let Err(e) = fs::File::create(&path) {
        log::error!("Error while executing eBPF program: {:?}", e);
        return -1;
    }

    match open(&path, OFlag::O_WRONLY, Mode::empty()) {
        Ok(fd) => {
            let _ = dup2(fd, 1);
            let _ = dup2(fd, 2);
        }
        Err(e) => {
            log::error!("Error while trying to perfoem execve: {:?}", e);
            return -1;
        }
    }

    match execve::<CString, CString>(
        &CString::new("./ecli").unwrap(),
        &[
            CString::new("run").unwrap(),
            CString::new("package.json").unwrap(),
            CString::new("--ppid_target").unwrap(),
            CString::new(format!("{}", pid)).unwrap(),
        ],
        &[],
    ) {
        Ok(_) => 0,
        Err(e) => {
            log::error!("Error while trying to perfoem execve: {:?}", e);
            -1
        }
    }
}

pub fn generate_ebpf_program(conatiner_id: String, pid: i32) -> Result<Pid, ErrorCode> {
    log::debug!("Cloning eBPF user process");

    let mut tmp_stack: [u8; STACK_SIZE] = [0; STACK_SIZE];
    unsafe {
        match clone(
            Box::new(|| ebpf_program(conatiner_id.clone(), pid)),
            &mut tmp_stack,
            CloneFlags::empty(),
            Some(Signal::SIGCHLD as c_int),
        ) {
            Ok(pid) => Ok(pid),
            Err(_) => Err(ErrorCode::ChildProcessError(0)),
        }
    }
}

pub fn clean_ebpf_program(pid: Pid) -> Result<(), ErrorCode> {
    log::debug!("Cleaning eBPF program (pid {})", pid);

    kill(pid, Signal::SIGTERM).map_err(|_| ErrorCode::ContainerError(2))?;
    Ok(())
}
