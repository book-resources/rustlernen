fn main() {
	for number in 1..5 {
		if number == 3 {
			continue;
		}
		println!("number = {}", number);
	}
}