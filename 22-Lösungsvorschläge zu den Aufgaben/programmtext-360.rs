use rand::Rng;
use std::collections::HashMap;

#[derive(Debug)]
enum Muenze {
	Kopf,
	Zahl
}

fn main() {
	let mut results: HashMap<String, u32> = HashMap::new();

	for _ in 1..=50 {
		store(&mut results, generate());
	}

	println!("Anzahl Kopf: {}", results.get("Kopf").unwrap());
	println!("Anzahl Zahl: {}", results.get("Zahl").unwrap());
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