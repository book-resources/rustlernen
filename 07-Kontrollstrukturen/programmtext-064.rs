fn main() {
	let array = [1, 3, 5, 7, 9];
	let mut index = 0;
	while index < array.len() {
		println!("value: {}", array[index]);
		index = index + 1;
	}
}