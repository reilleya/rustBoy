mod registers;
mod memory;
mod rom;
mod interrupts;
mod display;
mod timer;

pub struct Core {
	pub reg: registers::Registers,
	pub mem: memory::Memory,
	pub int: interrupts::Interrupts,
}

fn check_add_half_carry(a:u8, b:u8) -> bool {
	let lower = a & 0xF;
	let (_res, carry) = lower.overflowing_add(b & 0xF);
	carry
}

fn check_sub_half_carry(a:u8, b:u8) -> bool {
	let lower = a & 0xF;
	let (_res, carry) = lower.overflowing_sub(b & 0xF);
	carry
}

fn check_add_half_carry_16(a:u16, b:u16) -> bool {
	((a & 0xFFF) + (b & 0xFFF) & 0x1000) == 0x1000
}

fn check_sub_half_carry_16(a:u16, b:u16) -> bool {
	(((a & 0x0FFF) + 0x1000) - (b & 0x0FFF) & 0x1000) == 0x1000
}

impl Core {
	pub fn new() -> Core {
		Core {
			reg: registers::Registers::load_defaults(),
			mem: memory::Memory::create_memory(),
			int: interrupts::Interrupts::create(),
		}
	}

	pub fn step(&mut self) {
		let ins = self.mem.get_mem(self.reg.pc);
		println!("Running {:2X} at {:2X}", ins, self.reg.pc);
		let _numsteps:(u16, u64) = match ins {
			0x00 => (1, 4),
			0x01 => {
				let val = self.get_16_pc(1);
				self.reg.set_bc(val);
				(3, 12)
			}
			0x02 => {
				let addr = self.reg.get_bc();
				let val = self.reg.a;
				self.mem.set_mem(addr, val);
				(1, 8)
			}
			0x03 => {
				let val = self.reg.get_bc();
				let (res, _carry) = val.overflowing_add(1);
				self.reg.set_bc(res);
				(1, 8)
			}
			0x04 => self.inc_reg(registers::RegisterName::b),
			0x05 => self.dec_b(),
			0x06 => {
				self.reg.b = self.get_8_pc(1);
				(2, 8)
			}

			0x08 => {
				let addr = self.get_16_pc(1);
				let loval = (self.reg.sp & 0x00FF) as u8;
				let hival = ((self.reg.sp & 0xFF00) >> 8) as u8;
				self.mem.set_mem(addr, loval);
				self.mem.set_mem(addr + 1, hival);
				(3, 20)
			}
			0x09 => {
				let operand = self.reg.get_bc();
				self.add_hl(operand)
			}
			0x0A => {
				let addr = self.reg.get_bc();
				self.reg.a = self.mem.get_mem(addr);
				(1, 8)
			}
			0x0B => {
				let val = self.reg.get_bc();
				self.reg.set_bc(val - 1);
				(1, 8)
			}
			0x0C => self.inc_reg(registers::RegisterName::c),
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
			0x12 => {
				let addr = self.reg.get_de();
				let val = self.reg.a;
				self.mem.set_mem(addr, val);
				(1, 8)
			}
			0x13 => {
				let val = self.reg.get_de() + 1;
				self.reg.set_de(val);
				(1, 8)
			}
			0x14 => self.inc_reg(registers::RegisterName::d),
			0x15 => self.dec_d(),
			0x16 => {
				self.reg.d = self.get_8_pc(1);
				(2, 8)
			}

			0x18 => {
				let raw = self.get_8_pc(1);
				self.reg.pc += raw as u16;
				if raw & 0x80 != 0 {
					self.reg.pc -= 256
				}
				(2, 8)
			}
			0x19 => {
				let operand = self.reg.get_de();
				self.add_hl(operand)
			}
			0x1A => {
				let addr = self.reg.get_de();
				self.reg.a = self.mem.get_mem(addr);
				(1, 8)
			}
			0x1B => {
				let val = self.reg.get_de();
				self.reg.set_de(val - 1);
				(1, 8)
			}
			0x1C => self.inc_reg(registers::RegisterName::e),
			0x1D => self.dec_e(),
			0x1E => {
				self.reg.e = self.get_8_pc(1);
				(2, 8)
			}
			0x1F => {
				let b0 = self.reg.a & 0x1;
				let mut res = self.reg.a >> 1;
				res &= (self.reg.get_c() as u8) << 7;
				self.reg.a = res;
				self.reg.set_flags(res == 0, false, false, b0 == 0x1);
				(1, 4)
			}
			0x20 => { // JR NZ
				if !self.reg.get_z() {
					let offset = self.get_8_pc(1) as u16;
					self.reg.pc += offset & 0x7f;
					self.reg.pc -= 128 * ((offset & 0x80) >> 7)
				}
				(2, 8)
			}
			0x21 => {
				let val = self.get_16_pc(1);
				self.reg.set_hl(val);
				(3, 12)
			}
			0x22 => {
				let addr = self.reg.get_hl();
				let val = self.reg.a;
				self.mem.set_mem(addr, val);
				self.reg.set_hl(addr + 1);
				(1, 8)
			}

			0x23 => {
				let val = self.reg.get_hl() + 1;
				self.reg.set_hl(val);
				(1, 8)
			}
			0x24 => self.inc_reg(registers::RegisterName::h),
			0x25 => self.dec_h(),
			0x26 => {
				self.reg.h = self.get_8_pc(1);
				(2, 8)
			}

			0x28 => { // JR Z
				if self.reg.get_z() {
					let offset = self.get_8_pc(1) as u16;
					self.reg.pc += offset & 0x7f;
					self.reg.pc -= 128 * ((offset & 0x80) >> 7)
				}
				(2, 8)
			}
			0x29 => {
				let operand = self.reg.get_hl();
				self.add_hl(operand)
			}

			0x2A => {
				let val = self.mem.get_mem(self.reg.get_hl());
				self.reg.a = val;
				let hl = self.reg.get_hl();
				self.reg.set_hl(hl + 1);
				(1, 8)
			}
			0x2B => {
				let val = self.reg.get_hl();
				self.reg.set_hl(val - 1);
				(1, 8)
			}
			0x2C => self.inc_reg(registers::RegisterName::l),
			0x2D => self.dec_l(),
			0x2E => {
				self.reg.l = self.get_8_pc(1);
				(2, 8)
			}
			0x2F => {
				self.reg.a = !self.reg.a;
				let z = self.reg.get_z();
				let c = self.reg.get_c();
				self.reg.set_flags(z, true, true, c);
				(1, 4)
			}
			0x30 => { // JR NC
				if !self.reg.get_c() {
					let offset = self.get_8_pc(1) as u16;
					self.reg.pc += offset & 0x7f;
					self.reg.pc -= 128 * ((offset & 0x80) >> 7)
				}
				(2, 8)
			}
			0x31 => {
				self.reg.sp = self.get_16_pc(1);
				(3, 12)
			}
			0x32 => {
				let addr = self.reg.get_hl();
				self.mem.set_mem(addr, self.reg.a);
				if addr == 0 {
					self.reg.set_hl(0xFF);
				}
				else {
					self.reg.set_hl(addr - 1);
				}
				(1, 8)
			}
			0x33 => {
				self.reg.sp += 1;
				(1, 8)
			}

			0x35 => {
				let addr = self.reg.get_hl();
				let operand = self.mem.get_mem(addr);
				let (res, _carry) = operand.overflowing_sub(1);
				let c = self.reg.get_c();
				self.reg.set_flags(res == 0, true, false, c); // half carry
				self.mem.set_mem(addr, res);
				(1, 12)
			}
			0x36 => {
				let put_addr = self.reg.get_hl();
				let put_val = self.get_8_pc(1);
				self.mem.set_mem(put_addr, put_val);
				(2, 12)
			}

			0x38 => { // JR C
				if self.reg.get_c() {
					let offset = self.get_8_pc(1) as u16;
					self.reg.pc += offset & 0x7f;
					self.reg.pc -= 128 * ((offset & 0x80) >> 7)
				}
				(2, 8)
			}
			0x39 => {
				let operand = self.reg.sp;
				self.add_hl(operand)
			}

			0x3B => {
				self.reg.sp = self.reg.sp - 1;
				(1, 8)
			}
			0x3C => self.inc_reg(registers::RegisterName::a),
			0x3D => self.dec_reg(registers::RegisterName::a),
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
			0x46 => {
				let addr = self.reg.get_hl();
				self.reg.b = self.mem.get_mem(addr);
				(1, 8)
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
			0x4E => {
				let addr = self.reg.get_hl();
				self.reg.c = self.mem.get_mem(addr);
				(1, 8)
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
			0x56 => {
				let addr = self.reg.get_hl();
				self.reg.d = self.mem.get_mem(addr);
				(1, 8)
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
			0x5E => {
				let addr = self.reg.get_hl();
				self.reg.e = self.mem.get_mem(addr);
				(1, 8)
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
			0x66 => {
				let addr = self.reg.get_hl();
				self.reg.h = self.mem.get_mem(addr);
				(1, 8)
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
			0x6E => {
				let addr = self.reg.get_hl();
				self.reg.l = self.mem.get_mem(addr);
				(1, 8)
			}
			0x6F => {
				self.reg.l = self.reg.a;
				(1, 4)
			}
			0x70 => {
				let val = self.reg.b;
				let addr = self.reg.get_hl();
				self.mem.set_mem(addr, val);
				(1, 8)
			}
			0x71 => {
				let val = self.reg.c;
				let addr = self.reg.get_hl();
				self.mem.set_mem(addr, val);
				(1, 8)
			}
			0x72 => {
				let val = self.reg.d;
				let addr = self.reg.get_hl();
				self.mem.set_mem(addr, val);
				(1, 8)
			}
			0x73 => {
				let val = self.reg.e;
				let addr = self.reg.get_hl();
				self.mem.set_mem(addr, val);
				(1, 8)
			}
			0x74 => {
				let val = self.reg.h;
				let addr = self.reg.get_hl();
				self.mem.set_mem(addr, val);
				(1, 8)
			}
			0x75 => {
				let val = self.reg.l;
				let addr = self.reg.get_hl();
				self.mem.set_mem(addr, val);
				(1, 8)
			}

			0x77 => {
				let addr = self.reg.get_hl();
				let val = self.reg.a;
				self.mem.set_mem(addr, val);
				(1, 8)
			}
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
			0x7E => {
				let addr = self.reg.get_hl();
				self.reg.a = self.mem.get_mem(addr);
				(1, 8)
			}
			0x7F => (1, 4),
			0x80 => self.op_80(),
			0x81 => self.op_81(),
			0x82 => self.op_82(),
			0x83 => self.op_83(),
			0x84 => self.op_84(),
			0x85 => self.op_85(),
			0x87 => self.op_87(),

			0x90 => { // SUB A, B
				let val = self.reg.b;
				self.sub_a(val);
				(1, 4)
			}
			0x91 => { // SUB A, C
				let val = self.reg.c;
				self.sub_a(val);
				(1, 4)
			}
			0x92 => { // SUB A, D
				let val = self.reg.d;
				self.sub_a(val);
				(1, 4)
			}
			0x93 => { // SUB A, E
				let val = self.reg.e;
				self.sub_a(val);
				(1, 4)
			}
			0x94 => { // SUB A, H
				let val = self.reg.h;
				self.sub_a(val);
				(1, 4)
			}
			0x95 => { // SUB A, L
				let val = self.reg.l;
				self.sub_a(val);
				(1, 4)
			}
			0x96 => { // SUB A, (HL)
				let val = self.mem.get_mem(self.reg.get_hl());
				self.sub_a(val);
				(1, 4)
			}

			0x9B => {
				let val = self.reg.e;
				self.subc_a(val);
				(1, 4)
			}

			0xA0 => {
				let operand = self.reg.b;
				self.and_a(operand)
			}
			0xA1 => {
				let operand = self.reg.c;
				self.and_a(operand)
			}
			0xA2 => {
				let operand = self.reg.d;
				self.and_a(operand)
			}
			0xA3 => {
				let operand = self.reg.e;
				self.and_a(operand)
			}
			0xA4 => {
				let operand = self.reg.h;
				self.and_a(operand)
			}
			0xA5 => {
				let operand = self.reg.l;
				self.and_a(operand)
			}
			0xA6 => {
				let operand = self.mem.get_mem(self.reg.get_hl());
				self.and_a(operand);
				(1,8)
			}
			0xA7 => {
				let operand = self.reg.a;
				self.and_a(operand)
			}
			0xA8 => {
				let operand = self.reg.b;
				self.xor_a(operand)
			}
			0xA9 => {
				let operand = self.reg.c;
				self.xor_a(operand)
			}
			0xAA => {
				let operand = self.reg.d;
				self.xor_a(operand)
			}
			0xAB => {
				let operand = self.reg.e;
				self.xor_a(operand)
			}
			0xAC => {
				let operand = self.reg.h;
				self.xor_a(operand)
			}
			0xAD => {
				let operand = self.reg.l;
				self.xor_a(operand)
			}
			0xAE => {
				let operand = self.mem.get_mem(self.reg.get_hl());
				self.xor_a(operand);
				(1,8)
			}
			0xAF => { // A xor A -> A
				self.reg.a = 0;
				self.reg.set_flags(true, false, false, false);
				(1, 4)
			}
			0xB0 => {
				let operand = self.reg.b;
				self.or_a(operand)
			}
			0xB1 => {
				let operand = self.reg.c;
				self.or_a(operand)
			}
			0xB2 => {
				let operand = self.reg.d;
				self.or_a(operand)
			}
			0xB3 => {
				let operand = self.reg.e;
				self.or_a(operand)
			}
			0xB4 => {
				let operand = self.reg.h;
				self.or_a(operand)
			}
			0xB5 => {
				let operand = self.reg.l;
				self.or_a(operand)
			}
			0xB6 => {
				let operand = self.mem.get_mem(self.reg.get_hl());
				self.reg.a = self.reg.a | operand;
				let z = self.reg.a == 0;
				self.reg.set_flags(z, false, false, false);
				(1, 8)
			}
			0xB7 => {
				let operand = self.reg.a;
				self.or_a(operand)
			}

			0xC0 => {
				if !self.reg.get_z() {
					self.reg.pc = self.pop();
					(0, 8)
				} else {
					(1, 8)
				}
			}
			0xC1 => {
				let popped = self.pop();
				self.reg.set_bc(popped);
				(1, 12)
			}
			0xC2 => {
				if !self.reg.get_z() {
					self.reg.pc = self.get_16_pc(1);
					(0, 12)
				} else {
					(3, 12)
				}
			}
			0xC3 => {
				self.reg.pc = self.get_16_pc(1);
				(0, 4)
			}
			0xC4 => { // CALL NZ
				if !self.reg.get_z() {
					let pc = self.reg.pc;
					self.reg.pc = self.get_16_pc(1) - 2;
					self.push(pc + 2); // +2 so it will ret to the next op
					(2, 12)
				}
				else {
					(3, 12)
				}
			}
			0xC5 => {
				let val = self.reg.get_bc();
				self.push(val);
				(1, 16)
			}
			0xC6 => {
				let val = self.get_8_pc(1);
				let (res, _carry) = self.reg.a.overflowing_add(val);
				self.reg.a = res;
				self.reg.set_flags(res == 0, false, false, _carry); // Fix half carry
				(2, 8)
			}
			0xC7 => {
				let addr = self.reg.pc;
				self.push(addr);
				self.reg.pc = 0;
				(1, 32)
			}
			0xC8 => {
				if self.reg.get_z() {
					self.reg.pc = self.pop();
					(0, 8)
				} else {
					(1, 8)
				}
			}
			0xC9 => {
				self.reg.pc = self.pop();
				(1, 8)
			}
			0xCA => {
				if self.reg.get_z() {
					self.reg.pc = self.get_16_pc(1);
					(0, 12)
				} else {
					(3, 12)
				}
			}
			0xCB => {
				let op = self.get_8_pc(1);
				self.handle_cb(op)
			}
			0xCC => { // CALL Z
				if self.reg.get_z() {
					let pc = self.reg.pc;
					self.reg.pc = self.get_16_pc(1) - 2;
					self.push(pc + 2); // +2 so it will ret to the next op
					(2, 12)
				}
				else {
					(3, 12)
				}
			}
			0xCD => {
				let pc = self.reg.pc;
				self.reg.pc = self.get_16_pc(1) - 2;
				self.push(pc + 2); // +2 so it will ret to the next op
				(2, 12)
			}
			0xCE => { // ADC A,#
				let (operand, op_Carry) = self.get_8_pc(1).overflowing_add(self.reg.get_c() as u8);
				let hc = check_add_half_carry(self.reg.a, operand);
				let (res, carry) = self.reg.a.overflowing_add(operand);
				self.reg.a = res;
				self.reg.set_flags(res == 0, false, hc, carry);
				(2, 8)
			}
			0xCF => {
				let addr = self.reg.pc;
				self.push(addr + 1);
				self.reg.pc = 0x08;
				(0, 32)
			}
			0xD0 => {
				if !self.reg.get_c() {
					self.reg.pc = self.pop();
					(0, 8)
				} else {
					(1, 8)
				}
			}
			0xD1 => {
				let popped = self.pop();
				self.reg.set_de(popped);
				(1, 12)
			}
			0xD2 => {
				if !self.reg.get_c() {
					self.reg.pc = self.get_16_pc(1);
					(0, 12)
				} else {
					(3, 12)
				}
			}

			0xD4 => { // CALL NC
				if !self.reg.get_c() {
					let pc = self.reg.pc;
					self.reg.pc = self.get_16_pc(1) - 2;
					self.push(pc + 2); // +2 so it will ret to the next op
					(2, 12)
				}
				else {
					(3, 12)
				}
			}
			0xD5 => {
				let val = self.reg.get_de();
				self.push(val);
				(1, 16)
			}
			0xD6 => {
				let val = self.get_8_pc(1);
				let (res, _carry) = self.reg.a.overflowing_sub(val);
				self.reg.a = res;
				self.reg.set_flags(res == 0, true, false, !_carry); // Fix half carry
				(2, 8)
			}
			0xD7 => {
				let addr = self.reg.pc;
				self.push(addr + 1);
				self.reg.pc = 0x10;
				(0, 32)
			}
			0xD8 => {
				if self.reg.get_c() {
					self.reg.pc = self.pop();
					(0, 8)
				} else {
					(1, 8)
				}
			}

			0xDA => {
				if self.reg.get_c() {
					self.reg.pc = self.get_16_pc(1);
					(0, 12)
				} else {
					(3, 12)
				}
			}

			0xDC => { // CALL C
				if self.reg.get_c() {
					let pc = self.reg.pc;
					self.reg.pc = self.get_16_pc(1) - 2;
					self.push(pc + 2); // +2 so it will ret to the next op
					(2, 12)
				}
				else {
					(3, 12)
				}
			}

			0xDF => {
				let addr = self.reg.pc;
				self.push(addr + 1);
				self.reg.pc = 0x18;
				(0, 32)
			}
			0xE0 => {
				let addr = 0xFF00 + (self.get_8_pc(1) as u16);
				self.mem.set_mem(addr, self.reg.a);
				(2, 12)
			}
			0xE1 => {
				let popped = self.pop();
				self.reg.set_hl(popped);
				(1, 12)
			}
			0xE2 => {
				let addr = 0xFF00 + (self.reg.c as u16);
				self.mem.set_mem(addr, self.reg.a);
				(1, 8)
			}

			0xE5 => {
				let val = self.reg.get_hl();
				self.push(val);
				(1, 16)
			}
			0xE6 => {
				let operand = self.get_8_pc(1);
				self.and_a(operand);
				(2, 8)
			}
			0xE7 => {
				let addr = self.reg.pc;
				self.push(addr + 1);
				self.reg.pc = 0x20;
				(0, 32)
			}

			0xE9 => {
				self.reg.pc = self.reg.get_hl();
				(0, 4)
			}

			0xEA => {
				let put_addr = self.get_16_pc(1);
				self.mem.set_mem(put_addr, self.reg.a);
				(3, 16)
			}

			0xEE => {
				let operand = self.get_8_pc(1);
				self.xor_a(operand);
				(2, 8)
			}
			0xEF => {
				let addr = self.reg.pc;
				self.push(addr + 1);
				self.reg.pc = 0x28;
				(0, 32)
			}
			0xF0 => {
				let addr = 0xFF00 + (self.get_8_pc(1) as u16);
				self.reg.a = self.mem.get_mem(addr);
				(2, 12)
			}
			0xF1 => {
				let popped = self.pop();
				self.reg.set_af(popped);
				(1, 12)
			}

			0xF3 => {
				self.int.toggle(false);
				(1, 4)
			}

			0xF5 => {
				let val = self.reg.get_af();
				self.push(val);
				(1, 16)
			}

			0xF7 => {
				let addr = self.reg.pc;
				self.push(addr + 1);
				self.reg.pc = 0x30;
				(0, 32)
			}

			0xFA => {
				let addr = self.get_16_pc(1);
				self.reg.a = self.mem.get_mem(addr);
				(3, 16)
			}
			0xFB => {
				self.int.toggle(true);
				(1, 4)
			}			

			0xFE => { // CP A, #
				let cmp = self.get_8_pc(1);
				let a = self.reg.a;
				self.reg.set_z(a == cmp);
				self.reg.set_n(true);
				self.reg.set_h(check_sub_half_carry(a, cmp));
				self.reg.set_c(a < cmp);
				(2, 8)
			}
			0xFF => {
				let addr = self.reg.pc;
				self.push(addr + 1);
				self.reg.pc = 0x38;
				(0, 32)
			}
			_ => panic!("Instruction {:2X} at {:2X} not implemented!", ins, self.reg.pc)
		};
		self.reg.disp_state();
		println!("");
		self.reg.pc += _numsteps.0;
		self.mem.update(_numsteps.1);
	}

	fn get_8_pc(&mut self, offset:u16) -> u8 {
		self.mem.get_mem(self.reg.pc + offset)
	}

	fn get_16_pc(&mut self, offset:u16) -> u16 {
		((self.get_8_pc(offset + 1) as u16) << 8) + (self.get_8_pc(offset) as u16)
	}

	fn push(&mut self, value:u16) {
		self.mem.set_mem(self.reg.sp, (value & 0x00FF) as u8);
		self.mem.set_mem(self.reg.sp - 1, ((value & 0xFF00) >> 8) as u8);
		self.reg.sp -= 2;
	}

	fn pop(&mut self) -> u16 {
		let low = self.mem.get_mem(self.reg.sp + 2) as u16;
		let high = self.mem.get_mem(self.reg.sp + 1) as u16;
		self.reg.sp += 2;
		(high << 8) + low
	}

	fn or_a(&mut self, operand:u8) -> (u16, u64) {
		self.reg.a = self.reg.a | operand;
		let z = self.reg.a == 0;
		self.reg.set_flags(z, false, false, false);
		(1, 4)
	}

	fn and_a(&mut self, operand:u8) -> (u16, u64) {
		self.reg.a = self.reg.a & operand;
		let z = self.reg.a == 0;
		self.reg.set_flags(z, false, true, false);
		(1, 4)
	}

	fn xor_a(&mut self, operand:u8) -> (u16, u64) {
		self.reg.a ^= operand;
		let z = self.reg.a == 0;
		self.reg.set_flags(z, false, false, false);
		(1, 4)
	}

	fn add_hl(&mut self, operand:u16) -> (u16, u64) {
		let hc = check_add_half_carry_16(self.reg.get_hl(), operand);
		let operand = self.reg.get_hl();
		let (res, carry) = operand.overflowing_add(operand);
		self.reg.set_hl(res);
		let z = self.reg.get_z();
		self.reg.set_flags(z, false, hc, carry);
		(1, 8)
	}

	fn handle_cb(&mut self, op:u8) -> (u16, u64) {
		let mut operand = match op & 0x0F {
			0x0 | 0x8 => self.reg.b,
			0x1 | 0x9 => self.reg.c,
			0x2 | 0xA => self.reg.d,
			0x3 | 0xB => self.reg.e,
			0x4 | 0xC => self.reg.h,
			0x5 | 0xD => self.reg.l,

			0x7 | 0xF => self.reg.a,
			_ => panic!("Invalid CB operand")
		};

		operand = match op {
			0x00 ..= 0x07 => operand, // Rotate left through carry
			0x07 ..= 0x0F => operand, // Rotate right through carry
			
			0x19 ..= 0x1D => {
				let b0 = operand & 0x1;
				let mut res = operand >> 1;
				res |= (self.reg.get_c() as u8) << 7;
				self.reg.set_flags(res == 0, false, false, b0 == 0x1);
				res
			}

			0x30 ..= 0x37 => { // Swap
				let res = ((operand & 0xF0) >> 4) + ((operand & 0x0F) << 4);
				self.reg.set_flags(res == 0, false, false, false);
				res
			}
			0x38 ..= 0x3D | 0x3F => {
				let b0 = operand & 0x1;
				let res = operand >> 1;
				self.reg.set_flags(res == 0, false, false, b0 == 0x1);
				res
			}

			0x80 ..= 0x87 => operand & 0xFE, // Reset bit 0
			0x88 ..= 0x8F => operand & 0xFD, // Reset bit 1
			0x90 ..= 0x97 => operand & 0xFB, // Reset bit 2
			0x98 ..= 0x9F => operand & 0xF7, // Reset bit 3
			0xA0 ..= 0xA7 => operand & 0xEF, // Reset bit 4
			0xA8 ..= 0xAF => operand & 0xDF, // Reset bit 5
			0xB0 ..= 0xB7 => operand & 0xBF, // Reset bit 6
			0xB8 ..= 0xBF => operand & 0x7F, // Reset bit 7
			0xC0 ..= 0xC7 => operand | 0x01, // Set bit 0
			0xC8 ..= 0xCF => operand | 0x02, // Set bit 1
			0xD0 ..= 0xD7 => operand | 0x04, //	Set bit 2
			0xD8 ..= 0xDF => operand | 0x08, // Set bit 3
			0xE0 ..= 0xE7 => operand | 0x10, // Set bit 4
			0xE8 ..= 0xEF => operand | 0x20, // Set bit 5
			0xF0 ..= 0xF7 => operand | 0x40, // Set bit 6
			0xF8 ..= 0xFF => operand | 0x80, // Set bit 7
			_ => panic!("Unimplemented CB operation {:2X}", op)
		};
		match op & 0x0F {
			0x0 | 0x8 => self.reg.b = operand,
			0x1 | 0x9 => self.reg.c = operand,
			0x2 | 0xA => self.reg.d = operand,
			0x3 | 0xB => self.reg.e = operand,
			0x4 | 0xC => self.reg.h = operand,
			0x5 | 0xD => self.reg.l = operand,

			0x7 | 0xF => self.reg.a = operand,
			_ => panic!("Invalid CB operand")
		};
		(2, 8) // TODO: Fix timing for (HL) operations
	}

	fn inc_reg(&mut self, reg_name:registers::RegisterName) -> (u16, u64) {
		let operand = match reg_name {
			registers::RegisterName::a => self.reg.a,
			registers::RegisterName::b => self.reg.b,
			registers::RegisterName::c => self.reg.c,
			registers::RegisterName::d => self.reg.d,
			registers::RegisterName::e => self.reg.e,
			registers::RegisterName::h => self.reg.h,
			registers::RegisterName::l => self.reg.l,			
			_ => panic!("Invalid register in inc")
		};
		let (res, carry) = operand.overflowing_add(1);
		let cf = self.reg.get_c();
		let half = self.reg.e & 0x0F == 0x0F;
		self.reg.set_flags(carry, false, half, cf);
		match reg_name {
			registers::RegisterName::a => self.reg.a = res,
			registers::RegisterName::b => self.reg.b = res,
			registers::RegisterName::c => self.reg.c = res,
			registers::RegisterName::d => self.reg.d = res,
			registers::RegisterName::e => self.reg.e = res,
			registers::RegisterName::h => self.reg.h = res,
			registers::RegisterName::l => self.reg.l = res,			
			_ => panic!("Invalid register in inc")
		};
		(1, 4)
	}

	fn dec_reg(&mut self, reg_name:registers::RegisterName) -> (u16, u64) {
		let operand = match reg_name {
			registers::RegisterName::a => self.reg.a,
			registers::RegisterName::b => self.reg.b,
			registers::RegisterName::c => self.reg.c,
			registers::RegisterName::d => self.reg.d,
			registers::RegisterName::e => self.reg.e,
			registers::RegisterName::h => self.reg.h,
			registers::RegisterName::l => self.reg.l,			
			_ => panic!("Invalid register in inc")
		};
		let (res, carry) = operand.overflowing_sub(1);
		let cf = self.reg.get_c();
		let half = self.reg.e & 0x0F == 0x0F; // Check this!
		self.reg.set_flags(carry, false, half, cf);
		match reg_name {
			registers::RegisterName::a => self.reg.a = res,
			registers::RegisterName::b => self.reg.b = res,
			registers::RegisterName::c => self.reg.c = res,
			registers::RegisterName::d => self.reg.d = res,
			registers::RegisterName::e => self.reg.e = res,
			registers::RegisterName::h => self.reg.h = res,
			registers::RegisterName::l => self.reg.l = res,			
			_ => panic!("Invalid register in inc")
		};
		(1, 4)
	}

	fn dec_a(&mut self) -> (u16, u64) {
		let half = self.reg.a & 0xF == 0;
		let (res, _carry) = self.reg.a.overflowing_sub(1);
		self.reg.a = res;
		let zero = self.reg.a == 0;
		let carry = self.reg.get_c();
		self.reg.set_flags(zero, true, half, carry);
		(1, 4)
	}

	fn dec_b(&mut self) -> (u16, u64) {
		let half = self.reg.b & 0xF == 0;
		let (res, _carry) = self.reg.b.overflowing_sub(1);
		self.reg.b = res;
		let zero = self.reg.b == 0;
		let carry = self.reg.get_c();
		self.reg.set_flags(zero, true, half, carry);
		(1, 4)
	}

	fn dec_c(&mut self) -> (u16, u64) {
		let half = self.reg.c & 0xF == 0;
		let (res, _carry) = self.reg.c.overflowing_sub(1);
		self.reg.c = res;
		let zero = self.reg.c == 0;
		let carry = self.reg.get_c();
		self.reg.set_flags(zero, true, half, carry);
		(1, 4)
	}

	fn dec_d(&mut self) -> (u16, u64) {
		let half = self.reg.d & 0xF == 0;
		let (res, _carry) = self.reg.d.overflowing_sub(1);
		self.reg.d = res;
		let zero = self.reg.d == 0;
		let carry = self.reg.get_c();
		self.reg.set_flags(zero, true, half, carry);
		(1, 4)
	}
	
	fn dec_e(&mut self) -> (u16, u64) {
		let half = self.reg.e & 0xF == 0;
		let (res, _carry) = self.reg.e.overflowing_sub(1);
		self.reg.e = res;
		let zero = self.reg.e == 0;
		let carry = self.reg.get_c();
		self.reg.set_flags(zero, true, half, carry);
		(1, 4)
	}

	fn dec_h(&mut self) -> (u16, u64) {
		let half = self.reg.h & 0xF == 0;
		let (res, _carry) = self.reg.h.overflowing_sub(1);
		self.reg.h = res;
		let zero = self.reg.h == 0;
		let carry = self.reg.get_c();
		self.reg.set_flags(zero, true, half, carry);
		(1, 4)
	}

	fn dec_l(&mut self) -> (u16, u64) {
		let half = self.reg.l & 0xF == 0;
		let (res, _carry) = self.reg.h.overflowing_sub(1);
		self.reg.l = res;
		let zero = self.reg.l == 0;
		let carry = self.reg.get_c();
		self.reg.set_flags(zero, true, half, carry);
		(1, 4)
	}

	fn subc_a(&mut self, val:u8) {
		let hc = check_sub_half_carry(self.reg.a, (self.reg.get_c() as u8) + val);
		let (res, carry) = self.reg.a.overflowing_sub((self.reg.get_c() as u8) + val);
		self.reg.a = res;
		self.reg.set_flags(res == 0, true, hc, !carry);
	}

	fn sub_a(&mut self, val:u8) {
		let hc = check_sub_half_carry(self.reg.a, val);
		let (res, carry) = self.reg.a.overflowing_sub(val);
		self.reg.a = res;
		self.reg.set_flags(res == 0, true, hc, !carry);
	}

	// A + B -> A
	fn op_80(&mut self) -> (u16, u64) {
		let half = (self.reg.a & 0xF) + (self.reg.b & 0xF) > 0xF;
		let (res, _carry) = self.reg.a.overflowing_add(self.reg.b);
		self.reg.a = res;
		let zero = self.reg.a == 0;
		self.reg.set_flags(zero, false, half, _carry);
		(1, 4)
	}

	// A + C -> A
	fn op_81(&mut self) -> (u16, u64) {
		let half = (self.reg.a & 0xF) + (self.reg.c & 0xF) > 0xF;
		let (res, _carry) = self.reg.a.overflowing_add(self.reg.c);
		self.reg.a = res;
		let zero = self.reg.a == 0;
		self.reg.set_flags(zero, false, half, _carry);
		(1, 4)
	}

	// A + D -> A
	fn op_82(&mut self) -> (u16, u64) {
		let half = (self.reg.a & 0xF) + (self.reg.d & 0xF) > 0xF;
		let (res, _carry) = self.reg.a.overflowing_add(self.reg.d);
		self.reg.a = res;
		let zero = self.reg.a == 0;
		self.reg.set_flags(zero, false, half, _carry);
		(1, 4)
	}

	// A + E -> A
	fn op_83(&mut self) -> (u16, u64) {
		let half = (self.reg.a & 0xF) + (self.reg.e & 0xF) > 0xF;
		let (res, _carry) = self.reg.a.overflowing_add(self.reg.e);
		self.reg.a = res;
		let zero = self.reg.a == 0;
		self.reg.set_flags(zero, false, half, _carry);
		(1, 4)
	}

	// A + H -> A
	fn op_84(&mut self) -> (u16, u64) {
		let half = (self.reg.a & 0xF) + (self.reg.h & 0xF) > 0xF;
		let (res, _carry) = self.reg.a.overflowing_add(self.reg.h);
		self.reg.a = res;
		let zero = self.reg.a == 0;
		self.reg.set_flags(zero, false, half, _carry);
		(1, 4)
	}

	// A + L -> A
	fn op_85(&mut self) -> (u16, u64) {
		let half = (self.reg.a & 0xF) + (self.reg.l & 0xF) > 0xF;
		let (res, _carry) = self.reg.a.overflowing_add(self.reg.l);
		self.reg.a = res;
		let zero = self.reg.a == 0;
		self.reg.set_flags(zero, false, half, _carry);
		(1, 4)
	}

	// A + A -> A
	fn op_87(&mut self) -> (u16, u64) {
		let half = (self.reg.a & 0xF) + (self.reg.a & 0xF) > 0xF;
		let (res, _carry) = self.reg.a.overflowing_add(self.reg.a);
		self.reg.a = res;
		let zero = self.reg.a == 0;
		self.reg.set_flags(zero, false, half, _carry);
		(1, 4)
	}

}

mod test {
	#[test]
	fn test_stack() {
		let mut testcore = super::Core::new();
		testcore.push(0xFC55);
		assert_eq!(testcore.pop(), 0xFC55);

		testcore.push(0x1234);
		testcore.push(0x4F32);
		assert_eq!(testcore.pop(), 0x4F32);
		assert_eq!(testcore.pop(), 0x1234);

		testcore.push(0xABCD);
		testcore.push(0xDFD4);
		testcore.push(0x8642);
		assert_eq!(testcore.pop(), 0x8642);
		assert_eq!(testcore.pop(), 0xDFD4);
		assert_eq!(testcore.pop(), 0xABCD);
	}

	#[test]
	fn test_inc_reg() {
		let mut testcore = super::Core::new();
		testcore.inc_reg(super::registers::RegisterName::l);
		assert_eq!(testcore.reg.l, 0x4E);
	}

	#[test]
	fn test_dec_reg() {
		let mut testcore = super::Core::new();
		testcore.dec_reg(super::registers::RegisterName::l);
		assert_eq!(testcore.reg.l, 0x4C);
	}

	#[test]
	fn test_sub_half_carry() {
		use super::check_sub_half_carry;
		assert_eq!(check_sub_half_carry(0b00000000, 0x00000000), false);
		assert_eq!(check_sub_half_carry(0b10100110, 0x00000000), false);
		assert_eq!(check_sub_half_carry(0b10101100, 0b00000001), false);
		assert_eq!(check_sub_half_carry(0b10000000, 0b00000001), true);
		assert_eq!(check_sub_half_carry(0b10010000, 0b00000001), true);
	}
}