use std::io;

fn validate_password(pass: &String, _max_password_length: usize) -> Result<String, ()> {
    let result: bool = !pass.trim().is_empty() && pass.trim().len() < _max_password_length+1;

    println!("{}", pass.len());

    if result {
        let new_pass = pass.clone();
        Ok(new_pass)
    } else {
        Err({
            println!("password should be 1 ~ {} words.", _max_password_length)
        })
    }
}

fn main() {
    const MAX_PASSWORD_LENGTH: usize = 4;

    println!("Please input password.");
    let mut password = String::new();

    io::stdin()
        .read_line(&mut password)
        .expect("Failed to read line");

    let result = validate_password(&password, MAX_PASSWORD_LENGTH).unwrap();

    println!("password: {}", result);
}
