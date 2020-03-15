fn main() {
	let a = String::from("Hallo, Welt!");
	let length = get_str_length(&a);

	println!("length of '{}' = {}", a, length);
}

fn get_str_length(some_str: &String) -> usize {
	some_str.len()
}