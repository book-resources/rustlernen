fn main() {
	let mut s = String::from("Hallo");
	let all: &str = &s[..];

	s.push_str(", Welt!"); // Fehler

	println!("s = {}", s);
}