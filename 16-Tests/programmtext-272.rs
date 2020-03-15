struct Rechteck {
	breite: u32,
	hoehe: u32
}

impl Rechteck {
	fn is_square(&self) -> bool {
		&self.breite == &self.hoehe
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn rect_is_square() {
		let r = Rechteck {breite: 5, hoehe: 5};
		assert!(r.is_square());
	}
}