use crate::error::ErrorCode;
use std::{ffi::CString, os::fd::RawFd, path::PathBuf};

#[derive(Clone)]
pub struct ContainerOpts {
    pub path: CString,
    pub argv: Vec<CString>,
    pub uid: u32,
    pub mount_dir: PathBuf,
    pub fd: RawFd,
}

impl ContainerOpts {
    pub fn new(
        command: String,
        uid: u32,
        mount_dir: PathBuf,
        fd: RawFd,
    ) -> Result<Self, ErrorCode> {
        let argv: Vec<CString> = command
            .split_ascii_whitespace()
            .map(|s| CString::new(s).expect("Cannot read argument"))
            .collect();
        let path = argv[0].clone();

        Ok(Self {
            path,
            argv,
            uid,
            mount_dir,
            fd,
        })
    }
}
