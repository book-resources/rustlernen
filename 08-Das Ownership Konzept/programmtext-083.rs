fn main() {
	let mut s = String::from("Hallo");

	let s1 = &mut s;
	let s2 = &mut s; // Fehler!

	println!("s1 = {}, s2 = {}", s1, s2);
}