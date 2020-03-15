struct Book {
	author: String,
	isbn: String,
	price: f32,
	title: String
}

fn main() {
	let mut buch1 = Book {
		author: String::from("Max Mustermann"),
		isbn: String::from("123-4-5678-9123-4"),
		price: 9.99,
		title: String::from("Mustertitel")
	};

	println!("Preis vorher: {}", buch1.price);
	buch1.price = 8.99;
	println!("Preis nachher: {}", buch1.price);
}