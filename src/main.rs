mod rom;
mod cpu;

use crate::rom::Rom;
use crate::cpu::Cpu;

fn main() {
    let rom = Rom::new("roms/mario.nes").unwrap();
    let mut cpu = Cpu::new(rom);

    loop {
        let next_instruction = cpu.fetch_next_instruction();
        println!("{:x?}", next_instruction);
        cpu.execute_instruction(next_instruction);
    }
}
