use super::rom::ROM;

pub struct Memory {
	pub rom: ROM
}

impl Memory {
	pub fn create_memory() -> Memory {
		return Memory{
			rom: ROM::create_rom()
		}
	}

	pub fn get_mem(&self, loc:u16) -> u8 {
		self.rom.get_mem(loc)
	}

	pub fn set_mem(&mut self, loc:u16, val:u8) {
		println!("Wrote {:2X} to {:2X}", val, loc);
	}
}