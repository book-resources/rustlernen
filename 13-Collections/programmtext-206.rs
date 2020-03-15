use std::collections::HashSet;

fn main() {
	let elements = vec!["Auto", "Reifen", "Auto"];
	let set: HashSet<_> = elements.iter().collect();

	println!("set = {:?}", set);
}