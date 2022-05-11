use clap::Parser;
use dotenv::dotenv;
use std::time::Instant;
use terminal_spinners::{SpinnerBuilder, DOTS};

mod files;
mod lib;
mod utils;
mod validate;
mod wifi;
mod bruteforce;
mod dictionary;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// dictionary attack with wordlists
    #[clap(short)]
    dict: bool,
    /// performance test mode
    #[clap(short)]
    performance: bool,

    /// connecting WiFi mode
    #[clap(long)]
    wifi: bool,

    /// password for wifi
    #[clap(long)]
    password: String,

    /// number of threads
    #[clap(long)]
    thread: u8,
}

fn main() {
    dotenv().ok();

    let args = Args::parse();
    lib::number_to_string(36);
    const MAX_PASSWORD_LENGTH: usize = 10;

    let password = args.password;
    let result = validate::validate_password(&password, MAX_PASSWORD_LENGTH).unwrap();

    let now = Instant::now();

    let handle = SpinnerBuilder::new()
        .spinner(&DOTS)
        .text("cracking")
        .start();

    if args.wifi {
        let result = wifi::connect_to_wifi();
        println!("{:?}", result);
    }

    if args.dict {
        dictionary::dictionary_attack(result, args.thread);
    } else {
        bruteforce::bruteforce(result, args.thread);
    }

    handle.done();
    let new_now = Instant::now();
    println!("{:?}", new_now.saturating_duration_since(now));
}
