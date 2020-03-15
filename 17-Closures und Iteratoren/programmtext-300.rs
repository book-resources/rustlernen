fn main() {
	let array = [1, 3, 5, 7, 9];
	let sum: i32 = array.iter().sum();
	
	println!("Die Summe ist {}.", sum);
}