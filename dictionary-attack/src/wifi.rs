use std::error::Error;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
// use clap::lazy_static::lazy_static;

// use regex::Regex;

pub fn get_wifi_name() -> Result<(), Box<dyn Error>> {
	let stdout = Command::new("iwlist")
		.arg("wlo1")
		.arg("scan")
		.stdout(Stdio::piped())
		.spawn()?
		.stdout
		.expect("failed to execute process");

	let reader = BufReader::new(stdout);

	let target_line = reader
		.lines()
		.filter_map(|line| line.ok())
		.filter(|line| line.find("ESSID").is_some())
		.next();
	// .for_each(|line| println!("{}", line));

	println!("{:?}", target_line);

	Ok(())
}

pub fn example_command() {
	let output = Command::new("ls")
		.stdout(Stdio::piped())
		.output()
		.expect("failed to execute process");
	let stdout = String::from_utf8(output.stdout).unwrap();

	println!("{}", stdout);
}

pub fn connect_to_wifi(name: &str, password: &str) {
	let output = Command::new("nmcli")
		.arg("d")
		.arg("wifi")
		.arg("connect")
		.arg(name)
		.arg("password")
		.arg(password)
		.stdout(Stdio::piped())
		.output()
		.expect("failed to execute process");
	let stdout = String::from_utf8(output.stdout).unwrap();

	println!("{}", stdout);
}
