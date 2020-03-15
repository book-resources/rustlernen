use std::sync::mpsc;
use std::thread;

fn main() {
	let (tx, rx) = mpsc::channel();

	thread::spawn(move || {
		let s = String::from("Hallo, Welt!");
		tx.send(s).unwrap();
	});

	let msg = rx.recv().unwrap();
	println!("msg = {}", msg);
}