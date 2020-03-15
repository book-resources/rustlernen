struct Book {
	author: String,
	isbn: String,
	price: f32,
	title: String
}

fn main() {
	let buch1 = Book {
		author: String::from("Max Mustermann"),
		isbn: String::from("123-4-5678-9123-4"),
		price: 9.99,
		title: String::from("Mustertitel")
	};
}