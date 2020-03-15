fn main() {
	let mut s = String::from("Hallo");

	{
		let s1 = &mut s;
	} // s1 ist ab hier nicht mehr gueltig

	let s2 = &mut s;
}