struct Book {
	author: String,
	isbn: String,
	price: f32,
	title: String
}

fn main() {
	let buch1 = create_book(
		String::from("Max Mustermann"),
		String::from("123-4-5678-9123-4"),
		9.99,
		String::from("Mustertitel")
	);
}

fn create_book(author: String, isbn: String, price: f32, title: String) -> Book {
	Book {
		author: author,
		isbn: isbn,
		price: price,
		title: title
	}
}