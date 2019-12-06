#![allow(dead_code)]

use crate::rom::Rom;

pub struct Cpu {
    pc: u16,
    sp: u8,
    accumulator: u8,
    x: u8,
    y: u8,
    carry: bool,
    zero: bool,
    interrupt: bool,
    decimal: bool,
    brk: bool,
    overflow: bool,
    sign: bool,
    rom: Rom,
}

#[derive(Debug)]
enum AddressingMode {
    ZeroPage(u8), ZeroPageX(u8), ZeroPageY(u8),
    Absolute(u16), AbsoluteX(u16), AbsoluteY(u16),
    Indirect(u16), IndirectX(u8), IndirectY(u8),
    Implicit,
    Immediate(u8),
    Relative(i8),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Opcode {
    Add, And, Asl, Bcc, Bcs, Beq, Bit, Bmi, Bne, Bpl, Brk, Bvc, Bvs, Clc, Cld, Cli, Clv, Cmp, Cpx, Cpy, Dec, Dex, Dey, Eor, Inc, Inx, Iny, Jmp, Jsr, Lda, Ldx, Ldy, Lsr, Nop, Ora, Pha, Php, Pla, Plp, Rol, Ror, Rti, Rts, Sbc, Sec, Sed, Sei, Sta, Stx, Sty, Tax, Tay, Tsx, Txa, Txs, Ta,
}

#[derive(Debug)]
pub struct Instruction {
    pub opcode: Opcode,
    mode: AddressingMode,
    cycles: u8,
    page_cross_cost: bool,
}

impl Cpu {
    pub fn new(rom: Rom) -> Self {
        Cpu {
            pc: 0x0,
            sp: 0x0,
            accumulator: 0x0,
            x: 0x0,
            y: 0x0,
            carry: false,
            zero: false,
            interrupt: false,
            decimal: false,
            brk: false,
            overflow: false,
            sign: false,
            rom,
        }
    }

    pub fn get_next_instruction(&self) -> Instruction {
        let opcode: u8 = self.get_byte_at(AddressingMode::Absolute(self.pc));
        let byte_after_opcode: u8 = self.get_byte_at(AddressingMode::Absolute(self.pc + 1));
        let signed_byte_after_opcode: i8 = byte_after_opcode as i8;
        let word_after_opcode: u16 = self.get_word_at(AddressingMode::Absolute(self.pc + 1));

        match opcode {
            /* Add */
            0x69 => Instruction {
                opcode: Opcode::Add,
                mode: AddressingMode::Immediate(byte_after_opcode),
                cycles: 2,
                page_cross_cost: false,
            },
            0x65 => Instruction {
                opcode: Opcode::Add,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 3,
                page_cross_cost: false,
            },
            0x75 => Instruction {
                opcode: Opcode::Add,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0x60 => Instruction {
                opcode: Opcode::Add,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0x70 => Instruction {
                opcode: Opcode::Add,
                mode: AddressingMode::AbsoluteX(word_after_opcode),
                cycles: 4,
                page_cross_cost: true,
            },
            0x79 => Instruction {
                opcode: Opcode::Add,
                mode: AddressingMode::AbsoluteY(word_after_opcode),
                cycles: 4,
                page_cross_cost: true,
            },
            0x61 => Instruction {
                opcode: Opcode::Add,
                mode: AddressingMode::IndirectX(byte_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0x71 => Instruction {
                opcode: Opcode::Add,
                mode: AddressingMode::IndirectY(byte_after_opcode),
                cycles: 5,
                page_cross_cost: true,
            },
            /* And */
            0x29 => Instruction {
                opcode: Opcode::And,
                mode: AddressingMode::Immediate(byte_after_opcode),
                cycles: 2,
                page_cross_cost: false,
            },
            0x25 => Instruction {
                opcode: Opcode::And,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 3,
                page_cross_cost: false,
            },
            0x35 => Instruction {
                opcode: Opcode::And,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0x2D => Instruction {
                opcode: Opcode::And,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0x3D => Instruction {
                opcode: Opcode::And,
                mode: AddressingMode::AbsoluteX(word_after_opcode),
                cycles: 4,
                page_cross_cost: true,
            },
            0x39 => Instruction {
                opcode: Opcode::And,
                mode: AddressingMode::AbsoluteY(word_after_opcode),
                cycles: 4,
                page_cross_cost: true,
            },
            0x21 => Instruction {
                opcode: Opcode::And,
                mode: AddressingMode::IndirectX(byte_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0x31 => Instruction {
                opcode: Opcode::And,
                mode: AddressingMode::IndirectY(byte_after_opcode),
                cycles: 5,
                page_cross_cost: false,
            },
            /* Asl */
            0x0A => Instruction {
                opcode: Opcode::Asl,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            0x06 => Instruction {
                opcode: Opcode::Asl,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 5,
                page_cross_cost: false,
            },
            0x16 => Instruction {
                opcode: Opcode::Asl,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0x0E => Instruction {
                opcode: Opcode::Asl,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0x1E => Instruction {
                opcode: Opcode::Asl,
                mode: AddressingMode::AbsoluteX(word_after_opcode),
                cycles: 7,
                page_cross_cost: false,
            },
            /* Bcc */
            0x90 => Instruction {
                opcode: Opcode::Bcc,
                mode: AddressingMode::Relative(signed_byte_after_opcode),
                cycles: 2,
                page_cross_cost: true,
            },
            /* Bcs */
            0xB0 => Instruction {
                opcode: Opcode::Bcs,
                mode: AddressingMode::Relative(signed_byte_after_opcode),
                cycles: 2,
                page_cross_cost: true,
            },
            /* Beq */
            0xF0 => Instruction {
                opcode: Opcode::Beq,
                mode: AddressingMode::Relative(signed_byte_after_opcode),
                cycles: 2,
                page_cross_cost: true,
            },
            /* Bit */
            0x24 => Instruction {
                opcode: Opcode::Bit,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 3,
                page_cross_cost: false,
            },
            0x2C => Instruction {
                opcode: Opcode::Bit,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            /* Bmi */
            0x30 => Instruction {
                opcode: Opcode::Bmi,
                mode: AddressingMode::Relative(signed_byte_after_opcode),
                cycles: 2,
                page_cross_cost: false, /* Is this right? */
            },
            /* Bne */
            0xD0 => Instruction {
                opcode: Opcode::Bne,
                mode: AddressingMode::Relative(signed_byte_after_opcode),
                cycles: 2,
                page_cross_cost: true,
            },
            /* Bpl */
            0x10 => Instruction {
                opcode: Opcode::Bpl,
                mode: AddressingMode::Relative(signed_byte_after_opcode),
                cycles: 2,
                page_cross_cost: true,
            },
            /* Brk */
            0x00 => Instruction {
                opcode: Opcode::Brk,
                mode: AddressingMode::Implicit,
                cycles: 7,
                page_cross_cost: false,
            },
            /* Bvc */
            0x50 => Instruction {
                opcode: Opcode::Bvc,
                mode: AddressingMode::Relative(signed_byte_after_opcode),
                cycles: 2,
                page_cross_cost: true,
            },
            /* Bvs */
            0x70 => Instruction {
                opcode: Opcode::Bvs,
                mode: AddressingMode::Relative(signed_byte_after_opcode),
                cycles: 2,
                page_cross_cost: true,
            },
            /* Clc */
            0x18 => Instruction {
                opcode: Opcode::Clc,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            _ => Instruction {
                opcode: Opcode::Nop,
                mode: AddressingMode::Implicit,
                cycles: 0,
                page_cross_cost: false,
            },
        }
    }

    pub fn execute_instruction(&mut self, instruction: Instruction) {
        let instruction_length = 1 + match instruction.mode {
            AddressingMode::ZeroPage(_) => 1,
            AddressingMode::ZeroPageX(_) => 1,
            AddressingMode::ZeroPageY(_) => 1,
            AddressingMode::IndirectX(_) => 1,
            AddressingMode::IndirectY(_) => 1,
            AddressingMode::Immediate(_) => 1,
            AddressingMode::Relative(_) => 1,
            AddressingMode::Absolute(_) => 2,
            AddressingMode::AbsoluteX(_) => 2,
            AddressingMode::AbsoluteY(_) => 2,
            AddressingMode::Indirect(_) => 2,
            AddressingMode::Implicit => 0,
        };

        self.pc += instruction_length;
    }

    fn get_byte_at(&self, mode: AddressingMode) -> u8 {
        match mode {
            AddressingMode::Absolute(addr) => self.rom.prg_rom[usize::from(addr)],
            _ => 0x0,
        }
    }

    fn get_word_at(&self, mode: AddressingMode) -> u16 {
        match mode {
            AddressingMode::Absolute(addr) => {
                let low = self.rom.prg_rom[usize::from(addr)];
                let high = self.rom.prg_rom[usize::from(addr + 1)];
                ((high as u16) << 8) | (low as u16)
            },
            _ => 0x0,
        }
    }

    fn set_byte_at(&mut self, mode: AddressingMode, val: u8) { }

    fn set_word_at(&mut self, mode: AddressingMode, val: u16) { }
}
