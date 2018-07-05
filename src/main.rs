extern crate time;
use time::PreciseTime;

mod core;

use core::Core as gbcore;

fn main() {
    let mut test_core = gbcore::new();

    test_core.mem.rom.load_file("../pyGBE/ROMS/tetris.gb".to_string());

    let start = PreciseTime::now();
    while test_core.reg.pc != 0x02D6{
    	test_core.step();
    }
    let end = PreciseTime::now();
    println!("{} seconds.", start.to(end));
}
