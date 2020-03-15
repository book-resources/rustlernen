use std::collections::LinkedList;

fn main() {
	let elements = vec![7, 10, 12, 20];

	let mut list1: LinkedList<_> = elements.iter().collect();
	let list2 = list1.split_off(2);

	println!("list1 = {:?}, list2 = {:?}", list1, list2);
}