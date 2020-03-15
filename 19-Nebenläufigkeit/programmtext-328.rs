use std::thread;

fn main() {
	let v = vec![10, 23, 55, 63];

	let t = thread::spawn(|| {
		println!("v = {:?}", v); // Fehler!
	});

	t.join().unwrap();
}