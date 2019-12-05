mod rom;

use rom::Rom;

fn main() {
    let rom = Rom::new("roms/mario.nes").unwrap();
    rom.dump_opcodes();

    println!("Hello, world!");
}
