use super::rom::ROM;
use super::display::Display;

use super::timer::Timer;

pub struct Memory {
	pub rom: ROM,
	pub ram: [u8; 0x207F], // 0x2000 bytes for 0xC000-0xE000, 0x7f for 0xFF80-0xFF
	pub disp: Display,
	pub timer: Timer,
}

impl Memory {
	pub fn create_memory() -> Memory {
		return Memory{
			rom: ROM::create_rom(),
			ram: [0; 0x207F],
			disp: Display::create(),
			timer: Timer::create()
		}
	}

	pub fn update(&mut self, steps:u64) {
		self.disp.update(steps);
		self.timer.step(steps);
	}

	pub fn get_mem(&self, loc:u16) -> u8 {
		println!("Read {:2X}", loc);
		match loc {
			0x0000 ..= 0x7FFF => self.rom.get_mem(loc),
			0x8000 ..= 0x9FFF => 0, // VRAM
			0xA000 ..= 0xBFFF => 0, // SWITCH_RAM
			0xC000 ..= 0xDFFF => self.ram[(loc - 0xC000) as usize], // RAM
			0xE000 ..= 0xFDFF => self.ram[(loc - 0xE000) as usize], // RAM echo
			0xFE00 ..= 0xFE9F => 0, // OAM
			0xFEA0 ..= 0xFEFF => 0, // IO
			0xFF00 ..= 0xFF3F => 0, // IO
			0xFF40 ..= 0xFF4B => self.disp.get_mem(loc),
			0xFF4C ..= 0xFF7F => 0, // IO
			0xFF80 ..= 0xFFFE => self.ram[(0x2000 + (loc - 0xFF80)) as usize],// RAM
			0xFFFF => 0,
			_ => {
				// It is terrible that I need this at all
				panic!("Read out of bounds at {:2X}", loc);
				0
			}
		}
	}

	pub fn set_mem(&mut self, loc:u16, val:u8) {
		println!("Wrote {:2X} to {:2X}", val, loc);

		match loc {
			0x0000 ..= 0x7FFF => {
				// CART
			},
			0x8000 ..= 0x9FFF => {
				// VRAM
			},
			0xA000 ..= 0xBFFF => {
				// SWITCH_RAM
			},
			0xC000 ..= 0xDFFF => {
				self.ram[(loc - 0xC000) as usize] = val; // RAM
			},
			0xE000 ..= 0xFDFF => {
				self.ram[(loc - 0xE000) as usize] = val; // RAM echo
			},
			0xFE00 ..= 0xFE9F => {
				// OAM
			},
			0xFEA0 ..= 0xFEFF => {
				// IO
			},
			0xFF00 ..= 0xFF4B => {
				// IO
			},
			0xFF4C ..= 0xFF7F => {
				// IO
			},
			0xFF80 ..= 0xFFFE => {
				self.ram[(0x2000 + (loc - 0xFF80)) as usize] = val; // RAM
			},
			0xFFFF => {
				// INT
			},
			_ => {
				// It is terrible that I need this at all
				panic!("Write out of bounds at {:2X}", loc);
			}
		}
	}
}

mod test {
	#[test]
	fn test_read_reachable() {
		let mut memory = super::Memory::create_memory();
		for loc in 0x0000 ..= 0xFFFF {
			memory.get_mem(loc);
		}
	}

	#[test]
	fn test_write_reachable() {
		let mut memory = super::Memory::create_memory();
		for loc in 0x0000 ..= 0xFFFF {
			memory.set_mem(loc, 0x12);
		}
	}

	#[test]
	fn test_norm_ram() {
		let mut memory = super::Memory::create_memory();
		memory.set_mem(0xC000, 0xF1);
		assert_eq!(memory.get_mem(0xC000), 0xF1);
		memory.set_mem(0xCF43, 0x47);
		assert_eq!(memory.get_mem(0xCF43), 0x47);
		memory.set_mem(0xDFFF, 0xAA);
		assert_eq!(memory.get_mem(0xDFFF), 0xAA);
	}

	#[test]
	fn test_upper_ram() {
		let mut memory = super::Memory::create_memory();
		memory.set_mem(0xFF80, 0x53);
		assert_eq!(memory.get_mem(0xFF80), 0x53);
		memory.set_mem(0xFFAB, 0x12);
		assert_eq!(memory.get_mem(0xFFAB), 0x12);
		memory.set_mem(0xFFFE, 0xEF);
		assert_eq!(memory.get_mem(0xFFFE), 0xEF);
	}

	#[test]
	fn test_echo_ram() {
		let mut memory = super::Memory::create_memory();
		memory.set_mem(0xE055, 0x2F);
		assert_eq!(memory.get_mem(0xC055), 0x2F);
		assert_eq!(memory.get_mem(0xE055), 0x2F);
		memory.set_mem(0xCBBF, 0xC3);
		assert_eq!(memory.get_mem(0xCBBF), 0xC3);
		assert_eq!(memory.get_mem(0xEBBF), 0xC3);
	}
}