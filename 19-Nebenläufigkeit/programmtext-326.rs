use std::thread;

fn main() {
	thread::spawn(|| {
		for i in 1..10 {
			println!("Iteration {} in thread", i);
		}
	});

	for i in 1..4 {
		println!("Iteration {} in main", i);
	}
}