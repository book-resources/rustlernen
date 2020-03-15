use std::collections::HashSet;

fn main() {
	let mut set = HashSet::new();

	set.insert(String::from("Auto"));
	set.insert(String::from("Reifen"));
	set.insert(String::from("Auto"));

	println!("set = {:?}", set);
}