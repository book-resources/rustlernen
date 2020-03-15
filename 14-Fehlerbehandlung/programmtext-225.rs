use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
	let a = read_file("a.txt");
	println!("a = {:?}", a);
}

fn read_file(file_name: &str) -> Result<String, io::Error> {
	let f = File::open(file_name);

	let mut f = match f {
		Ok(file) => file,
		Err(err) => return Err(err)
	};

	let mut s = String::new();

	match f.read_to_string(&mut s) {
		Ok(_) => Ok(s),
		Err(err) => Err(err)
	}
}