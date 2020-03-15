use std::collections::HashMap;

fn main() {
	let mut map = HashMap::new();
	
	map.insert(String::from("Max"), 10);
	map.insert(String::from("Linda"), 20);

	for (key, value) in &map {
		println!("{} hat {} Punkte.", key, value);
	}
}