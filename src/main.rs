mod cpu;
mod memory;
mod ppu;
mod rom;

use crate::cpu::{Cpu, Interrupt};
use crate::memory::Memory;
use crate::ppu::Ppu;
use crate::rom::Rom;

fn main() {
    //let rom = Rom::new("roms/donkey.nes").unwrap();
    let rom = Rom::new("nestest.nes").unwrap();
    let ppu = Ppu::new();

    let mut memory = Memory::new(rom, ppu);
    let mut cpu = Cpu::new(&mut memory);
    cpu.interrupt(Interrupt::Reset);
    cpu.pc = 0x0C000;

    loop {
        let pc = cpu.pc;
        let next_instruction = cpu.fetch_next_instruction();
        println!("{:04X} -> {:X?}\t{:X?}\tA:{:02X}\tX:{:02X}\tY:{:02X}\tSP:{:02X}", pc, next_instruction.opcode, next_instruction.mode, cpu.accumulator, cpu.x, cpu.y, cpu.sp);
        cpu.execute_instruction(next_instruction);
    }
}
