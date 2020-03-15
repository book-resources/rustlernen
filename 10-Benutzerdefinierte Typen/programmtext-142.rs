fn main() {
	let var = 1;

	match var {
		1 | 5 | 10 => println!("1 oder 5 oder 10"),
		_ => println!("Irgendwas anderes")
	}
}