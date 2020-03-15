impl Iterator for Points {
	type Item = (u32, u32);

	fn next(&mut self) -> Option<Self::Item> {
		self.x += 1;
		if self.x <= 5 {
			Some((self.x, self.y))
		} else {
			None
		}
	}
}