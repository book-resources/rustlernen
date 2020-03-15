fn main() {
	let array = [1, 3, 5, 7, 9];
	let mut array_iter = array.iter();

	println!("{:?}", array_iter.next());
	println!("{:?}", array_iter.next());
	println!("{:?}", array_iter.next());
}