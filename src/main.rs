#[macro_use]
extern crate scan_fmt;

mod capabilities;
mod cgroup;
mod child;
mod cli;
mod config;
mod container;
mod ebpf;
mod error;
mod hosthname;
mod ipc;
mod mount;
mod syscall;
mod user_namespace;

use error::exit_with_retcode;

fn main() {
    match cli::parse_args() {
        Ok(args) => {
            log::info!("{:?}", args);
            exit_with_retcode(container::start(args));
        }
        Err(e) => {
            log::error!("Error while parsing arguments:\n\t{}", e);
            exit_with_retcode(Err(e));
        }
    }
}
