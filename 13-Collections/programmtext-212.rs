use std::collections::LinkedList;

fn main() {
	let elements = vec![7, 10, 12, 20];
	let mut list: LinkedList<_> = elements.iter().collect();

	let first = list.pop_front();
	println!("{:?} entfernt", first);

	let last = list.pop_back();
	println!("{:?} entfernt", last);

	println!("list = {:?}", list);
}