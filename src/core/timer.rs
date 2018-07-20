pub struct Timer {
	pub cycles: u64,
}

impl Timer {
	pub fn create() -> Timer {
		Timer {
			cycles: 0
		}
	}

	pub fn step(&mut self, cycles:u64){
		self.cycles += cycles;
	}

}