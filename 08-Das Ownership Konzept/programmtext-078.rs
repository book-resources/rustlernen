fn main() {
	let s = some_function(); // Der String wird aus der Funktion nach s bewegt
	println!("s = {}", s); // s ist hier gueltig und kann verwendet werden
}

fn some_function() -> String {
	let new_str = String::from("Hallo, Welt!");
	println!("new_str = {}", new_str);
	
	new_str // new_str wird in die aufrufende Funktion bewegt
}