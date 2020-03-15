fn match_option(opt: Option<u8>) {
	match opt {
		None => println!("None")
		// Fehler, weil kein Fall fuer "Some" existiert
	}
}