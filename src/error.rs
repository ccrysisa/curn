use std::{fmt, process::exit};

#[derive(Debug)]
pub enum ErrorCode {}

impl ErrorCode {
    pub fn get_retcode(&self) -> i32 {
        1
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            _ => write!(f, "{:?}", self),
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
