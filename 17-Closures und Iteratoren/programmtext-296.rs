fn main() {
	let s = String::from("Hallo");
	let compare_str = move |x| s == x;

	println!("s == Hallo: {}", compare_str(String::from("Hallo")));
	println!("s = {}", s); // Fehler!
}