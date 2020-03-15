fn main() {
	let a: Option<u8> = Some(10);
	let b: Option<u8> = Some(10);
	
	println!("{:?}", add_options(a, b));
}

fn add_options(a: Option<u8>, b: Option<u8>) -> Option<u8> {
	match a {
		None => None,
		Some(i) => match b {
			None => None,
			Some(j) => Some(i + j)
		}
	}
}