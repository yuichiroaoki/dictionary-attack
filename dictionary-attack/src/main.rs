use clap::Parser;
use std::fs::File;
// use std::io;
use std::io::prelude::*;
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

mod lib;
mod validate;
mod wifi;

fn main() {
    let args = Args::parse();
    lib::number_to_string(36);
    const MAX_PASSWORD_LENGTH: usize = 10;
    // wifi::get_wifi_name();
    // wifi::connect_to_wifi(&args.name, &args.password);

    println!("Please input password.");
    let password = args.password;
    // let mut password = String::new();
    // io::stdin()
    //     .read_line(&mut password)
    //     .expect("Failed to read line");
    let result = validate::validate_password(&password, MAX_PASSWORD_LENGTH).unwrap();

    let now = Instant::now();

    let handle = SpinnerBuilder::new()
        .spinner(&DOTS)
        .text("cracking")
        .start();

    if args.dict {
        let mut f = File::open("sample/xato-net-10-million-passwords-dup.txt").expect("file not found");
        //let mut f = File::open("sample/xato-net-10-million-passwords.txt").expect("file not found");

        let mut contents = String::new();
        f.read_to_string(&mut contents)
            // ファイルの読み込み中に問題がありました
            .expect("something went wrong reading the file");

        let v_contents: Vec<&str> = contents.split('\n').collect();
        for i in 0..v_contents.len() {
            if result == v_contents[i] {
                println!("no.{}", i);
                println!("found {}", result);
                break;
            } else {
                continue;
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
