use rand::Rng;
use std::io;

#[derive(Debug, PartialEq)]
enum Muenze {
	Kopf,
	Zahl
}

fn main() {
	println!("Kopf (k) oder Zahl (z)?");

	match read() == generate() {
		true => println!("Korrekt!"),
		false => println!("Leider falsch!")
	}
}

fn read() -> Muenze {
	let mut user_input = String::new();
	io::stdin()
		.read_line(&mut user_input)
		.expect("Fehler beim Lesen der Eingabe");

	match user_input.trim() {
		"k" => Muenze::Kopf,
		"z" => Muenze::Zahl,
		 _  => panic!("Ungueltige Eingabe.")
	}
}

fn generate() -> Muenze {
	match rand::thread_rng().gen_range(0, 2) {
		0 => Muenze::Kopf,
		_ => Muenze::Zahl
	}
}