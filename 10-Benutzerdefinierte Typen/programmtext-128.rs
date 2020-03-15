enum Tag {
	Montag,
	Dienstag,
	Mittwoch,
	Donnerstag,
	Freitag,
	Samstag,
	Sonntag
}

struct Tagebuch {
	datum: String,
	tag: Tag,
	text: String
}

fn main() {
	let eintrag = Tagebuch {
		datum: String::from("09.10.2019"),
		tag: Tag::Mittwoch,
		text: String::from("Heute war ein toller Tag!")
	};
}