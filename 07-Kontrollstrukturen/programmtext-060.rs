fn main() {
	let mut counter = 0;
	loop {
		if counter == 3 {
			break;
		}
		else {
			println!("Hallo, Welt!");
		}
		counter = counter + 1;
	}
}