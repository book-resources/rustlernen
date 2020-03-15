pub use self::geometrie::Rechteck;
pub use self::geometrie::Quadrat;

pub mod geometrie {
	pub struct Rechteck {
		breite: u32,
		hoehe: u32
	}

	pub struct Quadrat {
		laenge: u32
	}
}