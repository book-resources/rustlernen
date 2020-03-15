use std::collections::LinkedList;

fn main() {
	let elements = vec![7, 10, 12, 20];
	let mut list1: LinkedList<_> = elements.iter().collect();
	let mut list2: LinkedList<_> = elements.iter().collect();
	
	list1.append(&mut list2);

	println!("list1 = {:?}", list1);
}