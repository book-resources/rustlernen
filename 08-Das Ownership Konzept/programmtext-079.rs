fn main() {
	let s1 = String::from("Hallo, Welt!");
	let s2 = some_function(s1); // s1 wird nach some_function bewegt
	println!("s2 = {}", s2); // s1 ist nun nicht mehr gueltig, s2 aber schon
}

fn some_function(some_str: String) -> String { // some_str wird in some_function bewegt
	println!("some_str = {}", some_str);
	
	some_str // some_str wird in die aufrufende Funktion bewegt
}