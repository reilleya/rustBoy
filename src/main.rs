mod core;

use core::Core as gbcore;

fn main() {
    println!("Hello, world!");

    let mut test_core = gbcore::new();

    for i in (1..4) {
    	test_core.step();
    }
}
