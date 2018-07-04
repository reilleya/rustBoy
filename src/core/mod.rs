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
			0x80 => self.op_80(),
			0x81 => self.op_81(),
			0x82 => self.op_82(),
			0x83 => self.op_83(),
			0x84 => self.op_84(),
			0x85 => self.op_85(),
			0x87 => self.op_87(),
			_ => panic!("Instruction {:2X} not implemented!", ins)
		};
		self.reg.pc += _numsteps.0
	}

	// A + B -> A
	fn op_80(&mut self) -> (u16, u64) {
		self.reg.a = self.reg.a + self.reg.b;
		let zero = self.reg.a == 0;
		let half = (self.reg.a & 0xF) + (self.reg.a & 0xF) > 0xF;
		let carry = false;
		self.reg.set_flags(zero, false, half, carry);
		(1, 4)
	}

	// A + C -> A
	fn op_81(&mut self) -> (u16, u64) {
		self.reg.a = self.reg.a + self.reg.c;
		(1, 4)
	}

	// A + D -> A
	fn op_82(&mut self) -> (u16, u64) {
		self.reg.a = self.reg.a + self.reg.d;
		(1, 4)
	}

	// A + E -> A
	fn op_83(&mut self) -> (u16, u64) {
		self.reg.a = self.reg.a + self.reg.e;
		(1, 4)
	}

	// A + H -> A
	fn op_84(&mut self) -> (u16, u64) {
		self.reg.a = self.reg.a + self.reg.h;
		(1, 4)
	}

	// A + L -> A
	fn op_85(&mut self) -> (u16, u64) {
		self.reg.a = self.reg.a + self.reg.l;
		(1, 4)
	}


	// A + A -> A
	fn op_87(&mut self) -> (u16, u64) {
		self.reg.a = self.reg.a + self.reg.a;
		(1, 4)
	}

}