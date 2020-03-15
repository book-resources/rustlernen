use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
	let a = read_file("a.txt");
	println!("a = {:?}", a);
}

fn read_file(file_name: &str) -> Result<String, io::Error> {
	let mut f = File::open(file_name)?;

	let mut s = String::new();

	f.read_to_string(&mut s)?;

	Ok(s)
}