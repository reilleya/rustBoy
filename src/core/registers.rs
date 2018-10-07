pub enum RegisterName {
	a,
	f,
	b,
	c,
	d,
	e,
	h,
	l
}

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

	pub fn set_flags(&mut self, z:bool, n:bool, h:bool, c:bool) {
		self.f = ((z as u8) << 7) + ((n as u8) << 6)
			+ ((h as u8) << 5) + ((c as u8) << 4);
	}

	pub fn get_z(&self) -> bool {
		(self.f & 0b10000000) == 0b10000000
	}

	pub fn set_z(&mut self, z:bool) {
		self.f = (self.f & 0b01111111) | ((z as u8) << 7);
	}

	pub fn get_n(&self) -> bool {
		(self.f & 0b01000000) == 0b01000000
	}

	pub fn set_n(&mut self, n:bool) {
		self.f = (self.f & 0b10111111) | ((n as u8) << 6);
	}

	pub fn get_h(&self) -> bool {
		(self.f & 0b00100000) == 0b00100000
	}

	pub fn set_h(&mut self, h:bool) {
		self.f = (self.f & 0b11011111) | ((h as u8) << 5);
	}

	pub fn get_c(&self) -> bool {
		(self.f & 0b00010000) == 0b00010000
	}

	pub fn set_c(&mut self, c:bool) {
		self.f = (self.f & 0b11101111) | ((c as u8) << 4);
	}

	pub fn disp_state(&self) {
		println!("AF: {:4X}", self.get_af());
		println!("BC: {:4X}", self.get_bc());
		println!("DE: {:4X}", self.get_de());
		println!("HL: {:4X}", self.get_hl());
		println!("PC: {:4X}", self.pc);
		println!("SP: {:4X}", self.sp);
		println!("Z:{:?} N:{:?} H:{:?} C:{:?}", self.get_z(), self.get_n(), self.get_h(), self.get_c());
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

	#[test]
	fn test_z() {
		let mut testreg = super::Registers::load_defaults();
		testreg.set_z(true);
		assert_eq!(testreg.get_z(), true);
		testreg.set_z(false);
		assert_eq!(testreg.get_z(), false);
	}

	#[test]
	fn test_n() {
		let mut testreg = super::Registers::load_defaults();
		testreg.set_n(true);
		assert_eq!(testreg.get_n(), true);
		testreg.set_n(false);
		assert_eq!(testreg.get_n(), false);
	}

	#[test]
	fn test_h() {
		let mut testreg = super::Registers::load_defaults();
		testreg.set_h(true);
		assert_eq!(testreg.get_h(), true);
		testreg.set_h(false);
		assert_eq!(testreg.get_h(), false);
	}

	#[test]
	fn test_c() {
		let mut testreg = super::Registers::load_defaults();
		testreg.set_c(true);
		assert_eq!(testreg.get_c(), true);
		testreg.set_c(false);
		assert_eq!(testreg.get_c(), false);
	}

	#[test]
	fn test_set_flags() {
		let mut testreg = super::Registers::load_defaults();
		testreg.set_flags(true, false, false, true);
		assert_eq!(testreg.get_z(), true);
		assert_eq!(testreg.get_n(), false);
		assert_eq!(testreg.get_h(), false);
		assert_eq!(testreg.get_c(), true);

		testreg.set_flags(true, true, true, true);
		assert_eq!(testreg.get_z(), true);
		assert_eq!(testreg.get_n(), true);
		assert_eq!(testreg.get_h(), true);
		assert_eq!(testreg.get_c(), true);

		testreg.set_flags(false, false, false, false);
		assert_eq!(testreg.get_z(), false);
		assert_eq!(testreg.get_n(), false);
		assert_eq!(testreg.get_h(), false);
		assert_eq!(testreg.get_c(), false);
	}
}