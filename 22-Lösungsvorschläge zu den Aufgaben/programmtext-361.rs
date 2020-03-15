use rand::Rng;
use std::collections::HashMap;
use std::io;

#[derive(Debug)]
enum Muenze {
	Kopf,
	Zahl
}

fn main() {
	println!("Wie oft soll eine Muenze geworfen werden?");

	let num = read();
	let mut results: HashMap<String, u32> = HashMap::new();

	for _ in 1..=num {
		store(&mut results, generate());
	}

	println!("Anzahl Kopf: {}", results.get("Kopf").unwrap());
	println!("Anzahl Zahl: {}", results.get("Zahl").unwrap());
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

fn store(map: &mut HashMap<String, u32>, res: Muenze) {
	let entry = map.entry(format!("{:?}", res)).or_insert(0);
	*entry += 1;
}