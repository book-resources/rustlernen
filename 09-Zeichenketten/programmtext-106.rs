fn main() {
	let s = String::from("Hallo, Welt!");
	
	for b in s.bytes() {
		print!("{} ", b);
	}
}