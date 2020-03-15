use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
	let mutex = Arc::new(Mutex::new(10));
	let mut threads = vec![];
	
	for _ in 0..3 {
		let mutex = Arc::clone(&mutex);
		let t = thread::spawn(move || {
			let mut new_val = mutex.lock().unwrap();
			*new_val *= 10;
		});

		threads.push(t);
	}

	for t in threads {
		t.join().unwrap();
	}

	println!("mutex = {}", *mutex.lock().unwrap());
}