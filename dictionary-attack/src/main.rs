use std::io;
use std::time::Instant;
use terminal_spinners::{SpinnerBuilder, DOTS};
use std::fs::File;
use std::io::prelude::*;

fn validate_password(pass: &String, _max_password_length: usize) -> Result<String, ()> {
    let result: bool = !pass.trim().is_empty() && pass.trim().len() < _max_password_length + 1;

    if result {
        let new_pass = pass.trim().clone();
        Ok(String::from(new_pass))
    } else {
        Err(println!(
            "password should be 1 ~ {} words.",
            _max_password_length
        ))
    }
}

mod lib;

fn main() {
    const MAX_PASSWORD_LENGTH: usize = 10;

    println!("Please input password.");
    let mut password = String::new();
    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read line");
    // let password_bytes = Vec::from(password);
    let result = validate_password(&password, MAX_PASSWORD_LENGTH).unwrap();
    println!("password: {}", result);
    let now = Instant::now();

    let handle = SpinnerBuilder::new()
        .spinner(&DOTS)
        .text("cracking")
        .start();
    let mut v_contents = Vec::new();

    for k in 1..4 {
        let mut f = File::open(format!("sample/sample{}.txt", k)).expect("file not found");

        let mut contents = String::new();
        f.read_to_string(&mut contents)
            // ファイルの読み込み中に問題がありました
            .expect("something went wrong reading the file");
        let  v_contents_pre: Vec<&str> =  &mut contents.split('\n').collect();
        v_contents.push(v_contents_pre);
    }

    for j in 0..v_contents.len(){
        for i in 0..v_contents[0].len() {
            if result == v_contents[j][i] {
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

