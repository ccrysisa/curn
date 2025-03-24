use crate::error::ErrorCode;
use nix::{
    mount::{mount, umount2, MntFlags, MsFlags},
    unistd::{chdir, pivot_root},
};
use rand::Rng;
use std::{
    fs::{create_dir_all, remove_dir},
    path::PathBuf,
};

/// Return mounted path, e.g. /tmp/cunrc.xxx...
pub fn generate_rootpath() -> Result<String, ErrorCode> {
    Ok(format!("/tmp/cunrc.{}", random_string(12)))
}

pub fn set_mounts(
    mount_dir: &PathBuf,
    root_path: &String,
    add_paths: &Vec<(PathBuf, PathBuf)>,
    tool_dir: Option<&PathBuf>,
) -> Result<(), ErrorCode> {
    log::debug!("Setting mount points ...");

    // remount root `/` by private mount namespace
    mount_directory(
        None,
        &PathBuf::from("/"),
        None,
        vec![MsFlags::MS_REC, MsFlags::MS_PRIVATE],
    )?;

    // create new root directory and mount root to it
    log::debug!(
        "Mounting container's root to temp directory `{}`",
        root_path
    );

    let new_root = PathBuf::from(root_path);
    create_directory(&new_root)?;
    mount_directory(
        Some(&mount_dir),
        &new_root,
        None,
        vec![MsFlags::MS_BIND, MsFlags::MS_PRIVATE],
    )?;

    // mount additional volumes
    log::debug!("Mounting additional volumes");
    for (from_path, mnt_path) in add_paths.iter() {
        log::debug!(
            "Mount host's {} to container's /{}",
            from_path.to_str().unwrap(),
            mnt_path.to_str().unwrap()
        );

        let mnt_path = new_root.join(mnt_path);
        create_directory(&mnt_path)?;
        mount_directory(
            Some(from_path),
            &mnt_path,
            None,
            vec![MsFlags::MS_PRIVATE, MsFlags::MS_BIND],
        )?;
    }

    if let Some(tool_dir) = tool_dir {
        // mount tool volume
        let tool_mnt_point = new_root.join("curn");
        create_directory(&tool_mnt_point)?;
        mount_directory(
            Some(&tool_dir),
            &tool_mnt_point,
            None,
            vec![MsFlags::MS_BIND, MsFlags::MS_PRIVATE],
        )?;
    }

    // pivot and change working path to the new root
    log::debug!("Pivoting root");

    let old_root_tail = format!("oldroot.{}", random_string(6));
    let put_old = new_root.join(&old_root_tail);
    create_directory(&put_old)?;

    if let Err(_) = pivot_root(&new_root, &put_old) {
        return Err(ErrorCode::MountError(4));
    }

    if let Err(_) = chdir(&PathBuf::from("/")) {
        return Err(ErrorCode::MountError(5));
    }

    // unmount old root and delete temp directory
    log::debug!("Unmounting old root");

    let old_root = PathBuf::from("/").join(&old_root_tail);
    unmount_directory(&old_root)?;
    delete_directory(&old_root)?;

    // mount proc filesystem of container
    mount_directory(
        Some(&PathBuf::from("proc")),
        &PathBuf::from("/proc"),
        Some(&PathBuf::from("proc")),
        vec![],
    )?;

    Ok(())
}

pub fn clean_mounts(path: &String) -> Result<(), ErrorCode> {
    log::debug!("Cleaning mount points: {}", path);

    let root_mnt_point = PathBuf::from(&path);
    delete_directory(&root_mnt_point)?;

    Ok(())
}

fn mount_directory(
    path: Option<&PathBuf>,
    mount_point: &PathBuf,
    fstype: Option<&PathBuf>,
    flags: Vec<MsFlags>,
) -> Result<(), ErrorCode> {
    let mut ms_flags = MsFlags::empty();
    for f in flags {
        ms_flags.insert(f);
    }

    match mount::<_, _, PathBuf, PathBuf>(path, mount_point, fstype, ms_flags, None) {
        Ok(_) => Ok(()),
        Err(e) => {
            if let Some(p) = path {
                log::error!(
                    "Cannot mount `{}` to `{}`: {}",
                    p.to_str().expect("Path to be mounted must be valid"),
                    mount_point.to_str().expect("Mount point must be valid"),
                    e
                );
            } else {
                log::error!(
                    "Cannot remount `{}`: {}",
                    mount_point.to_str().expect("Mount point must be valid"),
                    e
                );
            }
            Err(ErrorCode::MountError(0))
        }
    }
}

fn unmount_directory(path: &PathBuf) -> Result<(), ErrorCode> {
    match umount2(path, MntFlags::MNT_DETACH) {
        Ok(_) => Ok(()),
        Err(e) => {
            log::error!(
                "Unable to umount `{}`: {}",
                path.to_str().expect("Path to be unmounted must be valid"),
                e
            );
            Err(ErrorCode::MountError(1))
        }
    }
}

fn create_directory(path: &PathBuf) -> Result<(), ErrorCode> {
    match create_dir_all(path) {
        Ok(_) => Ok(()),
        Err(e) => {
            log::error!(
                "Cannot create directory `{}`: {}",
                path.to_str().expect("Path to be created must be valid"),
                e
            );
            Err(ErrorCode::MountError(2))
        }
    }
}

/// Remove an empty directory since must protect old root while it not be unmounted
fn delete_directory(path: &PathBuf) -> Result<(), ErrorCode> {
    match remove_dir(path) {
        Ok(_) => Ok(()),
        Err(e) => {
            log::error!(
                "Unable to delete empty directory `{}`: {}",
                path.to_str().expect("Path to be deleted must be valid"),
                e
            );
            Err(ErrorCode::MountError(3))
        }
    }
}

/// Generate a n-char String
fn random_string(n: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();

    let result: String = (0..n)
        .map(|_| {
            let i = rng.gen_range(0..CHARSET.len());
            CHARSET[i] as char
        })
        .collect();
    result
}
