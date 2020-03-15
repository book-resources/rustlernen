fn main() {
	let s = String::from("Hallo, Welt!");
	some_function(s); // s wird in some_function bewegt
	println!("s = {}", s); // Fehler! s ist hier nicht mehr gueltig
}

fn some_function(some_str: String) { // hier beginnt der Scope von some_str
	println!("some_str = {}", some_str);
} // hier endet der Scope von some_str