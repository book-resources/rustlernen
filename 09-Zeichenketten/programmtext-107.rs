fn main() {
	let s1 = String::from("Hallo");
	let s2 = String::from(", ");
	let s3 = String::from("Welt");

	let conc = s1 + &s2 + &s3 + "!";
	println!("{}", conc);
}