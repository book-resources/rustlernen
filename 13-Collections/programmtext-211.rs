use std::collections::LinkedList;

fn main() {
	let elements = vec![7, 10, 12, 20];
	let list: LinkedList<_> = elements.iter().collect();

	println!("list = {:?}", list);
}