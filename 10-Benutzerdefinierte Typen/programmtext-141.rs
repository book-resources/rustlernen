fn main() {
	let mut counter = 1;

	while let Some(e) = some_function(counter) {
		println!("e = {}", e);
		counter += 1;
	}
}

fn some_function(a: i32) -> Option<i32> {
	if a <= 5 {
		Some(a)
	} else {
		None
	}
}