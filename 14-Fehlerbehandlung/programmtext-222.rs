use std::fs::File;

fn main() {
	let f = File::open("a.txt");

	let f = match f {
		Ok(file) => file,
		Err(err) => panic!("Error while opening: {:?}", err)
	};
}