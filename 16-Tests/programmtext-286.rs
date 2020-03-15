use beispiel;

mod startup;

#[test]
fn is_commutative() {
	startup::initialization();
	assert_eq!(beispiel::add(5, 10), beispiel::add(10, 5));
}