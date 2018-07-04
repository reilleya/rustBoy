
pub struct Registers {
	pub a: u8,
	pub f: u8,

	pub b: u8,
	pub c: u8,

	pub d: u8,
	pub e: u8,

	pub h: u8,
	pub l: u8,

	pub sp: u16,
	pub pc: u16
}

impl Registers {
	pub fn load_defaults() -> Registers {
		Registers{
			a: 0x01,
			f: 0xB0,
			b: 0x00,
			c: 0x13,
			d: 0x00,
			e: 0xD8,
			h: 0x01,
			l: 0x4D,
			sp: 0xFFFE,
			pc: 0x0100
		}
	}

	pub fn get_af(&self) -> u16 {
		((self.a as u16) << 8) | (self.f as u16)
	}

	pub fn set_af(&mut self, af:u16) {
		self.a = ((af & 0xFF00) >> 8) as u8;
		self.f = (af & 0x00FF) as u8;
	}

	pub fn get_bc(&self) -> u16 {
		((self.b as u16) << 8) | (self.c as u16)
	}

	pub fn set_bc(&mut self, bc:u16) {
		self.b = ((bc & 0xFF00) >> 8) as u8;
		self.c = (bc & 0x00FF) as u8;
	}

	pub fn get_de(&self) -> u16 {
		((self.d as u16) << 8) | (self.e as u16)
	}

	pub fn set_de(&mut self, de:u16) {
		self.d = ((de & 0xFF00) >> 8) as u8;
		self.e = (de & 0x00FF) as u8;
	}

	pub fn get_hl(&self) -> u16 {
		((self.h as u16) << 8) | (self.l as u16)
	}

	pub fn set_hl(&mut self, hl:u16) {
		self.h = ((hl & 0xFF00) >> 8) as u8;
		self.l = (hl & 0x00FF) as u8;
	}
}

mod test {
	#[test]
	fn test_af() {
		let mut testreg = super::Registers::load_defaults();
		testreg.a = 0x45;
		testreg.f = 0x67;
		assert_eq!(testreg.get_af(), 0x4567);
		testreg.set_af(0xF1A4);
		assert_eq!(testreg.get_af(), 0xF1A4);
		assert_eq!(testreg.a, 0xF1);
		assert_eq!(testreg.f, 0xA4);
	}

	#[test]
	fn test_bc() {
		let mut testreg = super::Registers::load_defaults();
		testreg.b = 0xCE;
		testreg.c = 0x12;
		assert_eq!(testreg.get_bc(), 0xCE12);
		testreg.set_bc(0xBEEF);
		assert_eq!(testreg.get_bc(), 0xBEEF);
		assert_eq!(testreg.b, 0xBE);
		assert_eq!(testreg.c, 0xEF);
	}

	#[test]
	fn test_de() {
		let mut testreg = super::Registers::load_defaults();
		testreg.d = 0x4F;
		testreg.e = 0xA3;
		assert_eq!(testreg.get_de(), 0x4FA3);
		testreg.set_de(0xF00F);
		assert_eq!(testreg.get_de(), 0xF00F);
		assert_eq!(testreg.d, 0xF0);
		assert_eq!(testreg.e, 0x0F);
	}

	#[test]
	fn test_hl() {
		let mut testreg = super::Registers::load_defaults();
		testreg.h = 0x8A;
		testreg.l = 0xFE;
		assert_eq!(testreg.get_hl(), 0x8AFE);
		testreg.set_hl(0xDEAD);
		assert_eq!(testreg.get_hl(), 0xDEAD);
		assert_eq!(testreg.h, 0xDE);
		assert_eq!(testreg.l, 0xAD);
	}
}