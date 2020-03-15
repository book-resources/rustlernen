fn main() {
	let var = 9;

	match var {
		1..=10 => println!("match!"),
		_ => println!("no match!")
	}
}