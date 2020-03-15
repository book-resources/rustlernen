#[derive(Debug)]
enum MixedType {
	Integer(i32),
	Text(String)
}

fn main() {
	let vector = vec![
		MixedType::Integer(5),
		MixedType::Integer(10),
		MixedType::Text(String::from("Hallo, Welt!"))
	];

	println!("{:?}", vector);
}