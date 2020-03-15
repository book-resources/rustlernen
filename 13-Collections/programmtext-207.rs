use std::collections::HashSet;

fn main() {
	let mut set = HashSet::new();

	set.insert(String::from("Auto"));
	set.insert(String::from("Reifen"));

	if set.contains("Auto") {
		set.remove("Auto");
	}

	println!("set = {:?}", set);
}