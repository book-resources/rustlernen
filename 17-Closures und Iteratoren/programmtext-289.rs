fn main() {
	let square = |val: i32| -> i32 {
		val * val
	};

	println!("5 * 5 = {}", square(5));
}