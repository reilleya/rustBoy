mod registers;

pub struct Core {
	reg: registers::Registers
}

impl Core {
	pub fn new() -> Core {
		Core {
			reg: registers::Registers::load_defaults()
		}
	}

	pub fn step(&mut self) {
		self.reg.pc += 1;
	}
}