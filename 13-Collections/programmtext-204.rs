use std::collections::HashMap;

fn main() {
	let s = String::from("Hallo");
	let mut map = HashMap::new();

	for c in s.chars() {
		let count = map.entry(c).or_insert(0);
		*count += 1;
	}

	println!("{:?}", map);
}