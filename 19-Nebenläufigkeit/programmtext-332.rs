use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
	let (tx, rx) = mpsc::channel();

	thread::spawn(move || {
		let values = vec![1, 2, 3, 4, 5];
		for val in values {
			tx.send(val).unwrap();
			thread::sleep(Duration::from_secs(1));
		}
	});

	for received in rx {
		println!("{}", received);
	}
}