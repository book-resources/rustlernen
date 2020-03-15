use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
	let a = read_file("a.txt");
	println!("a = {:?}", a);
}

fn read_file(file_name: &str) -> Result<String, io::Error> {
	let mut s = String::new();

	File::open(file_name)?.read_to_string(&mut s)?;

	Ok(s)
}