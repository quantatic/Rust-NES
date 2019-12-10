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
        let processor_status: u8 = ((cpu.sign as u8) << 7)
            | ((cpu.overflow as u8) << 6)
            | ((1 as u8) << 5)
            | ((0 as u8) << 4)
            | ((cpu.decimal as u8) << 3)
            | ((cpu.interrupt as u8) << 2)
            | ((cpu.zero as u8) << 1)
            | ((cpu.carry as u8) << 0);
        let a = cpu.accumulator;
        let x = cpu.x;
        let y = cpu.y;
        let sp = cpu.sp;
        let next_instruction = cpu.fetch_next_instruction();
        println!("{:04X} -> {:X?}\t{:X?}\tA:{:02X}\tX:{:02X}\tY:{:02X}\tP:{:02X}\tSP:{:02X}\t02h:{:02X}\t03h:{:02X}", pc, next_instruction.opcode, next_instruction.mode, a, x, y, processor_status, sp, cpu.memory.get_byte_at(0x02), cpu.memory.get_byte_at(0x03));
        cpu.execute_instruction(next_instruction);
    }
}
