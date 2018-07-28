pub struct Display {
	pub ly_coord: u8,
	pub steps: u64
}

impl Display {
	pub fn create() -> Display {
		Display {
			ly_coord: 0,
			steps: 0
		}
	}

	pub fn update(&mut self, steps:u64) {
		self.steps += steps;
		if self.steps > 456 {
			self.steps -= 456;
			self.ly_coord += 1;
			if self.ly_coord == 157 {
				self.ly_coord = 0;
			}
		}
	}

	pub fn get_mem(&self, loc:u16) -> u8 {
		match loc {
			0xFF44 => self.ly_coord,
			_ => {
				println!("Disp read from {:2X} unsupported", loc);
				0
			}
		}
	}

	pub fn set_mem(&mut self, loc:u16, val:u8) {

	}
}