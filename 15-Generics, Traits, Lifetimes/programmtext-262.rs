fn main() {
	let a;

	{
		let b = 15;
		a = &b;
	}

	println!("a = {}", a);
}