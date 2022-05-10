use wifi_rs::{prelude::*, WiFi};
mod utils;
use dotenv::dotenv;

fn main() -> Result<(), WifiConnectionError> {
    dotenv().ok();

    let config = Some(Config {
        interface: Some("wlo1"),
    });

    let mut wifi = WiFi::new(config);

    match wifi.connect(&utils::get_env("WIFI_SSID"), &utils::get_env("PASS")) {
        Ok(result) => println!(
            "{}",
            if result == true {
                "Connection Successfull."
            } else {
                "Invalid password."
            }
        ),
        Err(err) => println!("The following error occurred: {:?}", err),
    }

    Ok(())
}
