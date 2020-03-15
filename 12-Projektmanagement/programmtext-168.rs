use rand::Rng;

fn main() {
	let r = rand::thread_rng().gen_range(1, 11);
	println!("r = {}", r);
}