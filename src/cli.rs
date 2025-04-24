use crate::error::ErrorCode;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "cunrc",
    about = "A lightweight container solution enhanced by eBPF."
)]
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

    /// Mount additional directories inside the container
    #[structopt(parse(from_os_str), short = "a", long = "add")]
    pub add_paths: Vec<PathBuf>,

    /// Mount the tool directory inside the container
    #[structopt(parse(from_os_str), short = "t", long = "tool")]
    pub tool_dir: Option<PathBuf>,
}

// e.g. curnc --debug --command /bin/bash --mount ../ubuntu-fs --uid 0
pub fn parse_args() -> Result<Args, ErrorCode> {
    let mut args = Args::from_args();

    // setup logging level
    if args.debug {
        setup_log(log::LevelFilter::Debug);
    } else {
        setup_log(log::LevelFilter::Info);
    }

    // validate arguments
    if args.command.is_empty() {
        return Err(ErrorCode::ArgumentInvaild("command"));
    }
    if !args.mount_dir.exists() || !args.mount_dir.is_dir() {
        return Err(ErrorCode::ArgumentInvaild("mount"));
    }

    // parse `ecurn` command if tool flag is given
    let ecmd = "ecurn";
    let epath = "/curn/";
    if let Some(_) = args.tool_dir {
        if let Some(i) = args.command.find(ecmd) {
            if i == 0 {
                args.command.replace_range(..ecmd.len() + 1, epath);
            }
        }
    }

    Ok(args)
}

fn setup_log(level: log::LevelFilter) {
    env_logger::Builder::from_default_env()
        .format_timestamp_secs()
        .filter(None, level)
        .init();
}
