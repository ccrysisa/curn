use crate::{
    error::ErrorCode,
    ipc::{recv_bool, send_bool},
};
use nix::{
    sched::{unshare, CloneFlags},
    unistd::{setgroups, setresgid, setresuid, Gid, Pid, Uid},
};
use std::{fs::File, io::Write, os::fd::RawFd};

const USERNS_OFFSET: u64 = 10000;
const USERNS_COUNT: u64 = 2000;

pub fn set_user_namespace(fd: RawFd, uid: u32) -> Result<(), ErrorCode> {
    log::debug!("Setting up user namespaces with UID {}", uid);

    let has_userns = match unshare(CloneFlags::CLONE_NEWUSER) {
        Ok(_) => true,
        Err(_) => false,
    };
    send_bool(fd, has_userns)?;
    if recv_bool(fd)? {
        return Err(ErrorCode::NamespacesError(0));
    }

    if has_userns {
        log::info!("User namespaces has been set up");
    } else {
        log::info!("User namespaces not supported, continuing");
    }

    log::debug!("Switching to UID {} and GID {} ...", uid, uid);

    let gid = Gid::from_raw(uid);
    let uid = Uid::from_raw(uid);

    if let Err(_) = setgroups(&[gid]) {
        return Err(ErrorCode::NamespacesError(1));
    }

    if let Err(_) = setresgid(gid, gid, gid) {
        return Err(ErrorCode::NamespacesError(2));
    }

    if let Err(_) = setresuid(uid, uid, uid) {
        return Err(ErrorCode::NamespacesError(2));
    }

    Ok(())
}

pub fn handle_child_uid_gid_map(pid: Pid, fd: RawFd) -> Result<(), ErrorCode> {
    match recv_bool(fd)? {
        true => {
            if let Ok(mut uid_map) = File::create(format!("/proc/{}/{}", pid.as_raw(), "uid_map")) {
                if let Err(_) =
                    uid_map.write_all(format!("0 {} {}", USERNS_OFFSET, USERNS_COUNT).as_bytes())
                {
                    return Err(ErrorCode::NamespacesError(4));
                }
            } else {
                return Err(ErrorCode::NamespacesError(5));
            }

            if let Ok(mut gid_map) = File::create(format!("/proc/{}/{}", pid.as_raw(), "gid_map")) {
                if let Err(_) =
                    gid_map.write_all(format!("0 {} {}", USERNS_OFFSET, USERNS_COUNT).as_bytes())
                {
                    return Err(ErrorCode::NamespacesError(6));
                }
            } else {
                return Err(ErrorCode::NamespacesError(7));
            }
        }
        false => log::info!("No user namespace set up from child process"),
    }

    log::debug!("Child UID/GID map done, sending signal to child to continue ...");
    send_bool(fd, false)
}
