fn main() {
	let mut vector = vec![5, 10, 15];

	for e in &mut vector {
		*e *= 2;
	}

	println!("{:?}", vector);
}