fn main() {
	let reference = some_function();
}

fn some_function() -> &String {
	let s = String::from("Hallo, Welt!");

	&s // Wir geben eine Referenz auf s zurueck
} // Hier verlaesst s seinen Scope und wird somit ungueltig