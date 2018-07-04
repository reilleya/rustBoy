mod registers;
mod memory;
mod rom;

pub struct Core {
	pub reg: registers::Registers,
	pub mem: memory::Memory
}

impl Core {
	pub fn new() -> Core {
		Core {
			reg: registers::Registers::load_defaults(),
			mem: memory::Memory::create_memory()
		}
	}

	pub fn step(&mut self) {
		let ins = self.mem.get_mem(self.reg.pc);
		let _numsteps:(u16, u64) = match ins {
			0x00 => (1, 4),


			0x3C => self.inc_a(),


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

			0xC3 => {
				self.reg.pc = ((self.get_at_pc(2) as u16) << 8) + (self.get_at_pc(1) as u16);
				println!("{:4X}", self.reg.pc);
				(0, 4)
			}

			0x3E => {
				self.reg.a = self.mem.get_mem(self.reg.pc + 1);
				(2, 8)
			}

			_ => panic!("Instruction {:2X} not implemented!", ins)
		};
		self.reg.pc += _numsteps.0
	}

	fn get_at_pc(&self, offset:u16) -> u8 {
		self.mem.get_mem(self.reg.pc + offset)
	}

	fn inc_a(&mut self) -> (u16, u64) {
		self.reg.a = self.reg.a + 1;
		let zero = self.reg.a == 0;
		let half = (self.reg.a & 0xF) + 1 > 0xF;
		let carry = self.reg.get_c();
		self.reg.set_flags(zero, false, half, carry);
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