use crate::{
    container::generate_container_id, error::ErrorCode, hosthname::generate_hostname,
    mount::generate_rootpath,
};
use std::{ffi::CString, os::fd::RawFd, path::PathBuf};

#[derive(Clone)]
pub struct ContainerOpts {
    pub path: CString,
    pub argv: Vec<CString>,
    pub uid: u32,
    pub mount_dir: PathBuf,
    pub fd: RawFd,
    pub hostname: String,
    pub container_id: String,
    pub root_path: String,
    pub add_paths: Vec<(PathBuf, PathBuf)>,
    pub tool_dir: Option<PathBuf>,
}

impl ContainerOpts {
    pub fn new(
        command: String,
        uid: u32,
        mount_dir: PathBuf,
        fd: RawFd,
        add_paths: Vec<(PathBuf, PathBuf)>,
        tool_dir: Option<PathBuf>,
    ) -> Result<Self, ErrorCode> {
        let argv: Vec<CString> = command
            .split_ascii_whitespace()
            .map(|s| CString::new(s).expect("Cannot read argument"))
            .collect();
        let path = argv[0].clone();
        let hostname = generate_hostname()?;
        let container_id = generate_container_id()?;
        let root_path = generate_rootpath(&container_id)?;

        Ok(Self {
            path,
            argv,
            uid,
            mount_dir,
            fd,
            hostname,
            container_id,
            root_path,
            add_paths,
            tool_dir,
        })
    }
}
