extern crate flexi_logger;
use log::{debug,info,warn,error};
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    flexi_logger::Logger::try_with_str("debug").unwrap()
        .log_to_stderr()
        .format(flexi_logger::colored_detailed_format)
        .start().unwrap();

    info!("BDPod starting up.");

    info!("The end.");
    Ok(())
}