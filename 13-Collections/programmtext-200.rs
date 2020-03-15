use std::collections::HashMap;

fn main() {
	let mut map = HashMap::new();
	
	map.insert(String::from("Max"), 10);
	map.insert(String::from("Linda"), 20);

	let key = String::from("Max");
	println!("{:?}", map.get(&key));

	let key2 = String::from("Peter");
	println!("{:?}", map.get(&key2));
}