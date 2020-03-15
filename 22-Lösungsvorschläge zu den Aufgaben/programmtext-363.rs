use rand::Rng;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;

#[derive(Debug)]
enum Muenze {
	Kopf,
	Zahl
}

fn main() {
	let mut results = (0, 0);

	println!("Wie oft soll eine Muenze geworfen werden?");
	let num = read();

	for _ in 1..=num {
		match generate() {
			Muenze::Kopf => results.0 += 1,
			Muenze::Zahl => results.1 += 1
		}
	}

	match store(results.0, results.1, num) {
		Ok(_) => println!("Ergebis gespeichert."),
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

fn store(mut k: u32, mut z: u32, mut t: u32) -> Result<(), io::Error> {
	let result = match File::open("results.txt") {
		Ok(mut f) => {
			let mut content = String::new();
			f.read_to_string(&mut content)?;

			let content_vec: Vec<_> = content.split("\n").collect();
			k += content_vec[0].parse::<u32>().unwrap();
			z += content_vec[1].parse::<u32>().unwrap();
			t += content_vec[2].parse::<u32>().unwrap();

			format!("{}\n{}\n{}", k, z, t)
		},
		Err(_) => format!("{}\n{}\n{}", k, z, t)
	};

	fs::write("results.txt", result)?;

	Ok(())
}