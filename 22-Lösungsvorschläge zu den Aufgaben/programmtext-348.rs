fn main() {
	let s = String::from("Hallo");
	let reversed: String = s.chars().rev().collect();
	println!("Hallo <-> {}", reversed);
}