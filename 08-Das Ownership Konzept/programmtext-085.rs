fn main() {
	let mut s = String::from("Hallo");

	let s1 = &s; // OK
	let s2 = &s; // OK
	let s3 = &mut s; // Fehler!
}