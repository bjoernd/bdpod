use std::env;
extern crate flexi_logger;
use log::{debug,info,warn,error};
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    flexi_logger::Logger::try_with_str("debug").unwrap()
        .log_to_stderr()
        .format(flexi_logger::colored_detailed_format)
        .start().unwrap();

    info!("BDPod starting up.");

    /*
     * Client ID and secret are configured via https://www.podchaser.com/profile/settings/api and need to be provided
     * at application launch. (Actually, those are app-specific, so if we released this, we'd likely hard-code them.)
     */
    let client_id = env::var("CLIENT_ID").expect("Missing CLIENT_ID");
    let client_secret = env::var("CLIENT_SECRET").expect("Missing CLIENT_SECRET");

    debug!("Using client ID:     {}", client_id);
    debug!("Using client secret: {}", client_secret);

    info!("The end.");
    Ok(())
}