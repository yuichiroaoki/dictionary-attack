
pub fn validate_password(pass: &String, _max_password_length: usize) -> Result<String, ()> {
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
