mod cpu;
mod memory;
mod rom;

use crate::cpu::Cpu;
use crate::memory::Memory;
use crate::rom::Rom;

fn main() {
    let rom = Rom::new("roms/donkey.nes").unwrap();
    println!("{:x?}", &rom.prg_rom[..100]);
    let mut memory = Memory::new(rom);
    let mut cpu = Cpu::new(&mut memory);
    cpu.reset();

    for _ in 0..100 {
        let next_instruction = cpu.fetch_next_instruction();
        println!("0x{:05x}: {:x?}", cpu.pc, next_instruction);
        cpu.execute_instruction(next_instruction);
    }
}
