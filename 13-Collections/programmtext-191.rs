fn main() {
	let vector = vec![5, 10, 15];

	match vector.get(0) {
		Some(e) => println!("Element an Position 0: {}", e),
		None => println!("Es gibt kein Element an Position 0")
	}
}