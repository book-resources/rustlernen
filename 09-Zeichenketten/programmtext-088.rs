fn main() {
	let s = String::from("Hallo, Welt!");

	let hallo = &s[0..5];
	let welt = &s[7..11];

	println!("s = {}, {}!", hallo, welt);
}