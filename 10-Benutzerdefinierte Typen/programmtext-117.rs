fn create_book(author: String, isbn: String, price: f32, title: String) -> Book {
	Book {
		author,
		isbn,
		price,
		title
	}
}