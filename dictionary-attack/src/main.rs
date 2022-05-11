use clap::Parser;
use dotenv::dotenv;
use std::sync::{Arc, Mutex};
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

mod files;
mod lib;
mod utils;
mod validate;
mod wifi;

fn main() {
    dotenv().ok();

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

    if args.wifi {
        let result = wifi::connect_to_wifi();
        println!("{:?}", result);
    }

    let password = Arc::new(Mutex::new(result));
    let found = Arc::new(Mutex::new(false));
    if args.dict {
        let mut handles = vec![];
        let pass = Arc::new(Mutex::new(result));
        let found = Arc::new(Mutex::new(false));

        for i in 0..8 {
            let pass = Arc::clone(&pass);
            let found = Arc::clone(&found);

            if let Ok(lines) = files::read_lines("sample/xato-net-10-million-passwords-dup.txt") {
                let handle = thread::spawn(move || {
                    let word = pass.lock().unwrap();
                    let mut found_key = found.lock().unwrap();

                    for line in lines.step_by(i * 8) {
                        if *found_key == true {
                            println!("\n break ");
                            break;
                        }
                        if let Ok(ip) = line {
                            if *word == ip {
                                println!("\nfound {}", *word);
                                *found_key = true;
                                break;
                            } else {
                                continue;
                            }
                        }
                    }
                });
                handles.push(handle);
            }
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("found flag: {}", *found.lock().unwrap());
    } else {
        let max_count = 100000000;

        let pass = Arc::new(Mutex::new(result));
        let found = Arc::new(Mutex::new(false));
        let mut handles = vec![];

        // 8 threads
        for m in 1..args.thread + 1 {
            let pass = Arc::clone(&pass);
            let found = Arc::clone(&found);

            let handle = thread::spawn(move || {
                for i in 0..max_count {
                    let word = pass.lock().unwrap();
                    let mut found_key = found.lock().unwrap();

                    let idx = (i * args.thread as i64) + (m as i64);
                    if *found_key == true {
                        println!("\n break no.{}", idx);
                        break;
                    }
                    if *word == lib::number_to_string(idx) {
                        println!("\nno.{}", idx);
                        println!("found {}", *word);
                        *found_key = true;
                        break;
                    } else {
                        continue;
                    }
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        println!("found flag: {}", *found.lock().unwrap());
    }

    handle.done();
    let new_now = Instant::now();
    println!("{:?}", new_now.saturating_duration_since(now));
}
