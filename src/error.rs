use std::{fmt, process::exit};

#[derive(Debug)]
pub enum ErrorCode {
    ArgumentInvaild(&'static str),
    NotSupported(u8),
    ContainerError(u8),
    SocketError(u8),
    ChildProcessError(u8),
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
                    _ => "Unknown reason",
                };
                write!(f, "Container Error by: {}", reason)
            }
            ErrorCode::SocketError(element) => {
                let reason = match element {
                    0 => "Cannot generate a pair of connected sockets",
                    1 => "Cannot send bool value through socket",
                    2 => "Cannot receive bool value through socket",
                    3 => "Cannot close write socket of parent",
                    4 => "Cannot close read socket of child",
                    _ => "Unknown reason",
                };
                write!(f, "Socket Error: {}", reason)
            }
            ErrorCode::ChildProcessError(_) => write!(f, "Cloen child process failed"),
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
