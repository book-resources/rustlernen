use std::thread;

fn main() {
	let v = vec![10, 23, 55, 63];

	let t = thread::spawn(move || {
		println!("v = {:?}", v);
	});

	t.join().unwrap();
}