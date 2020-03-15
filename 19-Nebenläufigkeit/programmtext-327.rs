use std::thread;

fn main() {
	let t = thread::spawn(|| {
		for i in 1..10 {
			println!("Iteration {} in thread", i);
		}
	});

	for i in 1..4 {
		println!("Iteration {} in main", i);
	}

	t.join().unwrap();

	println!("Ende von main");
}