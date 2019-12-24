use crate::bus::Bus;

pub struct Cpu<'a> {
    pub pc: u16,
    pub sp: u8,
    pub accumulator: u8,
    pub x: u8,
    pub y: u8,
    pub cycles_left: u8,
    pub cycles_completed: u64,
    pub carry: bool,
    pub zero: bool,
    pub interrupt: bool,
    pub decimal: bool,
    pub overflow: bool,
    pub sign: bool,
    pub bus: &'a mut Bus,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Opcode {
    Add, And, Asl, Bcc, Bcs, Beq, Bit, Bmi, Bne, Bpl, Brk, Bvc, Bvs, Clc, Cld, Cli, Clv, Cmp, Cpx, Cpy, Dcp, Dec, Dex, Dey, Eor, Inc, Inx, Iny, Isc, Jmp, Jsr, Lax, Lda, Ldx, Ldy, Lsr, Nop, Ora, Pha, Php, Pla, Plp, Rla, Rol, Ror, Rra, Rti, Rts, Sax, Sbc, Sec, Sed, Sei, Slo, Sre, Sta, Stx, Sty, Tax, Tay, Tsx, Txa, Txs, Tya,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum AddressingMode {
    ZeroPage(u8), ZeroPageX(u8), ZeroPageY(u8),
    Absolute(u16), AbsoluteX(u16), AbsoluteY(u16),
    Indirect(u16), IndirectX(u8), IndirectY(u8),
    Implicit,
    Immediate(u8),
    Relative(i8),
    Accumulator,
}

#[derive(Copy, Clone, Debug)]
pub struct Instruction {
    pub opcode: Opcode,
    pub mode: AddressingMode,
    pub cycles: u8,
    pub page_cross_cost: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum Interrupt {
    Irq,    // maskable interrupt
    Nmi,    // non-maskable interrupt
    Reset   // reset interrupt
}

impl<'a> Cpu<'a> {
    pub fn new(bus: &'a mut Bus) -> Self {
        Cpu {
            pc: 0x0,
            sp: 0x0,
            accumulator: 0x0,
            x: 0x0,
            y: 0x0,
            cycles_left: 0x0,
            cycles_completed: 0x0,
            carry: false,
            zero: false,
            interrupt: false,
            decimal: false,
            overflow: false,
            sign: false,
            bus,
        }
    }

    pub fn step(&mut self) {
        if self.cycles_left > 0 {
            self.cycles_left -= 1;
            self.cycles_completed += 1;
            return;
        }

        let pc = self.pc;
        let processor_status: u8 = ((self.sign as u8) << 7)
            | ((self.overflow as u8) << 6)
            | ((1 as u8) << 5)
            | ((0 as u8) << 4)
            | ((self.decimal as u8) << 3)
            | ((self.interrupt as u8) << 2)
            | ((self.zero as u8) << 1)
            | ((self.carry as u8) << 0);
        let a = self.accumulator;
        let x = self.x;
        let y = self.y;
        let sp = self.sp;
        if self.bus.ppu.nmi_waiting {
            self.bus.ppu.nmi_waiting = false;
            self.interrupt(Interrupt::Nmi);
        } else {
            let next_instruction = self.fetch_next_instruction();
            //println!("{:04X}  A:{:02X}  X:{:02X}  Y:{:02X}  P:{:02X}  SP:{:02X}", pc, a, x, y, processor_status, sp);
            //println!("{:04X} -> {:X?}\tA:{:02X}\tX:{:02X}\tY:{:02X}\tP:{:02X}\tSP:{:02X}\tCYC:{}\tV:0x{:04X}", pc, next_instruction, a, x, y, processor_status, sp, self.cycles_completed, self.bus.ppu.ppuaddr);
            self.execute_instruction(next_instruction);
            self.cycles_left += next_instruction.cycles;
        }

        self.cycles_left -= 1;
        self.cycles_completed += 1;
    }

    fn push_byte(&mut self, val: u8) {
        self.bus.set_byte_at(0x100 + u16::from(self.sp), val);
        self.sp = self.sp.wrapping_sub(1);
    }

    fn push_word(&mut self, val: u16) {
        let low_byte = (val & 0xFF) as u8;
        let high_byte = ((val >> 8) & 0xFF) as u8;

        self.push_byte(high_byte);
        self.push_byte(low_byte);
    }

    fn pop_byte(&mut self) -> u8 {
        self.sp = self.sp.wrapping_add(1);
        return self.bus.get_byte_at(0x100 + u16::from(self.sp));
    }

    fn pop_word(&mut self) -> u16 {
        let low_byte = self.pop_byte();
        let high_byte = self.pop_byte();

        return ((high_byte as u16) << 8) | (low_byte as u16);
    }

    pub fn interrupt(&mut self, int_type: Interrupt) {
        self.push_word(self.pc);

        // Push processor flags to stack
        self.php();

        self.interrupt = true;
        self.pc = match int_type {
            Interrupt::Irq => self.bus.get_word_at(0xFFFE),
            Interrupt::Nmi => self.bus.get_word_at(0xFFFA),
            Interrupt::Reset => self.bus.get_word_at(0xFFFC),
        };

        self.cycles_left = 7;
    }

    pub fn fetch_next_instruction(&mut self) -> Instruction {
        let opcode: u8 = self.bus.get_byte_at(self.pc);
        let byte_after_opcode: u8 = self.bus.get_byte_at(self.pc + 1);
        let signed_byte_after_opcode: i8 = byte_after_opcode as i8;
        let word_after_opcode: u16 = self.bus.get_word_at(self.pc + 1);
        //println!("Opcode: 0x{:02x}", opcode);
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
                mode: AddressingMode::Accumulator,
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
                cycles: 0, // this generates an interrupt which adds the right number of cycles itself
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
            /* Dcp */
            0xC3 => Instruction {
                opcode: Opcode::Dcp,
                mode: AddressingMode::IndirectX(byte_after_opcode),
                cycles: 8,
                page_cross_cost: false,
            },
            0xC7 => Instruction {
                opcode: Opcode::Dcp,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 5,
                page_cross_cost: false,
            },
            0xCF => Instruction {
                opcode: Opcode::Dcp,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0xD3 => Instruction {
                opcode: Opcode::Dcp,
                mode: AddressingMode::IndirectY(byte_after_opcode),
                cycles: 8,
                page_cross_cost: false,
            },
            0xD7 => Instruction {
                opcode: Opcode::Dcp,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0xDB => Instruction {
                opcode: Opcode::Dcp,
                mode: AddressingMode::AbsoluteY(word_after_opcode),
                cycles: 7,
                page_cross_cost: false,
            },
            0xDF => Instruction {
                opcode: Opcode::Dcp,
                mode: AddressingMode::AbsoluteX(word_after_opcode),
                cycles: 7,
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
            /* Isc */
            0xE3 => Instruction {
                opcode: Opcode::Isc,
                mode: AddressingMode::IndirectX(byte_after_opcode),
                cycles: 8,
                page_cross_cost: false,
            },
            0xE7 => Instruction {
                opcode: Opcode::Isc,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 5,
                page_cross_cost: false,
            },
            0xEF => Instruction {
                opcode: Opcode::Isc,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0xF3 => Instruction {
                opcode: Opcode::Isc,
                mode: AddressingMode::IndirectY(byte_after_opcode),
                cycles: 7,
                page_cross_cost: false,
            },
            0xF7 => Instruction {
                opcode: Opcode::Isc,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0xFB => Instruction {
                opcode: Opcode::Isc,
                mode: AddressingMode::AbsoluteY(word_after_opcode),
                cycles: 7,
                page_cross_cost: false,
            },
            0xFF => Instruction {
                opcode: Opcode::Isc,
                mode: AddressingMode::AbsoluteX(word_after_opcode),
                cycles: 7,
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
            /* Lax */
            0xA3 => Instruction {
                opcode: Opcode::Lax,
                mode: AddressingMode::IndirectX(byte_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0xA7 => Instruction {
                opcode: Opcode::Lax,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 3,
                page_cross_cost: false,
            },
            0xAF => Instruction {
                opcode: Opcode::Lax,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0xB3 => Instruction {
                opcode: Opcode::Lax,
                mode: AddressingMode::IndirectY(byte_after_opcode),
                cycles: 5,
                page_cross_cost: false,
            },
            0xB7 => Instruction {
                opcode: Opcode::Lax,
                mode: AddressingMode::ZeroPageY(byte_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0xBF => Instruction {
                opcode: Opcode::Lax,
                mode: AddressingMode::AbsoluteY(word_after_opcode),
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
                mode: AddressingMode::Accumulator,
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
            0x04 => Instruction {
                opcode: Opcode::Nop,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 2,
                page_cross_cost: false,
            },
            0x0C => Instruction {
                opcode: Opcode::Nop,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 2,
                page_cross_cost: false,
            },
            0x14 => Instruction {
                opcode: Opcode::Nop,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 2,
                page_cross_cost: false,
            },
            0x1A => Instruction {
                opcode: Opcode::Nop,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            0x1C => Instruction {
                opcode: Opcode::Nop,
                mode: AddressingMode::AbsoluteX(word_after_opcode),
                cycles: 2,
                page_cross_cost: false,
            },
            0x3A => Instruction {
                opcode: Opcode::Nop,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            0x34 => Instruction {
                opcode: Opcode::Nop,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 2,
                page_cross_cost: false,
            },
            0x3C => Instruction {
                opcode: Opcode::Nop,
                mode: AddressingMode::AbsoluteX(word_after_opcode),
                cycles: 2,
                page_cross_cost: false,
            },
            0x44 => Instruction {
                opcode: Opcode::Nop,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 2,
                page_cross_cost: false,
            },
            0x54 => Instruction {
                opcode: Opcode::Nop,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 2,
                page_cross_cost: false,
            },
            0x5A => Instruction {
                opcode: Opcode::Nop,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            0x5C => Instruction {
                opcode: Opcode::Nop,
                mode: AddressingMode::AbsoluteX(word_after_opcode),
                cycles: 2,
                page_cross_cost: false,
            },
            0x64 => Instruction {
                opcode: Opcode::Nop,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 2,
                page_cross_cost: false,
            },
            0x74 => Instruction {
                opcode: Opcode::Nop,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 2,
                page_cross_cost: false,
            },
            0x7A => Instruction {
                opcode: Opcode::Nop,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            0x7C => Instruction {
                opcode: Opcode::Nop,
                mode: AddressingMode::AbsoluteX(word_after_opcode),
                cycles: 2,
                page_cross_cost: false,
            },
            0x80 => Instruction {
                opcode: Opcode::Nop,
                mode: AddressingMode::Immediate(byte_after_opcode),
                cycles: 2,
                page_cross_cost: false,
            },
            0xD4 => Instruction {
                opcode: Opcode::Nop,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 2,
                page_cross_cost: false,
            },
            0xDA => Instruction {
                opcode: Opcode::Nop,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            0xDC => Instruction {
                opcode: Opcode::Nop,
                mode: AddressingMode::AbsoluteX(word_after_opcode),
                cycles: 2,
                page_cross_cost: false,
            },
            0xEA => Instruction {
                opcode: Opcode::Nop,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            0xF4 => Instruction {
                opcode: Opcode::Nop,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 2,
                page_cross_cost: false,
            },
            0xFA => Instruction {
                opcode: Opcode::Nop,
                mode: AddressingMode::Implicit,
                cycles: 2,
                page_cross_cost: false,
            },
            0xFC => Instruction {
                opcode: Opcode::Nop,
                mode: AddressingMode::AbsoluteX(word_after_opcode),
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
            /* Rla */
            0x23 => Instruction {
                opcode: Opcode::Rla,
                mode: AddressingMode::IndirectX(byte_after_opcode),
                cycles: 8,
                page_cross_cost: false,
            },
            0x27 => Instruction {
                opcode: Opcode::Rla,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 5,
                page_cross_cost: false,
            },
            0x2F => Instruction {
                opcode: Opcode::Rla,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0x33 => Instruction {
                opcode: Opcode::Rla,
                mode: AddressingMode::IndirectY(byte_after_opcode),
                cycles: 8,
                page_cross_cost: false,
            },
            0x37 => Instruction {
                opcode: Opcode::Rla,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0x3B => Instruction {
                opcode: Opcode::Rla,
                mode: AddressingMode::AbsoluteY(word_after_opcode),
                cycles: 7,
                page_cross_cost: false,
            },
            0x3F => Instruction {
                opcode: Opcode::Rla,
                mode: AddressingMode::AbsoluteX(word_after_opcode),
                cycles: 7,
                page_cross_cost: false,
            },
            /* Rol */
            0x2A => Instruction {
                opcode: Opcode::Rol,
                mode: AddressingMode::Accumulator,
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
                mode: AddressingMode::Accumulator,
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
            /* Rra */
            0x63 => Instruction {
                opcode: Opcode::Rra,
                mode: AddressingMode::IndirectX(byte_after_opcode),
                cycles: 8,
                page_cross_cost: false,
            },
            0x67 => Instruction {
                opcode: Opcode::Rra,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 5,
                page_cross_cost: false,
            },
            0x6F => Instruction {
                opcode: Opcode::Rra,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0x73 => Instruction {
                opcode: Opcode::Rra,
                mode: AddressingMode::IndirectY(byte_after_opcode),
                cycles: 8,
                page_cross_cost: false,
            },
            0x77 => Instruction {
                opcode: Opcode::Rra,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0x7B => Instruction {
                opcode: Opcode::Rra,
                mode: AddressingMode::AbsoluteY(word_after_opcode),
                cycles: 7,
                page_cross_cost: false,
            },
            0x7F => Instruction {
                opcode: Opcode::Rra,
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
            /* Sax */
            0x83 => Instruction {
                opcode: Opcode::Sax,
                mode: AddressingMode::IndirectX(byte_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0x87 => Instruction {
                opcode: Opcode::Sax,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 3,
                page_cross_cost: false,
            },
            0x8F => Instruction {
                opcode: Opcode::Sax,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            0x97 => Instruction {
                opcode: Opcode::Sax,
                mode: AddressingMode::ZeroPageY(byte_after_opcode),
                cycles: 4,
                page_cross_cost: false,
            },
            /* Sbc */
            0xE9 => Instruction {
                opcode: Opcode::Sbc,
                mode: AddressingMode::Immediate(byte_after_opcode),
                cycles: 2,
                page_cross_cost: false,
            },
            0xEB => Instruction {
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
            /* Slo */
            0x03 => Instruction {
                opcode: Opcode::Slo,
                mode: AddressingMode::IndirectX(byte_after_opcode),
                cycles: 8,
                page_cross_cost: false,
            },
            0x07 => Instruction {
                opcode: Opcode::Slo,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 5,
                page_cross_cost: false,
            },
            0x0F => Instruction {
                opcode: Opcode::Slo,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0x13 => Instruction {
                opcode: Opcode::Slo,
                mode: AddressingMode::IndirectY(byte_after_opcode),
                cycles: 8,
                page_cross_cost: false,
            },
            0x17 => Instruction {
                opcode: Opcode::Slo,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0x1B => Instruction {
                opcode: Opcode::Slo,
                mode: AddressingMode::AbsoluteY(word_after_opcode),
                cycles: 7,
                page_cross_cost: false,
            },
            0x1F => Instruction {
                opcode: Opcode::Slo,
                mode: AddressingMode::AbsoluteX(word_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            /* Sre */
            0x43 => Instruction {
                opcode: Opcode::Sre,
                mode: AddressingMode::IndirectX(byte_after_opcode),
                cycles: 8,
                page_cross_cost: false,
            },
            0x47 => Instruction {
                opcode: Opcode::Sre,
                mode: AddressingMode::ZeroPage(byte_after_opcode),
                cycles: 5,
                page_cross_cost: false,
            },
            0x4F => Instruction {
                opcode: Opcode::Sre,
                mode: AddressingMode::Absolute(word_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
           0x53 => Instruction {
                opcode: Opcode::Sre,
                mode: AddressingMode::IndirectY(byte_after_opcode),
                cycles: 8,
                page_cross_cost: false,
            },
            0x57 => Instruction {
                opcode: Opcode::Sre,
                mode: AddressingMode::ZeroPageX(byte_after_opcode),
                cycles: 6,
                page_cross_cost: false,
            },
            0x5B => Instruction {
                opcode: Opcode::Sre,
                mode: AddressingMode::AbsoluteY(word_after_opcode),
                cycles: 7,
                page_cross_cost: false,
            },
            0x5F => Instruction {
                opcode: Opcode::Sre,
                mode: AddressingMode::AbsoluteX(word_after_opcode),
                cycles: 7,
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
            opcode => panic!("0x{:02X} not implemented!", opcode),
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
            AddressingMode::Accumulator => 0,
        };

        self.pc += instruction_length;
        result
    }

    pub fn execute_instruction(&mut self, instruction: Instruction) {
        match instruction.opcode {
            Opcode::Add => self.adc(instruction.mode),
            Opcode::And => self.and(instruction.mode),
            Opcode::Asl => self.asl(instruction.mode),
            Opcode::Bcc => self.bcc(instruction.mode),
            Opcode::Bcs => self.bcs(instruction.mode),
            Opcode::Beq => self.beq(instruction.mode),
            Opcode::Bit => self.bit(instruction.mode),
            Opcode::Bmi => self.bmi(instruction.mode),
            Opcode::Bne => self.bne(instruction.mode),
            Opcode::Bpl => self.bpl(instruction.mode),
            Opcode::Brk => self.brk(instruction.mode),
            Opcode::Bvc => self.bvc(instruction.mode),
            Opcode::Bvs => self.bvs(instruction.mode),
            Opcode::Clc => self.clc(),
            Opcode::Cld => self.cld(),
            Opcode::Cli => self.cli(),
            Opcode::Clv => self.clv(),
            Opcode::Cmp => self.cmp(instruction.mode),
            Opcode::Cpx => self.cpx(instruction.mode),
            Opcode::Cpy => self.cpy(instruction.mode),
            Opcode::Dcp => self.dcp(instruction.mode),
            Opcode::Dec => self.dec(instruction.mode),
            Opcode::Dex => self.dex(),
            Opcode::Dey => self.dey(),
            Opcode::Eor => self.eor(instruction.mode),
            Opcode::Inc => self.inc(instruction.mode),
            Opcode::Inx => self.inx(),
            Opcode::Iny => self.iny(),
            Opcode::Isc => self.isc(instruction.mode),
            Opcode::Jmp => self.jmp(instruction.mode),
            Opcode::Jsr => self.jsr(instruction.mode),
            Opcode::Lax => self.lax(instruction.mode),
            Opcode::Lda => self.lda(instruction.mode),
            Opcode::Ldx => self.ldx(instruction.mode),
            Opcode::Ldy => self.ldy(instruction.mode),
            Opcode::Lsr => self.lsr(instruction.mode),
            Opcode::Nop => self.nop(),
            Opcode::Ora => self.ora(instruction.mode),
            Opcode::Pha => self.pha(),
            Opcode::Php => self.php(),
            Opcode::Pla => self.pla(),
            Opcode::Plp => self.plp(),
            Opcode::Rla => self.rla(instruction.mode),
            Opcode::Rol => self.rol(instruction.mode),
            Opcode::Ror => self.ror(instruction.mode),
            Opcode::Rra => self.rra(instruction.mode),
            Opcode::Rti => self.rti(),
            Opcode::Rts => self.rts(),
            Opcode::Sax => self.sax(instruction.mode),
            Opcode::Sbc => self.sbc(instruction.mode),
            Opcode::Sec => self.sec(),
            Opcode::Sed => self.sed(),
            Opcode::Sei => self.sei(),
            Opcode::Slo => self.slo(instruction.mode),
            Opcode::Sre => self.sre(instruction.mode),
            Opcode::Sta => self.sta(instruction.mode),
            Opcode::Stx => self.stx(instruction.mode),
            Opcode::Sty => self.sty(instruction.mode),
            Opcode::Tax => self.tax(),
            Opcode::Tay => self.tay(),
            Opcode::Tsx => self.tsx(),
            Opcode::Txa => self.txa(),
            Opcode::Txs => self.txs(),
            Opcode::Tya => self.tya(),
        }
    }

    fn read_with_addressing_mode(&mut self, mode: AddressingMode) -> u8 {
        match mode {
            AddressingMode::ZeroPage(val) => self.bus.get_byte_at(u16::from(val)),
            AddressingMode::ZeroPageX(val) => self.bus.get_byte_at(u16::from(val.wrapping_add(self.x))),
            AddressingMode::ZeroPageY(val) => self.bus.get_byte_at(u16::from(val.wrapping_add(self.y))),
            AddressingMode::Absolute(addr) => self.bus.get_byte_at(addr),
            AddressingMode::AbsoluteX(val) => self.bus.get_byte_at(val + u16::from(self.x)),
            AddressingMode::AbsoluteY(val) => self.bus.get_byte_at(val.wrapping_add(u16::from(self.y))),
            AddressingMode::IndirectX(val) => {
                let low_byte = self.bus.get_byte_at((val.wrapping_add(self.x)) as u16);
                let high_byte = self.bus.get_byte_at((val.wrapping_add(self.x).wrapping_add(1)) as u16);
                let indirect_addr = ((high_byte as u16) << 8) | (low_byte as u16);
                self.bus.get_byte_at(indirect_addr)
            },
            AddressingMode::IndirectY(val) => {
                let low_byte = self.bus.get_byte_at(val as u16);
                let high_byte = self.bus.get_byte_at((val.wrapping_add(1)) as u16);
                let indirect_addr = ((((high_byte as u16) << 8) | (low_byte as u16))).wrapping_add(self.y as u16);
                self.bus.get_byte_at(indirect_addr)
            },
            AddressingMode::Immediate(val) => val,
            AddressingMode::Accumulator => self.accumulator,
            _ => panic!("Attempted to read from address value of {:?} illegally", mode),
        }
    }

    fn write_with_addressing_mode(&mut self, mode: AddressingMode, assigned_val: u8) {
        match mode {
            AddressingMode::ZeroPage(val) => self.bus.set_byte_at(u16::from(val), assigned_val),
            AddressingMode::ZeroPageX(val) => self.bus.set_byte_at(u16::from(val.wrapping_add(self.x)), assigned_val),
            AddressingMode::ZeroPageY(val) => self.bus.set_byte_at(u16::from(val.wrapping_add(self.y)), assigned_val),
            AddressingMode::Absolute(addr) => self.bus.set_byte_at(addr, assigned_val),
            AddressingMode::AbsoluteX(val) => self.bus.set_byte_at(val + u16::from(self.x), assigned_val),
            AddressingMode::AbsoluteY(val) => self.bus.set_byte_at(val + u16::from(self.y), assigned_val),
            AddressingMode::IndirectX(val) => {
                let low_byte = self.bus.get_byte_at((val.wrapping_add(self.x)) as u16);
                let high_byte = self.bus.get_byte_at((val.wrapping_add(self.x).wrapping_add(1)) as u16);
                let indirect_addr = ((high_byte as u16) << 8) | (low_byte as u16);
                self.bus.set_byte_at(indirect_addr, assigned_val);
            },
            AddressingMode::IndirectY(val) => {
                let low_byte = self.bus.get_byte_at(val as u16);
                let high_byte = self.bus.get_byte_at((val.wrapping_add(1)) as u16);
                let indirect_addr = ((((high_byte as u16) << 8) | (low_byte as u16))).wrapping_add(self.y as u16);
                self.bus.set_byte_at(indirect_addr, assigned_val);
            },
            AddressingMode::Accumulator => self.accumulator = assigned_val,
            _ => panic!("Attempted to write to address value of {:?} illegally", mode),
        }
    }

    fn adc(&mut self, mode: AddressingMode) {
        let to_be_added = self.read_with_addressing_mode(mode);
        let old_accumulator = self.accumulator;

        let (first_add, first_carry) = old_accumulator.overflowing_add(to_be_added);
        let (result, second_carry) = first_add.overflowing_add(u8::from(self.carry));

        self.accumulator = result;
        self.sign = (result as i8) < 0;
        self.zero = result == 0;
        self.overflow = ((to_be_added ^ result) & (old_accumulator ^ result) & 0x80) != 0;
        self.carry = first_carry | second_carry;
    }

    fn and(&mut self, mode: AddressingMode) {
        let to_be_anded = self.read_with_addressing_mode(mode);
        let result = to_be_anded & self.accumulator;

        self.accumulator = result;
        self.sign = (result as i8) < 0;
        self.zero = result == 0;
    }

    fn asl(&mut self, mode: AddressingMode) {
        let to_be_asled = self.read_with_addressing_mode(mode);
        let result = to_be_asled << 1;

        self.write_with_addressing_mode(mode, result);
        self.sign = (result as i8) < 0;
        self.zero = result == 0;
        self.carry = (to_be_asled & (1 << 7)) != 0;
    }

    fn branch(&mut self, condition: bool, offset: i8) {
        if condition {
            // If branch is taken, add 1 cycle
            self.cycles_left += 1;
            let new_pc = (self.pc as i32 + offset as i32) as u16;

            // If branch is to new page, add 1 more cycle
            if (new_pc / 0x100) != (self.pc / 0x100) {
                self.cycles_left += 1;
            }

            self.pc = new_pc;
        }
    }

    fn bcc(&mut self, mode: AddressingMode) {
        match mode {
            AddressingMode::Relative(offset) => self.branch(!self.carry, offset),
            _ => panic!("Cannot branch using {:?}", mode),
        }
    }

    fn bcs(&mut self, mode: AddressingMode) {
        match mode {
            AddressingMode::Relative(offset) => self.branch(self.carry, offset),
            _ => panic!("Cannot branch using {:?}", mode),
        }
    }

    fn beq(&mut self, mode: AddressingMode) {
        match mode {
            AddressingMode::Relative(offset) => self.branch(self.zero, offset),
            _ => panic!("Cannot branch using {:?}", mode),
        }
    }

    fn bit(&mut self, mode: AddressingMode) {
        let val = self.read_with_addressing_mode(mode);
        self.sign = (val & (1 << 7)) != 0;
        self.overflow = (val & (1 << 6)) != 0;

        self.zero = (val & self.accumulator) == 0;
    }

    fn bmi(&mut self, mode: AddressingMode) {
        match mode {
            AddressingMode::Relative(offset) => self.branch(self.sign, offset),
            _ => panic!("Cannot branch using {:?}", mode),
        }
    }

    fn bne(&mut self, mode: AddressingMode) {
        match mode {
            AddressingMode::Relative(offset) => self.branch(!self.zero, offset),
            _ => panic!("Cannot branch using {:?}", mode),
        }
    }

    fn bpl(&mut self, mode: AddressingMode) {
        match mode {
            AddressingMode::Relative(offset) => self.branch(!self.sign, offset),
            _ => panic!("Cannot branch using {:?}", mode),
        }
    }

    fn brk(&mut self, mode: AddressingMode) {
        self.interrupt(Interrupt::Irq);
        //self.pc += 1; //brk requires a padding byte
    }

    fn bvc(&mut self, mode: AddressingMode) {
        match mode {
            AddressingMode::Relative(offset) => self.branch(!self.overflow, offset),
            _ => panic!("Cannot branch using {:?}", mode),
        }
    }

    fn bvs(&mut self, mode: AddressingMode) {
        match mode {
            AddressingMode::Relative(offset) => self.branch(self.overflow, offset),
            _ => panic!("Cannot branch using {:?}", mode),
        }
    }

    fn clc(&mut self) {
        self.carry = false;
    }

    fn cld(&mut self) {
        self.decimal = false;
    }

    fn cli(&mut self) {
        self.interrupt = false;
    }

    fn clv(&mut self) {
        self.overflow = false;
    }

    fn cmp(&mut self, mode: AddressingMode) {
        let to_compare = self.read_with_addressing_mode(mode);

        self.sign = (self.accumulator.wrapping_sub(to_compare) as i8) < 0;
        self.zero = self.accumulator == to_compare;
        self.carry = self.accumulator >= to_compare;
    }

    fn cpx(&mut self, mode: AddressingMode) {
        let to_compare = self.read_with_addressing_mode(mode);

        self.sign = (self.x.wrapping_sub(to_compare) as i8) < 0;
        self.zero = self.x == to_compare;
        self.carry = self.x >= to_compare;
    }

    fn cpy(&mut self, mode: AddressingMode) {
        let to_compare = self.read_with_addressing_mode(mode);

        self.sign = (self.y.wrapping_sub(to_compare) as i8) < 0;
        self.zero = self.y == to_compare;
        self.carry = self.y >= to_compare;
    }

    // Equivalent to dec then cmp
    fn dcp(&mut self, mode: AddressingMode) {
        self.dec(mode);
        self.cmp(mode);
    }

    fn dec(&mut self, mode: AddressingMode) {
        let old_val = self.read_with_addressing_mode(mode);
        let result = old_val.wrapping_sub(1);
        self.write_with_addressing_mode(mode, result);

        self.sign = (result as i8) < 0;
        self.zero = result == 0;
    }

    fn dex(&mut self) {
        self.x = self.x.wrapping_sub(1);

        self.sign = (self.x as i8) < 0;
        self.zero = self.x == 0;
    }

    fn dey(&mut self) {
        self.y = self.y.wrapping_sub(1);

        self.sign = (self.y as i8) < 0;
        self.zero = self.y == 0;
    }

    fn eor(&mut self, mode: AddressingMode) {
        self.accumulator ^= self.read_with_addressing_mode(mode);

        self.sign = (self.accumulator as i8) < 0;
        self.zero = self.accumulator == 0;
    }

    fn inc(&mut self, mode: AddressingMode) {
        let old_val = self.read_with_addressing_mode(mode);
        let result = old_val.wrapping_add(1);
        self.write_with_addressing_mode(mode, result);

        self.sign = (result as i8) < 0;
        self.zero = result == 0;
    }

    fn inx(&mut self) {
        self.x = self.x.wrapping_add(1);

        self.sign = (self.x as i8) < 0;
        self.zero = self.x == 0;
    }

    fn iny(&mut self) {
        self.y = self.y.wrapping_add(1);

        self.sign = (self.y as i8) < 0;
        self.zero = self.y == 0;
    }

    // Equivalent to inc then sbc
    fn isc(&mut self, mode: AddressingMode) {
        self.inc(mode);
        self.sbc(mode);
    }

    fn jmp(&mut self, mode: AddressingMode) {
        match mode {
            AddressingMode::Absolute(addr) => {
                self.pc = addr;
            },
            // jmp (xxFF) will read from xxFF and xx00 instead of crossing page boundary.
            AddressingMode::Indirect(addr) => {
                if addr & 0x00FF == 0xFF {
                    let low_byte = self.bus.get_byte_at(addr);
                    let high_byte = self.bus.get_byte_at(addr & 0xFF00);
                    self.pc = ((high_byte as u16) << 8) | (low_byte as u16);
                } else {
                    self.pc = self.bus.get_word_at(addr);
                }
            },
            _ => panic!("Cannot jmp using {:?}", mode),
        }
    }

    fn jsr(&mut self, mode: AddressingMode) {
        match mode {
            AddressingMode::Absolute(addr) => {
                self.push_word(self.pc - 1);
                self.pc = addr;
            },
            _ => panic!("Cannot jsr using {:?}", mode),
        }
    }

    // Shortcut for lda then tax
    fn lax(&mut self, mode: AddressingMode) {
        // lda
        self.accumulator = self.read_with_addressing_mode(mode);

        self.sign = (self.accumulator as i8) < 0;
        self.zero = self.accumulator == 0;

        // tax
        self.x = self.accumulator;
    }

    fn lda(&mut self, mode: AddressingMode) {
        self.accumulator = self.read_with_addressing_mode(mode);

        self.sign = (self.accumulator as i8) < 0;
        self.zero = self.accumulator == 0;
    }

    fn ldx(&mut self, mode: AddressingMode) {
        self.x = self.read_with_addressing_mode(mode);

        self.sign = (self.x as i8) < 0;
        self.zero = self.x == 0;
    }

    fn ldy(&mut self, mode: AddressingMode) {
        self.y = self.read_with_addressing_mode(mode);

        self.sign = (self.y as i8) < 0;
        self.zero = self.y == 0;
    }

    fn lsr(&mut self, mode: AddressingMode) {
        let to_be_lsred = self.read_with_addressing_mode(mode);
        let result = to_be_lsred >> 1;

        self.write_with_addressing_mode(mode, result);
        self.sign = false;
        self.zero = result == 0;
        self.carry = (to_be_lsred & (1 << 0)) != 0;
    }

    fn nop(&mut self) {
        /* do nothing */
    }

    fn ora(&mut self, mode: AddressingMode) {
        self.accumulator |= self.read_with_addressing_mode(mode);

        self.sign = (self.accumulator as i8) < 0;
        self.zero = self.accumulator == 0;
    }

    fn pha(&mut self) {
        self.push_byte(self.accumulator);
    }

    fn php(&mut self) {
        let processor_status: u8 = ((self.sign as u8) << 7)
            | ((self.overflow as u8) << 6)
            | ((1 as u8) << 5)
            | ((1 as u8) << 4)
            | ((self.decimal as u8) << 3)
            | ((self.interrupt as u8) << 2)
            | ((self.zero as u8) << 1)
            | ((self.carry as u8) << 0);
        self.push_byte(processor_status);
    }

    fn pla(&mut self) {
        self.accumulator = self.pop_byte();

        self.zero = self.accumulator == 0;
        self.sign = (self.accumulator as i8) < 0;
    }

    fn plp(&mut self) {
        let processor_flags = self.pop_byte();

        self.sign = (processor_flags & (1 << 7)) != 0;
        self.overflow = (processor_flags & (1 << 6)) != 0;
        let _ = (processor_flags & (1 << 5)) != 0;
        let _ = (processor_flags & (1 << 4)) != 0;
        self.decimal = (processor_flags & (1 << 3)) != 0;
        self.interrupt = (processor_flags & (1 << 2)) != 0;
        self.zero = (processor_flags & (1 << 1)) != 0;
        self.carry = (processor_flags & (1 << 0)) != 0;
    }

    // Equivalent to rol then and
    fn rla(&mut self, mode: AddressingMode) {
        self.rol(mode);
        self.and(mode);
    }

    fn rol(&mut self, mode: AddressingMode) {
        let to_be_roled = self.read_with_addressing_mode(mode);
        let new_val = (to_be_roled << 1) | ((self.carry as u8) << 0);

        self.write_with_addressing_mode(mode, new_val);
        self.sign = (new_val as i8) < 0;
        self.zero = new_val == 0;
        self.carry = (to_be_roled & (1 << 7)) != 0;
    }

    fn ror(&mut self, mode: AddressingMode) {
        let to_be_rored = self.read_with_addressing_mode(mode);
        let new_val = (to_be_rored >> 1) | ((self.carry as u8) << 7);

        self.write_with_addressing_mode(mode, new_val);
        self.sign = (new_val as i8) < 0;
        self.zero = new_val == 0;
        self.carry = (to_be_rored & (1 << 0)) != 0;
    }

    // Equivalent to ror then adc
    fn rra(&mut self, mode: AddressingMode) {
        self.ror(mode);
        self.adc(mode);
    }

    fn rti(&mut self) {
        // Pull processor flags from stack
        self.plp();

        self.pc = self.pop_word();
    }

    fn rts(&mut self) {
        self.pc = self.pop_word() + 1;
    }

    fn sax(&mut self, mode: AddressingMode) {
        let result = self.accumulator & self.x;

        self.write_with_addressing_mode(mode, result);
    }

    fn sbc(&mut self, mode: AddressingMode) {
        // We can take advantage of:
        // A - M - (1 - C)
        // A + !M + 1 - (1 - C)
        // A + !M + 1 + C - 1
        // A + !M + C -> same as adc

        let to_be_added = !self.read_with_addressing_mode(mode);
        let old_accumulator = self.accumulator;

        let (first_add, first_carry) = old_accumulator.overflowing_add(to_be_added);
        let (result, second_carry) = first_add.overflowing_add(u8::from(self.carry));

        self.accumulator = result;
        self.sign = (result as i8) < 0;
        self.zero = result == 0;
        self.overflow = ((to_be_added ^ result) & (old_accumulator ^ result) & 0x80) != 0;
        self.carry = first_carry | second_carry;
    }

    fn sec(&mut self) {
        self.carry = true;
    }

    fn sed(&mut self) {
        self.decimal = true;
    }

    fn sei(&mut self) {
        self.interrupt = true;
    }

    // Equivalent to asl then ora
    fn slo(&mut self, mode: AddressingMode) {
        self.asl(mode);
        self.ora(mode);
    }

    // Equivalent to lsr then eor
    fn sre(&mut self, mode: AddressingMode) {
        self.lsr(mode);
        self.eor(mode);
    }

    fn sta(&mut self, mode: AddressingMode) {
        self.write_with_addressing_mode(mode, self.accumulator);
    }

    fn stx(&mut self, mode: AddressingMode) {
        self.write_with_addressing_mode(mode, self.x);
    }

    fn sty(&mut self, mode: AddressingMode) {
        self.write_with_addressing_mode(mode, self.y);
    }

    fn tax(&mut self) {
        self.x = self.accumulator;

        self.zero = self.x == 0;
        self.sign = (self.x as i8) < 0;
    }

    fn tay(&mut self) {
        self.y = self.accumulator;

        self.zero = self.y == 0;
        self.sign = (self.y as i8) < 0;
    }

    fn tsx(&mut self) {
        self.x = self.sp;

        self.zero = self.x == 0;
        self.sign = (self.x as i8) < 0;
    }

    fn txa(&mut self) {
        self.accumulator = self.x;

        self.zero = self.accumulator == 0;
        self.sign = (self.accumulator as i8) < 0;
    }

    fn txs(&mut self) {
        self.sp = self.x;
    }

    fn tya(&mut self) {
        self.accumulator = self.y;

        self.zero = self.accumulator == 0;
        self.sign = (self.accumulator as i8) < 0;
    }
}
