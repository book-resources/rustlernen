use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
	fs::write("hallo.txt", "Hallo, Welt!")?;
	
	Ok(())
}