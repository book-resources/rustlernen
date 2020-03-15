use std::fs::File;
use std::io::ErrorKind;

fn main() {
	let f = File::open("a.txt");

	let f = match f {
		Ok(file) => file,
		Err(err) => match err.kind() {
			ErrorKind::NotFound => match File::create("a.txt") {
				Ok(created_file) => created_file,
				Err(e) => panic!("Cannot create file: {:?}", e)
			},
			other => panic!("Cannot open file: {:?}", other)
		}
	};
}