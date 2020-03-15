fn main() {
	let a = 15;
	some_function(a);
	// a ist auch nach der Funktion gueltig, weil nur eine Kopie uebergeben wurde
	println!("a = {}", a);
}

fn some_function(some_int: i32) {
	println!("some_int = {}", some_int);
}