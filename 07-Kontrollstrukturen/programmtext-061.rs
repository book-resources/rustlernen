fn main() {
	let mut counter = 0;
	let loop_value = loop {
		if counter == 3 {
			break counter;
		}
		else {
			println!("Hallo, Welt!");
		}
		counter = counter + 1;
	};
	println!("loop_value = {}", loop_value);
}