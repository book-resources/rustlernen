struct Point<T> {
	x: T,
	y: T
}

fn main() {
	let p1 = Point{ x: 5, y: 5.5 }; // Fehler!
}