use std::env;

pub fn get_env(key: &str) -> String {
    match env::var(key) {
        Ok(val) => val,
        Err(e) => panic!("couldn't interpret {}: {}", key, e),
    }
}

pub fn round_float(num: f64, scale: u32) -> f64 {
    let base = u64::pow(10, scale) as f64;
    (num * base).round() / base
}