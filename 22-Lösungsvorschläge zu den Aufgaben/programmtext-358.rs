use rand::Rng;

#[derive(Debug)]
enum Muenze {
	Kopf,
	Zahl
}

fn main() {
	let r = rand::thread_rng().gen_range(0, 2);
	
	match r {
		0 => println!("{:?}", Muenze::Kopf),
		_ => println!("{:?}", Muenze::Zahl)
	}
}