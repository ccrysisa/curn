use error::exit_with_retcode;

mod cli;
mod error;

fn main() {
    match cli::parse_args() {
        Ok(args) => {
            log::info!("{:?}", args);
            exit_with_retcode(Ok(()));
        }
        Err(e) => {
            log::error!("Error while parsing arguments:\n\t{}", e);
            exit_with_retcode(Err(e));
        }
    }
}
