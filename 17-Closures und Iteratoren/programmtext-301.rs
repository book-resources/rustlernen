fn main() {
	let array = [1, 3, 5, 7, 9];
	let map = array.iter().map(|e| 2 * e);
	
	println!("{:?}.", map);
}