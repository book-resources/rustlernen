use std::io;

fn main() {
	println!("Kopf (k) oder Zahl (z)?");

	let mut user_input = String::new();

	io::stdin()
		.read_line(&mut user_input)
		.expect("Fehler beim Lesen der Eingabe");

	println!("Die Eingabe lautet: {}", user_input.trim());
}