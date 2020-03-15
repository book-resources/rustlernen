use std::collections::HashMap;

fn main() {
	let keys = vec![String::from("Max"), String::from("Linda")];
	let values = vec![10, 20];

	let m: HashMap<_, _> = keys.iter().zip(values.iter()).collect();
	println!("map = {:?}", m);
}