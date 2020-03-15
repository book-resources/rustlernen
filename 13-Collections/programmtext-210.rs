use std::collections::LinkedList;

fn main() {
	let mut list = LinkedList::new();

	list.push_back(7);
	list.push_back(10);

	println!("list = {:?}", list);
}