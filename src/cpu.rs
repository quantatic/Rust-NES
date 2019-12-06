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
    Add, And, Asl, Bcc, Bcs, Beq, Bit, Bmi, Bne, Bpl, Brk, Bvc, Bvs, Clc, Cld, Cli, Clv, Cmp, Cpx, Cpy, Dec, Dex, Dey, Eor, Inc, Inx, Iny, Jmp, Jsr, Lda, Ldx, Ldy, Lsr, Nop, Ora, Pha, Php, Pla, Plp, Rol, Ror, Rti, Rts, Sbc, Sec, Sed, Sei, Sta, Stx, Sty, Tax, Tay, Tsx, Txa, Txs, Tya,
}

#[derive(Debug)]
pub struct Instruction {
    opcode: Opcode,
    mode: AddressingMode,
    pub cycles: u8,
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

    pub fn fetch_next_instruction(&mut self) -> Instruction {
        let opcode: u8 = self.get_byte_at(AddressingMode::Absolute(self.pc));
        let byte_after_opcode: u8 = self.get_byte_at(AddressingMode::Absolute(self.pc + 1));
        let signed_byte_after_opcode: i8 = byte_after_opcode as i8;
        let word_after_opcode: u16 = self.get_word_at(AddressingMode::Absolute(self.pc + 1));

        println!("Opcode: 0x{:02x}", opcode);

        let result = match opcode {
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
            0x6D => Instruction {
                opcode: Opcode::Add,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0x7D => Instruction {
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
            /* Cld */
            0xD8 => Instruction {
                opcode: Opcode::Cld,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            /* Cli */
            0x58 => Instruction {
                opcode: Opcode::Cli,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            /* Clv */
            0xB8 => Instruction {
                opcode: Opcode::Clv,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            /* Cmp */
            0xC9 => Instruction {
                opcode: Opcode::Cmp,
                mode: AddressingMode::Immediate(byte_after_opcode),
                cycles: 2,
                page_cross_cost: false,
            },
            0xC5 => Instruction {
                opcode: Opcode::Cmp,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 2,
                page_cross_cost: false,
            },
            0xD5 => Instruction {
                opcode: Opcode::Cmp,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 2,
                page_cross_cost: false,
            },
            0xCD => Instruction {
                opcode: Opcode::Cmp,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0xDD => Instruction {
                opcode: Opcode::Cmp,
                mode: AddressingMode::AbsoluteX(word_after_opcode),
                cycles: 4,
                page_cross_cost: true,
            },
            0xD9 => Instruction {
                opcode: Opcode::Cmp,
                mode: AddressingMode::AbsoluteY(word_after_opcode),
                cycles: 4,
                page_cross_cost: true,
            },
            0xC1 => Instruction {
                opcode: Opcode::Cmp,
                mode: AddressingMode::IndirectX(byte_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0xD1 => Instruction {
                opcode: Opcode::Cmp,
                mode: AddressingMode::IndirectY(byte_after_opcode),
                cycles: 5,
                page_cross_cost: true,
            },
            /* Cpx */
            0xE0 => Instruction {
                opcode: Opcode::Cpx,
                mode: AddressingMode::Immediate(byte_after_opcode),
                cycles: 2,
                page_cross_cost: false,
            },
            0xE4 => Instruction {
                opcode: Opcode::Cpx,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 3,
                page_cross_cost: false,
            },
            0xEC => Instruction {
                opcode: Opcode::Cpx,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            /* Cpy */
            0xC0 => Instruction {
                opcode: Opcode::Cpy,
                mode: AddressingMode::Immediate(byte_after_opcode),
                cycles: 2,
                page_cross_cost: false,
            },
            0xC4 => Instruction {
                opcode: Opcode::Cpy,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 3,
                page_cross_cost: false,
            },
            0xCC => Instruction {
                opcode: Opcode::Cpy,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            /* Dec */
            0xC6 => Instruction {
                opcode: Opcode::Dec,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 5,
                page_cross_cost: false,
            },
            0xD6 => Instruction {
                opcode: Opcode::Dec,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0xCE => Instruction {
                opcode: Opcode::Dec,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0xDE => Instruction {
                opcode: Opcode::Dec,
                mode: AddressingMode::AbsoluteX(word_after_opcode),
                cycles: 7,
                page_cross_cost: false,
            },
            /* Dex */
            0xCA => Instruction {
                opcode: Opcode::Dex,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            /* Dey */
            0x88 => Instruction {
                opcode: Opcode::Dey,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            /* Eor */
            0x49 => Instruction {
                opcode: Opcode::Eor,
                mode: AddressingMode::Immediate(byte_after_opcode),
                cycles: 2,
                page_cross_cost: false,
            },
            0x45 => Instruction {
                opcode: Opcode::Eor,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 3,
                page_cross_cost: false,
            },
            0x55 => Instruction {
                opcode: Opcode::Eor,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0x4D => Instruction {
                opcode: Opcode::Eor,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0x5D => Instruction {
                opcode: Opcode::Eor,
                mode: AddressingMode::AbsoluteX(word_after_opcode),
                cycles: 4,
                page_cross_cost: true,
            },
            0x59 => Instruction {
                opcode: Opcode::Eor,
                mode: AddressingMode::AbsoluteY(word_after_opcode),
                cycles: 5,
                page_cross_cost: true,
            },
            0x41 => Instruction {
                opcode: Opcode::Eor,
                mode: AddressingMode::IndirectX(byte_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0x51 => Instruction {
                opcode: Opcode::Eor,
                mode: AddressingMode::IndirectY(byte_after_opcode),
                cycles: 5,
                page_cross_cost: true,
            },
            /* Inc */
            0xE6 => Instruction {
                opcode: Opcode::Inc,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 5,
                page_cross_cost: false,
            },
            0xF6 => Instruction {
                opcode: Opcode::Inc,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0xEE => Instruction {
                opcode: Opcode::Inc,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0xFE => Instruction {
                opcode: Opcode::Inc,
                mode: AddressingMode::AbsoluteX(word_after_opcode),
                cycles: 7,
                page_cross_cost: false,
            },
            /* Inx */
            0xE8 => Instruction {
                opcode: Opcode::Inx,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            /* Iny */
            0xC8 => Instruction {
                opcode: Opcode::Iny,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            /* Jmp */
            0x4C => Instruction {
                opcode: Opcode::Jmp,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 3,
                page_cross_cost: false,
            },
            0x6C => Instruction {
                opcode: Opcode::Jmp,
                mode: AddressingMode::Indirect(word_after_opcode),
                cycles: 5,
                page_cross_cost: false,
            },
            /* Jsr */
            0x20 => Instruction {
                opcode: Opcode::Jsr,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            /* Lda */
            0xA9 => Instruction {
                opcode: Opcode::Lda,
                mode: AddressingMode::Immediate(byte_after_opcode),
                cycles: 2,
                page_cross_cost: false,
            },
            0xA5 => Instruction {
                opcode: Opcode::Lda,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 3,
                page_cross_cost: false,
            },
            0xB5 => Instruction {
                opcode: Opcode::Lda,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0xAD => Instruction {
                opcode: Opcode::Lda,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0xBD => Instruction {
                opcode: Opcode::Lda,
                mode: AddressingMode::AbsoluteX(word_after_opcode),
                cycles: 4,
                page_cross_cost: true,
            },
            0xB9 => Instruction {
                opcode: Opcode::Lda,
                mode: AddressingMode::AbsoluteY(word_after_opcode),
                cycles: 4,
                page_cross_cost: true,
            },
            0xA1 => Instruction {
                opcode: Opcode::Lda,
                mode: AddressingMode::IndirectX(byte_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0xB1 => Instruction {
                opcode: Opcode::Lda,
                mode: AddressingMode::IndirectY(byte_after_opcode),
                cycles: 5,
                page_cross_cost: true,
            },
            /* Ldx */
            0xA2 => Instruction {
                opcode: Opcode::Ldx,
                mode: AddressingMode::Immediate(byte_after_opcode),
                cycles: 2,
                page_cross_cost: false,
            },
            0xA6 => Instruction {
                opcode: Opcode::Ldx,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 3,
                page_cross_cost: false,
            },
            0xB6 => Instruction {
                opcode: Opcode::Ldx,
                mode: AddressingMode::ZeroPageY(byte_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0xAE => Instruction {
                opcode: Opcode::Ldx,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0xBE => Instruction {
                opcode: Opcode::Ldx,
                mode: AddressingMode::AbsoluteY(word_after_opcode),
                cycles: 4,
                page_cross_cost: true,
            },
            /* Ldy */
            0xA0 => Instruction {
                opcode: Opcode::Ldy,
                mode: AddressingMode::Immediate(byte_after_opcode),
                cycles: 2,
                page_cross_cost: false,
            },
            0xA4 => Instruction {
                opcode: Opcode::Ldy,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 3,
                page_cross_cost: false,
            },
            0xB4 => Instruction {
                opcode: Opcode::Ldy,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0xAC => Instruction {
                opcode: Opcode::Ldy,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0xBC => Instruction {
                opcode: Opcode::Ldy,
                mode: AddressingMode::AbsoluteX(word_after_opcode),
                cycles: 4,
                page_cross_cost: true,
            },
            /* Lsr */
            0x4A => Instruction {
                opcode: Opcode::Lsr,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            0x46 => Instruction {
                opcode: Opcode::Lsr,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 5,
                page_cross_cost: false,
            },
            0x56 => Instruction {
                opcode: Opcode::Lsr,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0x4E => Instruction {
                opcode: Opcode::Lsr,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0x5E => Instruction {
                opcode: Opcode::Lsr,
                mode: AddressingMode::AbsoluteX(word_after_opcode),
                cycles: 7,
                page_cross_cost: false,
            },
            /* Nop */
            0xEA => Instruction {
                opcode: Opcode::Nop,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            /* Ora */
            0x09 => Instruction {
                opcode: Opcode::Ora,
                mode: AddressingMode::Immediate(byte_after_opcode),
                cycles: 2,
                page_cross_cost: false,
            },
            0x05 => Instruction {
                opcode: Opcode::Ora,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 3,
                page_cross_cost: false,
            },
            0x15 => Instruction {
                opcode: Opcode::Ora,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0x0D => Instruction {
                opcode: Opcode::Ora,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0x1D => Instruction {
                opcode: Opcode::Ora,
                mode: AddressingMode::AbsoluteX(word_after_opcode),
                cycles: 4,
                page_cross_cost: true,
            },
            0x19 => Instruction {
                opcode: Opcode::Ora,
                mode: AddressingMode::AbsoluteY(word_after_opcode),
                cycles: 2,
                page_cross_cost: true,
            },
            0x01 => Instruction {
                opcode: Opcode::Ora,
                mode: AddressingMode::IndirectX(byte_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0x11 => Instruction {
                opcode: Opcode::Ora,
                mode: AddressingMode::IndirectY(byte_after_opcode),
                cycles: 5,
                page_cross_cost: false,
            },
            /* Pha */
            0x48 => Instruction {
                opcode: Opcode::Pha,
                mode: AddressingMode::Implicit,
                cycles: 3,
                page_cross_cost: false,
            },
            /* Php */
            0x08 => Instruction {
                opcode: Opcode::Php,
                mode: AddressingMode::Implicit,
                cycles: 3,
                page_cross_cost: false,
            },
            /* Pla */
            0x68 => Instruction {
                opcode: Opcode::Pla,
                mode: AddressingMode::Implicit,
                cycles: 4,
                page_cross_cost: false,
            },
            /* Plp */
            0x28 => Instruction {
                opcode: Opcode::Plp,
                mode: AddressingMode::Implicit,
                cycles: 4,
                page_cross_cost: false,
            },
            /* Rol */
            0x2A => Instruction {
                opcode: Opcode::Rol,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            0x26 => Instruction {
                opcode: Opcode::Rol,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 5,
                page_cross_cost: false,
            },
            0x36 => Instruction {
                opcode: Opcode::Rol,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0x2E => Instruction {
                opcode: Opcode::Rol,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0x3E => Instruction {
                opcode: Opcode::Rol,
                mode: AddressingMode::AbsoluteX(word_after_opcode),
                cycles: 7,
                page_cross_cost: false,
            },
            /* Ror */
            0x6A => Instruction {
                opcode: Opcode::Ror,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            0x66 => Instruction {
                opcode: Opcode::Ror,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 5,
                page_cross_cost: false,
            },
            0x76 => Instruction {
                opcode: Opcode::Ror,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0x6E => Instruction {
                opcode: Opcode::Ror,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0x7E => Instruction {
                opcode: Opcode::Ror,
                mode: AddressingMode::AbsoluteX(word_after_opcode),
                cycles: 7,
                page_cross_cost: false,
            },
            /* Rti */
            0x40 => Instruction {
                opcode: Opcode::Rti,
                mode: AddressingMode::Implicit,
                cycles: 6,
                page_cross_cost: false,
            },
            /* Rts */
            0x60 => Instruction {
                opcode: Opcode::Rts,
                mode: AddressingMode::Implicit,
                cycles: 6,
                page_cross_cost: false,
            },
            /* Sbc */
            0xE9 => Instruction {
                opcode: Opcode::Sbc,
                mode: AddressingMode::Immediate(byte_after_opcode),
                cycles: 2,
                page_cross_cost: false,
            },
            0xE5 => Instruction {
                opcode: Opcode::Sbc,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 3,
                page_cross_cost: false,
            },
            0xF5 => Instruction {
                opcode: Opcode::Sbc,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0xED => Instruction {
                opcode: Opcode::Sbc,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0xFD => Instruction {
                opcode: Opcode::Sbc,
                mode: AddressingMode::AbsoluteX(word_after_opcode),
                cycles: 4,
                page_cross_cost: true,
            },
            0xF9 => Instruction {
                opcode: Opcode::Sbc,
                mode: AddressingMode::AbsoluteY(word_after_opcode),
                cycles: 4,
                page_cross_cost: true,
            },
            0xE1 => Instruction {
                opcode: Opcode::Sbc,
                mode: AddressingMode::IndirectX(byte_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0xF1 => Instruction {
                opcode: Opcode::Sbc,
                mode: AddressingMode::IndirectY(byte_after_opcode),
                cycles: 5,
                page_cross_cost: false,
            },
            /* Sec */
            0x38 => Instruction {
                opcode: Opcode::Sec,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            /* Sed */
            0xF8 => Instruction {
                opcode: Opcode::Sed,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            /* Sei */
            0x78 => Instruction {
                opcode: Opcode::Sei,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            /* Sta */
            0x85 => Instruction {
                opcode: Opcode::Sta,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 3,
                page_cross_cost: false,
            },
            0x95 => Instruction {
                opcode: Opcode::Sta,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0x8D => Instruction {
                opcode: Opcode::Sta,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0x9D => Instruction {
                opcode: Opcode::Sta,
                mode: AddressingMode::AbsoluteX(word_after_opcode),
                cycles: 5,
                page_cross_cost: false,
            },
            0x99 => Instruction {
                opcode: Opcode::Sta,
                mode: AddressingMode::AbsoluteY(word_after_opcode),
                cycles: 5,
                page_cross_cost: false,
            },
            0x81 => Instruction {
                opcode: Opcode::Sta,
                mode: AddressingMode::IndirectX(byte_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0x91 => Instruction {
                opcode: Opcode::Sta,
                mode: AddressingMode::IndirectY(byte_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            /* Stx */
            0x86 => Instruction {
                opcode: Opcode::Stx,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 3,
                page_cross_cost: false,
            },
            0x96 => Instruction {
                opcode: Opcode::Stx,
                mode: AddressingMode::ZeroPageY(byte_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0x8E => Instruction {
                opcode: Opcode::Stx,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            /* Sty */
            0x84 => Instruction {
                opcode: Opcode::Sty,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 3,
                page_cross_cost: false,
            },
            0x94 => Instruction {
                opcode: Opcode::Sty,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0x8C => Instruction {
                opcode: Opcode::Sty,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            /* Tax */
            0xAA => Instruction {
                opcode: Opcode::Tax,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            /* Tay */
            0xA8 => Instruction {
                opcode: Opcode::Tay,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            /* Tsx */
            0xBA => Instruction {
                opcode: Opcode::Tsx,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            /* Txa */
            0x8A => Instruction {
                opcode: Opcode::Txa,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            /* Txs */
            0x9A => Instruction {
                opcode: Opcode::Txs,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            /* Tya */
            0x98 => Instruction {
                opcode: Opcode::Tya,
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
        };

        let instruction_length = 1 + match result.mode {
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
        result
    }

    pub fn execute_instruction(&mut self, instruction: Instruction) { }

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
