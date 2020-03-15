struct NewBox<T>(T);

fn main() {
	let a = 5;
	let b = NewBox(a);

	assert_eq!(a, *b); // Fehler!
}