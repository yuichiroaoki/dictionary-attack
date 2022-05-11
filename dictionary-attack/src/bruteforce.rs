use std::sync::{Arc, Mutex};
use std::thread;
use crate::lib;

pub fn bruteforce(pwd: String, num_of_thread: u8) {
	let max_count = 100000000;

	let pass = Arc::new(Mutex::new(pwd));
	let found = Arc::new(Mutex::new(false));
	let mut handles = vec![];

	// 8 threads
	for m in 1..num_of_thread + 1 {
		let pass = Arc::clone(&pass);
		let found = Arc::clone(&found);

		let handle = thread::spawn(move || {
			let mut num_of_attempt = 0;

			for i in 0..max_count {
				let word = pass.lock().unwrap();
				let mut found_key = found.lock().unwrap();

				let idx = (i * num_of_thread as i64) + (m as i64);
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
