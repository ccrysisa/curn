use crate::{cli::Args, config::ContainerOpts, error::ErrorCode};
use nix::sys::utsname::uname;

const MINIMAL_KERNEL_VERSION: f64 = 5.4; // kernel version of Ubuntu 20.04 LTS

pub struct Container {
    config: ContainerOpts,
}

impl Container {
    pub fn new(args: Args) -> Result<Self, ErrorCode> {
        let config = ContainerOpts::new(args.command, args.uid, args.mount_dir)?;
        Ok(Self { config })
    }

    pub fn create(&mut self) -> Result<(), ErrorCode> {
        log::debug!("Creation finished");
        Ok(())
    }

    pub fn clean_exit(&mut self) -> Result<(), ErrorCode> {
        log::debug!("Cleaning container");
        Ok(())
    }
}

fn check_linux_version() -> Result<(), ErrorCode> {
    let host = uname().expect("Cannot get uname of host");
    let release = host.release().to_str().expect("Release must be valid");
    log::debug!("Linux release: {}", release);

    if let Ok(version) = scan_fmt!(release, "{f}.{}", f64) {
        if version < MINIMAL_KERNEL_VERSION {
            return Err(ErrorCode::NotSupported(0));
        }
    } else {
        return Err(ErrorCode::ContainerError(0));
    }

    if host.machine() != "x86_64" {
        return Err(ErrorCode::NotSupported(1));
    }

    Ok(())
}

pub fn start(args: Args) -> Result<(), ErrorCode> {
    check_linux_version()?;

    let mut container = Container::new(args)?;

    if let Err(e) = container.create() {
        log::error!("Error while creating container: {:?}", e);
        return Err(e);
    }

    log::debug!("Execution finished, now cleaning and exit");
    container.clean_exit()
}
