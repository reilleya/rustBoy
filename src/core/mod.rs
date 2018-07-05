mod registers;
mod memory;
mod rom;
mod interrupts;

pub struct Core {
	pub reg: registers::Registers,
	pub mem: memory::Memory,
	pub int: interrupts::Interrupts
}

impl Core {
	pub fn new() -> Core {
		Core {
			reg: registers::Registers::load_defaults(),
			mem: memory::Memory::create_memory(),
			int: interrupts::Interrupts::create()
		}
	}

	pub fn step(&mut self) {
		let ins = self.mem.get_mem(self.reg.pc);
		//println!("Running {:2X} at {:2X}", ins, self.reg.pc);
		let _numsteps:(u16, u64) = match ins {
			0x00 => (1, 4),
			0x01 => {
				let val = self.get_16_pc(1);
				self.reg.set_bc(val);
				(3, 12)
			}

			0x05 => self.dec_b(),
			0x06 => {
				self.reg.b = self.get_8_pc(1);
				(2, 8)
			}


			0x0D => self.dec_c(),
			0x0E => {
				self.reg.c = self.get_8_pc(1);
				(2, 8)
			}


			0x11 => {
				let val = self.get_16_pc(1);
				self.reg.set_de(val);
				(3, 12)
			}


			0x15 => self.dec_d(),
			0x16 => {
				self.reg.d = self.get_8_pc(1);
				(2, 8)
			}


			0x1D => self.dec_e(),
			0x1E => {
				self.reg.e = self.get_8_pc(1);
				(2, 8)
			}


			0x20 => { // JR NZ
				if !self.reg.get_z() {
					let offset = self.get_8_pc(1) as u16;
					self.reg.pc += (offset & 0x7f);
					self.reg.pc -= 128 * ((offset & 0x80) >> 7)
				}
				(2, 8)
			}
			0x21 => {
				let val = self.get_16_pc(1);
				self.reg.set_hl(val);
				(3, 12)
			}


			0x25 => self.dec_h(),
			0x26 => {
				self.reg.h = self.get_8_pc(1);
				(2, 8)
			}


			0x2D => self.dec_l(),
			0x2E => {
				self.reg.l = self.get_8_pc(1);
				(2, 8)
			}


			0x31 => {
				self.reg.sp = self.get_16_pc(1);
				(3, 12)
			}
			0x32 => {
				let addr = self.reg.get_hl();
				self.mem.set_mem(addr, self.reg.a);
				self.reg.set_hl(addr - 1);
				(1, 8)
			}


			0x3C => self.inc_a(),


			0x3E => {
				self.reg.a = self.get_8_pc(1);
				(2, 8)
			}


			0x40 => (1, 4),
			0x41 => {
				self.reg.b = self.reg.c;
				(1, 4)
			}
			0x42 => {
				self.reg.b = self.reg.d;
				(1, 4)
			}
			0x43 => {
				self.reg.b = self.reg.e;
				(1, 4)
			}
			0x44 => {
				self.reg.b = self.reg.h;
				(1, 4)
			}
			0x45 => {
				self.reg.b = self.reg.l;
				(1, 4)
			}


			0x47 => {
				self.reg.b = self.reg.a;
				(1, 4)
			}
			0x48 => {
				self.reg.c = self.reg.b;
				(1, 4)
			}
			0x49 => (1, 4),
			0x4A => {
				self.reg.c = self.reg.d;
				(1, 4)
			}
			0x4B => {
				self.reg.c = self.reg.e;
				(1, 4)
			}
			0x4C => {
				self.reg.c = self.reg.h;
				(1, 4)
			}
			0x4D => {
				self.reg.c = self.reg.l;
				(1, 4)
			}


			0x4F => {
				self.reg.c = self.reg.a;
				(1, 4)
			}
			0x50 => {
				self.reg.d = self.reg.b;
				(1, 4)
			}
			0x51 => {
				self.reg.d = self.reg.c;
				(1, 4)
			}
			0x52 => (1, 4),
			0x53 => {
				self.reg.d = self.reg.e;
				(1, 4)
			}
			0x54 => {
				self.reg.d = self.reg.h;
				(1, 4)
			}
			0x55 => {
				self.reg.d = self.reg.l;
				(1, 4)
			}


			0x57 => {
				self.reg.d = self.reg.a;
				(1, 4)
			}
			0x58 => {
				self.reg.e = self.reg.b;
				(1, 4)
			}
			0x59 => {
				self.reg.e = self.reg.c;
				(1, 4)
			}
			0x5A => {
				self.reg.e = self.reg.d;
				(1, 4)
			}
			0x5B => (1, 4),
			0x5C => {
				self.reg.e = self.reg.h;
				(1, 4)
			}
			0x5D => {
				self.reg.e = self.reg.l;
				(1, 4)
			}


			0x5F => {
				self.reg.e = self.reg.a;
				(1, 4)
			}
			0x60 => {
				self.reg.h = self.reg.b;
				(1, 4)
			}
			0x61 => {
				self.reg.h = self.reg.c;
				(1, 4)
			}
			0x62 => {
				self.reg.h = self.reg.d;
				(1, 4)
			}
			0x63 => {
				self.reg.h = self.reg.e;
				(1, 4)
			}
			0x64 => (1, 4),
			0x65 => {
				self.reg.h = self.reg.l;
				(1, 4)
			}


			0x67 => {
				self.reg.h = self.reg.a;
				(1, 4)
			}
			0x68 => {
				self.reg.l = self.reg.b;
				(1, 4)
			}
			0x69 => {
				self.reg.l = self.reg.c;
				(1, 4)
			}
			0x6A => {
				self.reg.l = self.reg.d;
				(1, 4)
			}
			0x6B => {
				self.reg.l = self.reg.e;
				(1, 4)
			}
			0x6C => {
				self.reg.l = self.reg.h;
				(1, 4)
			}
			0x6D => (1, 4),


			0x6F => {
				self.reg.l = self.reg.a;
				(1, 4)
			}


			0x80 => self.op_80(),
			0x81 => self.op_81(),
			0x82 => self.op_82(),
			0x83 => self.op_83(),
			0x84 => self.op_84(),
			0x85 => self.op_85(),
			0x87 => self.op_87(),

			0x78 => {
				self.reg.a = self.reg.b;
				(1, 4)
			}
			0x79 => {
				self.reg.a = self.reg.c;
				(1, 4)
			}
			0x7A => {
				self.reg.a = self.reg.d;
				(1, 4)
			}
			0x7B => {
				self.reg.a = self.reg.e;
				(1, 4)
			}
			0x7C => {
				self.reg.a = self.reg.h;
				(1, 4)
			}
			0x7D => {
				self.reg.a = self.reg.l;
				(1, 4)
			}

			0x7F => (1, 4),


			0xAF => { // A xor A -> A
				self.reg.a = 0;
				self.reg.set_flags(true, false, false, false);
				(1, 4)
			}


			0xC3 => {
				self.reg.pc = self.get_16_pc(1);
				(0, 4)
			}


			0xE0 => {
				let addr = 0xFF00 + (self.get_8_pc(1) as u16);
				self.mem.set_mem(addr, self.reg.a);
				(2, 12)
			}



			0xF0 => {
				let addr = 0xFF00 + (self.get_8_pc(1) as u16);
				self.reg.a = self.mem.get_mem(addr);
				(2, 12)
			}


			0xF3 => {
				self.int.toggle(false);
				(1, 4)
			}

			_ => panic!("Instruction {:2X} at {:2X} not implemented!", ins, self.reg.pc)
		};
		self.reg.pc += _numsteps.0
	}

	fn get_8_pc(&self, offset:u16) -> u8 {
		self.mem.get_mem(self.reg.pc + offset)
	}

	fn get_16_pc(&self, offset:u16) -> u16 {
		((self.get_8_pc(offset + 1) as u16) << 8) + (self.get_8_pc(offset) as u16)
	}

	fn inc_a(&mut self) -> (u16, u64) {
		self.reg.a = self.reg.a + 1;
		let zero = self.reg.a == 0;
		let half = (self.reg.a & 0xF) + 1 > 0xF;
		let carry = self.reg.get_c();
		self.reg.set_flags(zero, false, half, carry);
		(1, 4)
	}

	fn dec_a(&mut self) -> (u16, u64) {
		let half = self.reg.a & 0xF == 0;
		let (res, carry) = self.reg.a.overflowing_sub(1);
		self.reg.a = res;
		let zero = self.reg.a == 0;
		let carry = self.reg.get_c();
		self.reg.set_flags(zero, true, half, carry);
		(1, 4)
	}

	fn dec_b(&mut self) -> (u16, u64) {
		let half = self.reg.b & 0xF == 0;
		let (res, carry) = self.reg.b.overflowing_sub(1);
		self.reg.b = res;
		let zero = self.reg.b == 0;
		let carry = self.reg.get_c();
		self.reg.set_flags(zero, true, half, carry);
		(1, 4)
	}

	fn dec_c(&mut self) -> (u16, u64) {
		let half = self.reg.c & 0xF == 0;
		let (res, carry) = self.reg.c.overflowing_sub(1);
		self.reg.c = res;
		let zero = self.reg.c == 0;
		let carry = self.reg.get_c();
		self.reg.set_flags(zero, true, half, carry);
		(1, 4)
	}

	fn dec_d(&mut self) -> (u16, u64) {
		let half = self.reg.d & 0xF == 0;
		let (res, carry) = self.reg.d.overflowing_sub(1);
		self.reg.d = res;
		let zero = self.reg.d == 0;
		let carry = self.reg.get_c();
		self.reg.set_flags(zero, true, half, carry);
		(1, 4)
	}
	
	fn dec_e(&mut self) -> (u16, u64) {
		let half = self.reg.e & 0xF == 0;
		let (res, carry) = self.reg.e.overflowing_sub(1);
		self.reg.e = res;
		let zero = self.reg.e == 0;
		let carry = self.reg.get_c();
		self.reg.set_flags(zero, true, half, carry);
		(1, 4)
	}

	fn dec_h(&mut self) -> (u16, u64) {
		let half = self.reg.h & 0xF == 0;
		let (res, carry) = self.reg.h.overflowing_sub(1);
		self.reg.h = res;
		let zero = self.reg.h == 0;
		let carry = self.reg.get_c();
		self.reg.set_flags(zero, true, half, carry);
		(1, 4)
	}

	fn dec_l(&mut self) -> (u16, u64) {
		let half = self.reg.l & 0xF == 0;
		let (res, carry) = self.reg.h.overflowing_sub(1);
		self.reg.l = res;
		let zero = self.reg.l == 0;
		let carry = self.reg.get_c();
		self.reg.set_flags(zero, true, half, carry);
		(1, 4)
	}

	// A + B -> A
	fn op_80(&mut self) -> (u16, u64) {
		let (res, carry) = self.reg.a.overflowing_add(self.reg.b);
		self.reg.a = res;
		let zero = self.reg.a == 0;
		let half = (self.reg.a & 0xF) + (self.reg.a & 0xF) > 0xF;
		self.reg.set_flags(zero, false, half, carry);
		(1, 4)
	}

	// A + C -> A
	fn op_81(&mut self) -> (u16, u64) {
		let (res, carry) = self.reg.a.overflowing_add(self.reg.c);
		self.reg.a = res;
		let zero = self.reg.a == 0;
		let half = (self.reg.a & 0xF) + (self.reg.c & 0xF) > 0xF;
		self.reg.set_flags(zero, false, half, carry);
		(1, 4)
	}

	// A + D -> A
	fn op_82(&mut self) -> (u16, u64) {
		let (res, carry) = self.reg.a.overflowing_add(self.reg.d);
		self.reg.a = res;
		let zero = self.reg.a == 0;
		let half = (self.reg.a & 0xF) + (self.reg.d & 0xF) > 0xF;
		self.reg.set_flags(zero, false, half, carry);
		(1, 4)
	}

	// A + E -> A
	fn op_83(&mut self) -> (u16, u64) {
		let (res, carry) = self.reg.a.overflowing_add(self.reg.e);
		self.reg.a = res;
		let zero = self.reg.a == 0;
		let half = (self.reg.a & 0xF) + (self.reg.e & 0xF) > 0xF;
		self.reg.set_flags(zero, false, half, carry);
		(1, 4)
	}

	// A + H -> A
	fn op_84(&mut self) -> (u16, u64) {
		let (res, carry) = self.reg.a.overflowing_add(self.reg.h);
		self.reg.a = res;
		let zero = self.reg.a == 0;
		let half = (self.reg.a & 0xF) + (self.reg.h & 0xF) > 0xF;
		self.reg.set_flags(zero, false, half, carry);
		(1, 4)
	}

	// A + L -> A
	fn op_85(&mut self) -> (u16, u64) {
		let (res, carry) = self.reg.a.overflowing_add(self.reg.l);
		self.reg.a = res;
		let zero = self.reg.a == 0;
		let half = (self.reg.a & 0xF) + (self.reg.l & 0xF) > 0xF;
		self.reg.set_flags(zero, false, half, carry);
		(1, 4)
	}

	// A + A -> A
	fn op_87(&mut self) -> (u16, u64) {
		let (res, carry) = self.reg.a.overflowing_add(self.reg.a);
		self.reg.a = res;
		let zero = self.reg.a == 0;
		let half = (self.reg.a & 0xF) + (self.reg.a & 0xF) > 0xF;
		self.reg.set_flags(zero, false, half, carry);
		(1, 4)
	}

}