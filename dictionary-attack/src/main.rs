use clap::Parser;
use std::fs::File;
use std::io::prelude::*;
use std::thread;
use std::time::Instant;
use terminal_spinners::{SpinnerBuilder, DOTS};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// dictionary attack with wordlists
    #[clap(short)]
    dict: bool,
    /// performance test mode
    #[clap(short)]
    performance: bool,

    /// password for wifi
    #[clap(long)]
    password: String,
}

mod files;
mod lib;
mod validate;
mod wifi;

fn main() {
    let args = Args::parse();
    lib::number_to_string(36);
    const MAX_PASSWORD_LENGTH: usize = 10;
    // wifi::get_wifi_name();
    // wifi::connect_to_wifi(&args.name, &args.password);

    let password = args.password;
    let result = validate::validate_password(&password, MAX_PASSWORD_LENGTH).unwrap();

    let now = Instant::now();

    let handle = SpinnerBuilder::new()
        .spinner(&DOTS)
        .text("cracking")
        .start();

    if args.dict {
        let result_cp = result.clone();
        if let Ok(lines) = files::read_lines("sample/sample1.txt") {
            thread::spawn(move || {
                for line in lines {
                    if let Ok(ip) = line {
                        if result == ip {
                            println!("found {}", result);
                            break;
                        } else {
                            continue;
                        }
                    }
                }
            });
        }

        if let Ok(lines) = files::read_lines("sample/sample2.txt") {
            for line in lines {
                if let Ok(ip) = line {
                    if result_cp == ip {
                        println!("found {}", result_cp);
                        break;
                    } else {
                        continue;
                    }
                }
            }
        }
    } else {
        for i in 1..10000000000000 {
            if result == lib::number_to_string(i) {
                println!("no.{}", i);
                println!("found {}", result);
                break;
            } else {
                continue;
            }
        }
    }
    handle.done();
    let new_now = Instant::now();
    println!("{:?}", new_now.saturating_duration_since(now));
}
