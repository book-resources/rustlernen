use rand::Rng;
use std::fs;
use std::io;

#[derive(Debug)]
enum Muenze {
	Kopf,
	Zahl
}

fn main() {
	let mut results = (0, 0);

	println!("Wie oft soll eine Muenze geworfen werden?");

	for _ in 1..=read() {
		match generate() {
			Muenze::Kopf => results.0 += 1,
			Muenze::Zahl => results.1 += 1
		}
	}

	let result = format!("K: {}, Z: {}", results.0, results.1);
	match store(&result[..]) {
		Ok(_) => println!("Ergebnis gespeichert."),
		Err(e) => println!("Es ist ein Fehler aufgetreten: {}", e)
	};
}

fn read() -> u32 {
	let mut user_input = String::new();
	io::stdin()
		.read_line(&mut user_input)
		.expect("Fehler beim Lesen der Eingabe");

	user_input.trim().parse::<u32>().unwrap()
}

fn generate() -> Muenze {
	match rand::thread_rng().gen_range(0, 2) {
		0 => Muenze::Kopf,
		_ => Muenze::Zahl
	}
}

fn store(results: &str) -> Result<(), io::Error> {
	fs::write("result.txt", results)?;

	Ok(())
}