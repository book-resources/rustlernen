fn main() {
	let a = 15;
	let b = Box::new(a);

	assert_eq!(a, *b);
}