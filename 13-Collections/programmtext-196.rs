use std::collections::HashMap;

fn main() {
	let mut map = HashMap::new();

	map.insert(String::from("Max"), 10);
	map.insert(String::from("Linda"), 20);

	println!("{:?}", map);
}