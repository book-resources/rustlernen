fn main() {
	let s = String::from("Привет мир");
	let slice = &s[0..12];
	println!("slice = {}", slice);
}