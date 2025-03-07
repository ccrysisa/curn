use crate::error::ErrorCode;
use which::which;

const TOOLS_LIST: [&'static str; 1] = ["sqlbrowser"];

fn check_tools() -> Result<(), ErrorCode> {
    for tool in TOOLS_LIST {
        match which(tool) {
            Ok(path) => {
                log::debug!("Find tool at {:?}", path);
            }
            Err(_) => {
                log::debug!("Not find tool: {}", tool);
            }
        }
    }
    Ok(())
}

pub fn set_tools() -> Result<(), ErrorCode> {
    check_tools()
    // todo!()
}
