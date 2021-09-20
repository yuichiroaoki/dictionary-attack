use std::io;
use std::time::{Duration, Instant};
use terminal_spinners::{SpinnerBuilder, DOTS};

fn validate_password(pass: &String, _max_password_length: usize) -> Result<String, ()> {
    let result: bool = !pass.trim().is_empty() && pass.trim().len() < _max_password_length + 1;

    println!("{}", pass.len());

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
    const MAX_PASSWORD_LENGTH: usize = 6;

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
    std::thread::sleep(Duration::from_secs(3));

    for i in 1.. {
        if result == String::from_utf8(lib::generate_string(i)).unwrap() {
            println!("no.{}", i);
            println!("found {}", result);
            break;
        } else {
            continue;
        }
    }

    handle.done();
    let new_now = Instant::now();
    println!("{:?}", new_now.saturating_duration_since(now));
}
