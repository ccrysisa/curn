use crate::error::ErrorCode;
use cgroups_rs::{cgroup_builder::CgroupBuilder, hierarchies::V2, CgroupPid, MaxValue};
use nix::unistd::Pid;
use rlimit::{setrlimit, Resource};
use std::fs::{canonicalize, remove_dir};

const KMEM_LIMIT: i64 = 1024 * 1024 * 1024;
const MEM_LIMIT: i64 = KMEM_LIMIT;
const MAX_PID: MaxValue = MaxValue::Value(64);
const NOFILE_RLIMIT: u64 = 64;

pub fn restrict_resources(hostname: &String, pid: Pid) -> Result<(), ErrorCode> {
    log::debug!("Restricting resources for hostname {}", hostname);

    let cgs = CgroupBuilder::new(&hostname)
        .cpu()
        .shares(256)
        .done()
        .memory()
        .kernel_memory_limit(KMEM_LIMIT)
        .memory_hard_limit(MEM_LIMIT)
        .done()
        .pid()
        .maximum_number_of_processes(MAX_PID)
        .done()
        .blkio()
        .weight(50)
        .done()
        .build(Box::new(V2::new()))
        .map_err(|_| ErrorCode::CgroupError(0))?;

    let pid: u64 = pid
        .as_raw()
        .try_into()
        .expect("pid (i32) should be convert to u64");
    if let Err(e) = cgs.add_task_by_tgid(CgroupPid::from(pid)) {
        log::error!("{}", e);
        return Err(ErrorCode::CgroupError(1));
    }

    if let Err(e) = setrlimit(Resource::NOFILE, NOFILE_RLIMIT, NOFILE_RLIMIT) {
        log::error!("{}", e);
        return Err(ErrorCode::CgroupError(2));
    }

    Ok(())
}

pub fn clean_cgroups(hostname: &String) -> Result<(), ErrorCode> {
    log::debug!("Cleaning cgruops");

    match canonicalize(format!("/sys/fs/cgroup/{}/", hostname)) {
        Ok(d) => match remove_dir(d) {
            Ok(_) => Ok(()),
            Err(_) => Err(ErrorCode::CgroupError(3)),
        },
        Err(e) => {
            log::error!("Error while canonicalize path: {}", e);
            Err(ErrorCode::CgroupError(4))
        }
    }
}
