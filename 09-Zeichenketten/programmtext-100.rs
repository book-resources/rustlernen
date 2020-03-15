fn main() {
	let s1 = String::from("Hallo");
	let s2 = String::from(", Welt!");

	println!("s1 + s2 = {}", s1 + &s2);
}