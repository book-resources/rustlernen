fn main() {
	let a = String::from("Hallo, Welt!");
	let (b, length) = get_str_length(a);

	println!("length of '{}' = {}", b, length);
}

fn get_str_length(some_str: String) -> (String, usize) {
	let length = some_str.len();
	(some_str, length)
}