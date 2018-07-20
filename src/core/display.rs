pub struct Display {
	pub ly_coord: u8,
}

impl Display {
	pub fn create() -> Display {
		Display {
			ly_coord: 0
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