use std::collections::HashMap;

fn main() {
	let key = String::from("Max");
	let value = 10;

	let mut map = HashMap::new();
	map.insert(key, value); // key ist nun nicht mehr gueltig
}