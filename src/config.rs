use crate::{error::ErrorCode, hosthname::generate_hostname, mount::generate_rootpath};
use std::{ffi::CString, os::fd::RawFd, path::PathBuf};

#[derive(Clone)]
pub struct ContainerOpts {
    pub path: CString,
    pub argv: Vec<CString>,
    pub uid: u32,
    pub mount_dir: PathBuf,
    pub fd: RawFd,
    pub hostname: String,
    pub root_path: String,
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
        let hostname = generate_hostname()?;
        let root_path = generate_rootpath()?;

        Ok(Self {
            path,
            argv,
            uid,
            mount_dir,
            fd,
            hostname,
            root_path,
        })
    }
}
