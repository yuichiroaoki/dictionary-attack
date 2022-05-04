use clap::Parser;
use std::fs::File;
// use std::io;
use std::io::prelude::*;
use std::time::Instant;
use terminal_spinners::{SpinnerBuilder, DOTS};
use std::thread;

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
//mod wifi;
fn type_of<T>(_: T) -> String{
    let a = std::any::type_name::<T>();
    return a.to_string();
  }
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

    
    let mut f1  = File::open("sample/sample1.txt").expect("file not found");
    let mut f2  = File::open("sample/sample2.txt").expect("file not found");
    //let mut f = File::open("sample/xato-net-10-million-passwords.txt").expect("file not found");

    let mut contents1 = String::new();
    f1.read_to_string(&mut contents1)
        // ファイルの読み込み中に問題がありました
        .expect("something went wrong reading the file");
    
    let mut contents2 = String::new();
    f2.read_to_string(&mut contents2)
            // ファイルの読み込み中に問題がありました
            .expect("something went wrong reading the file");
    
    
    let v_contents2: Vec<&str> = contents2.split('\n').collect();
    let result2 = result.clone();

    let handle2 = thread::spawn( move || {
        let v_contents1: Vec<&str> = contents1.split('\n').collect();

        for i in 0..v_contents1.len() {
            
            if result == v_contents1[i].to_string() {
                println!("no.{}", i);
                println!("found {}", result);
                break;
            } else {
                continue;
            }
        }
    //     // thread code
     });

    for k in 0..v_contents2.len() {
        //println!("no.{}", k);
        if result2 == v_contents2[k].to_string() {
            println!("no.{}", k);
            println!("found {}", result2);
            break;
        } else {
            continue;
        }
    }
    
    handle2.join().unwrap();
    handle.done();
    let new_now = Instant::now();
    println!("{:?}", new_now.saturating_duration_since(now));
}
// fn main() {
//     let args = Args::parse();
//     lib::number_to_string(36);
//     const MAX_PASSWORD_LENGTH: usize = 15;
//     // wifi::get_wifi_name();
//     // wifi::connect_to_wifi(&args.name, &args.password);

//     println!("Please input password.");
//     let password = args.password;
//     // let mut password = String::new();
//     // io::stdin()
//     //     .read_line(&mut password)
//     //     .expect("Failed to read line");
//     let result = validate::validate_password(&password, MAX_PASSWORD_LENGTH).unwrap();

//     let now = Instant::now();

//     let handle = SpinnerBuilder::new()
//         .spinner(&DOTS)
//         .text("cracking")
//         .start();

    
//     let mut f1  = File::open("sample/xato-net-10-million-passwords-dup.txt").expect("file not found");


//     let mut contents = String::new();
//     f1.read_to_string(&mut contents)
//         // ファイルの読み込み中に問題がありました
//         .expect("something went wrong reading the file");

//     let contents1 : String = contents[0..contents.len()/2].to_string();
//     let contents2: String  = contents[contents.len()/2+1..].to_string();
//     let v_contents2: Vec<&str> = contents2.split('\n').collect();
//     let result2 = result.clone();

//     let handle2 = thread::spawn( move || {
//         let v_contents1: Vec<&str> = contents1.split('\n').collect();

//         for i in 0..v_contents1.len() {
//             //println!("no.{}", i);
//             if result == v_contents1[i].to_string() {
//                 println!("no.{}", i);
//                 println!("found {}", result);
//                 break;
//             } else {
//                 continue;
//             }
//         }
//     //     // thread code
//         });

//     for k in 0..v_contents2.len() {
//         //println!("no.{}", k);
//         if result2 == v_contents2[k].to_string() {
//             println!("no.{}", k);
//             println!("found {}", result2);
//             break;
//         } else {
//             continue;
//         }
//     }
    
//     handle2.join().unwrap();
//     handle.done();
//     let new_now = Instant::now();
//     println!("{:?}", new_now.saturating_duration_since(now));
// }



