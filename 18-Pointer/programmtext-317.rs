fn main() {
	let a = 15;
	let b = &a;

	assert_eq!(a, *b);
}