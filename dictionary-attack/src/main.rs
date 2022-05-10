use clap::Parser;
use dotenv::dotenv;
use std::sync::mpsc;
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

    /// password for wifi
    #[clap(long)]
    password: String,
}

mod files;
mod lib;
mod validate;
mod wifi;

fn main() {
    dotenv().ok();

    let args = Args::parse();
    lib::number_to_string(36);
    const MAX_PASSWORD_LENGTH: usize = 10;
    // wifi::get_wifi_name();
    // wifi::connect_to_wifi(&args.name, &args.password);

    // let (tx, rx) = mpsc::channel();
    let password = args.password;
    let result = validate::validate_password(&password, MAX_PASSWORD_LENGTH).unwrap();

    let now = Instant::now();

    let handle = SpinnerBuilder::new()
        .spinner(&DOTS)
        .text("cracking")
        .start();

    let password = Arc::new(Mutex::new(result));
    let found = Arc::new(Mutex::new(false));
    if args.dict {
        // let mut handles = vec![];

        // for i in 0..8 {
        //     if let Ok(lines) = files::read_lines("sample/xato-net-10-million-passwords-dup.txt") {
        //         let handle = thread::spawn(move || {
        //             for line in lines.step_by(i * 8) {
        //                 if let Ok(ip) = line {
        //                     if result == ip {
        //                         println!("\nfound {}", result);
        //                         tx.send(result).unwrap();
        //                         break;
        //                     } else {
        //                         continue;
        //                     }
        //                 }
        //             }
        //         });
        //         handle.join().unwrap();
        //     }
        // }
    } else {
        let mut handles = vec![];
        let max_count = 100000000;

        // 8 threads
        for m in 1..9 {
            let password = Arc::clone(&password);
            let found = Arc::clone(&found);
            let t_handle = thread::spawn(move || {
                for i in 0..max_count {
                    let word = password.lock().unwrap();
                    let mut found_key = found.lock().unwrap();
                    if *found_key {
                        break;
                    }
                    if *word == lib::number_to_string(i * m) {
                        println!("\nno.{}", i * m);
                        println!("found {}", *word);
                        *found_key = true;
                        break;
                    } else {
                        continue;
                    }
                }
            });
            handles.push(t_handle)
        }

        for hd in handles {
            hd.join().unwrap();
        }
    }
    // for received in rx {
    //     println!("Got: {}", received);
    // }

    handle.done();
    let new_now = Instant::now();
    println!("{:?}", new_now.saturating_duration_since(now));
}
