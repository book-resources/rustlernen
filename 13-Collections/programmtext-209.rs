use std::collections::LinkedList;

fn main() {
	let mut list = LinkedList::new();

	list.push_front(7);
	list.push_front(10);

	println!("list = {:?}", list);
}