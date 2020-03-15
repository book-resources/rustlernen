use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
	let (tx, rx) = mpsc::channel();

	let tx1 = mpsc::Sender::clone(&tx);
	thread::spawn(move || {
		let values = vec![1, 2, 3, 4];
		for val in values {
			tx1.send(val).unwrap();
			thread::sleep(Duration::from_secs(1));
		}
	});

	thread::spawn(move || {
		let values = vec![5, 6, 7, 8];
		for val in values {
			tx.send(val).unwrap();
			thread::sleep(Duration::from_secs(1));
		}
	});

	for received in rx {
		println!("{}", received);
	}
}