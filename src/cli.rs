use crate::error::ErrorCode;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "cunrc", about = "A simple container in Rust.")]
pub struct Args {
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    /// Command to execute inside the container
    #[structopt(short, long)]
    pub command: String,

    /// User ID to create inside the container
    #[structopt(short, long)]
    pub uid: u32,

    /// Directory to mount as root of the container
    #[structopt(parse(from_os_str), short = "m", long = "mount")]
    pub mount_dir: PathBuf,
}

// e.g. curnc --debug --command /bin/bash --mount ../ubuntu-fs --uid 0
pub fn parse_args() -> Result<Args, ErrorCode> {
    let args = Args::from_args();

    // setup logging level
    if args.debug {
        setup_log(log::LevelFilter::Debug);
    } else {
        setup_log(log::LevelFilter::Info);
    }

    // validate arguments
    if !args.mount_dir.exists() || !args.mount_dir.is_dir() {
        return Err(ErrorCode::ArgumentInvaild("mount"));
    }

    Ok(args)
}

fn setup_log(level: log::LevelFilter) {
    env_logger::Builder::from_default_env()
        .format_timestamp_secs()
        .filter(None, level)
        .init();
}
