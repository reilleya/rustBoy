use std::fs::File;
use std::io::Read;

pub struct ROM {
	pub data: [u8; 0x8000],
	pub r_type: u8
}

impl ROM {
	pub fn create_rom() -> ROM {
		ROM{
			data: [0; 0x8000],
			r_type: 0
		}
	}

	pub fn load_file(&mut self, filename:String) {
		let mut fo: File = match File::open(filename.clone()) {
			Ok(file) => file,
            Err(_) => panic!("Can't read {}", filename)
		};
    	fo.read(&mut self.data);
	}

	pub fn get_mem(&self, loc:u16) -> u8 {
		self.data[loc as usize]
	}
}