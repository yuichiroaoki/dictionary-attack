use clap::Parser;
use dotenv::dotenv;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;
use terminal_spinners::{SpinnerBuilder, DOTS};

mod files;
mod lib;
mod utils;
mod validate;
mod wifi;

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
        let pass = Arc::new(Mutex::new(result));
        let found = Arc::new(Mutex::new(false));
        let mut handles = vec![];

        for i in 1..args.thread + 1 {
            let pass = Arc::clone(&pass);
            let found = Arc::clone(&found);

            let handle = thread::spawn(move || {
                let mut num_of_attempt = 0;
                if let Ok(mut lines) =
                    files::read_lines("sample/xato-net-10-million-passwords-dup.txt")
                {
                    if i > 1 {
                        for _ in 1..i {
                            lines.next();
                        }
                    }

                    println!("thread started: {}", i);
                    for line in lines.step_by(args.thread.into()) {
                        let word = pass.lock().unwrap();
                        let mut found_key = found.lock().unwrap();
                        if *found_key == true {
                            println!("\n break, number of attempts: {}", num_of_attempt);
                            break;
                        }
                        if let Ok(ip) = line {
                            if *word == ip {
                                println!("\nfound {}", *word);
                                println!("\n break, number of attempts: {}", num_of_attempt);
                                *found_key = true;
                                break;
                            } else {
                                num_of_attempt += 1;
                                continue;
                            }
                        }
                    }
                    println!("\n not found. thread: {}", i);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
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
                let mut num_of_attempt = 0;

                for i in 0..max_count {
                    let word = pass.lock().unwrap();
                    let mut found_key = found.lock().unwrap();

                    let idx = (i * args.thread as i64) + (m as i64);
                    if *found_key == true {
                        println!("\n break no.{}", idx);
                        println!("num_of_attempt: {}", num_of_attempt);
                        break;
                    }
                    if *word == lib::number_to_string(idx) {
                        println!("\nno.{}", idx);
                        println!("found {}", *word);
                        *found_key = true;
                        println!("num_of_attempt: {}", num_of_attempt);
                        break;
                    } else {
                        num_of_attempt += 1;
                        continue;
                    }
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    handle.done();
    let new_now = Instant::now();
    println!("{:?}", new_now.saturating_duration_since(now));
}
