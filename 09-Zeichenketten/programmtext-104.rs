fn main() {
	let s = String::from("Привет мир");
	let slice = &s[0..1];
	println!("slice = {}", slice);
}