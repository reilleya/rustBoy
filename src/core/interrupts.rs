pub struct Interrupts {
	pub enabled: bool,
}

impl Interrupts {
	pub fn create() -> Interrupts {
		Interrupts {
			enabled: true
		}
	}

	pub fn toggle(&mut self, en:bool) {
		self.enabled = en;
	}
}