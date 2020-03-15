fn main() {
	let array = [1, 3, 5, 7, 9];
	let f: Vec<_> = array.iter().filter(|&e| *e > 3).collect();
	
	println!("{:?}", f);
}