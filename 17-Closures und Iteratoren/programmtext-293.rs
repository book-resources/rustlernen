fn main() {
	let mut c = ClosureCache::new(|v| v * v);

	println!("value = {}", c.calc_value(5));
	println!("value = {}", c.calc_value(10));
}