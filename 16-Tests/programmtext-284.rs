use beispiel;

#[test]
fn is_commutative() {
	assert_eq!(beispiel::add(5, 10), beispiel::add(10, 5));
}