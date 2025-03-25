use std::{fmt, process::exit};

#[derive(Debug)]
pub enum ErrorCode {
    ArgumentInvaild(&'static str),
    NotSupported(u8),
    ContainerError(u8),
    SocketError(u8),
    ChildProcessError(u8),
    RngError,
    HostnameError(u8),
    MountError(u8),
    NamespacesError(u8),
    CapabilitiesError(u8),
    SyscallError(u8),
    CgroupError(u8),
}

impl ErrorCode {
    pub fn get_retcode(&self) -> i32 {
        1
    }
}

#[allow(unreachable_patterns)]
impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorCode::ArgumentInvaild(element) => write!(f, "Invalid argument: {}", element),
            ErrorCode::NotSupported(element) => {
                let reason = match element {
                    0 => "Kernel version",
                    1 => "Machine architecture",
                    _ => "Unknown reason",
                };
                write!(f, "Not supported by: {}", reason)
            }
            ErrorCode::ContainerError(element) => {
                let reason = match element {
                    0 => "Hardware and OS donot support container",
                    1 => "Error while waiting for pid to finish",
                    2 => "Error while killing a process",
                    _ => "Unknown reason",
                };
                write!(f, "Container Error by: {}", reason)
            }
            ErrorCode::SocketError(element) => {
                let reason = match element {
                    0 => "Cannot generate a pair of connected sockets",
                    1 => "Cannot send value through socket",
                    2 => "Cannot receive value through socket",
                    3 => "Cannot close write socket of parent",
                    4 => "Cannot close read socket of child",
                    _ => "Unknown reason",
                };
                write!(f, "Socket Error: {}", reason)
            }
            ErrorCode::RngError => write!(f, "Failed to random choose"),
            ErrorCode::HostnameError(_element) => write!(f, "Cannot set up hostname for container"),
            ErrorCode::ChildProcessError(_element) => write!(f, "Clone child process failed"),
            ErrorCode::MountError(element) => {
                let reason = match element {
                    0 => "Failed to mount file system",
                    1 => "Failed to unmount file system",
                    2 => "Failed to create directory or file by given path",
                    3 => "Failed to delete empty directory",
                    4 => "Failed to pivot root",
                    5 => "Failed to change working directory to root",
                    _ => "Unknown reason",
                };
                write!(f, "Mount Error: {}", reason)
            }
            ErrorCode::NamespacesError(element) => {
                let reason = match element {
                    0 => "Failed to map UID and GID",
                    1 => "Failed to set groups",
                    2 => "Failed to set GID",
                    3 => "Failed to set UID",
                    4 => "Failed to write uid_map file",
                    5 => "Failed to create uid_map file",
                    6 => "Failed to write gid_map file",
                    7 => "Failed to create gid_map file",
                    _ => "Unknown reason",
                };
                write!(f, "Namespace Error: {}", reason)
            }
            ErrorCode::CapabilitiesError(_element) => write!(f, "Failed to restrict capabilities"),
            ErrorCode::SyscallError(element) => {
                let reason = match element {
                    0 => "Failed to load seccomp policy",
                    1 => "Failed to create seccomp context",
                    2 => "Failed to set action for syscall",
                    3 => "Failed to set rule for syscall",
                    _ => "Unknown reason",
                };
                write!(f, "Syscall Error: {}", reason)
            }
            ErrorCode::CgroupError(element) => {
                let reason = match element {
                    0 => "Failed to build a control group",
                    1 => "Failed to attach task to control group",
                    2 => "Failed to set resource limits",
                    3 => "Failed to remove directory",
                    4 => "Failed to canonicalize path",
                    _ => "Unknown reason",
                };
                write!(f, "Cgroup Error: {}", reason)
            }
            _ => write!(f, "Unknown Error: {:?}", self),
        }
    }
}

pub fn exit_with_retcode(res: Result<(), ErrorCode>) {
    match res {
        Ok(_) => {
            log::debug!("Exit without any error, return 0");
            exit(0);
        }
        Err(e) => {
            let retcode = e.get_retcode();
            log::error!("Error on exit:\n\t{}\n\tReturn {}", e, retcode);
            exit(retcode);
        }
    }
}
