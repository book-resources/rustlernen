use rand::Rng;

fn main() {
	let r = rand::thread_rng().gen_range(1, 100);
	println!("r = {}", r);
}