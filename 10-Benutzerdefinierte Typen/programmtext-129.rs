enum Tagebuch {
	Montag(String, String),
	Dienstag(String, String),
	Mittwoch(String, String),
	Donnerstag(String, String),
	Freitag(String, String),
	Samstag(String, String),
	Sonntag(String, String)
}

fn main() {
	let eintrag = Tagebuch::Mittwoch(
		String::from("09.10.2019"),
		String::from("Heute war ein toller Tag!")
	);
}