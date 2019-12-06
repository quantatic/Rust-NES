mod rom;
mod cpu;

use crate::rom::Rom;
use crate::cpu::{Cpu, Opcode};

fn main() {
    let rom = Rom::new("roms/mario.nes").unwrap();
    let mut cpu = Cpu::new(rom);

    loop {
        let next_instruction = cpu.get_next_instruction();
        if next_instruction.opcode != Opcode::Nop {
            println!("{:x?}", next_instruction);
        }
        cpu.execute_instruction(next_instruction);
    }
}
