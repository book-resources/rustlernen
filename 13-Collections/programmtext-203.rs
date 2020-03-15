use std::collections::HashMap;

fn main() {
	let mut map = HashMap::new();
	
	map.insert(String::from("Max"), 10);

	map.entry(String::from("Max")).or_insert(20);
	map.entry(String::from("Linda")).or_insert(20);

	println!("{:?}", map);
}