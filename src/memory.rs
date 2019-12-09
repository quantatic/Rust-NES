use crate::ppu::Ppu;

use crate::rom::Rom;

pub struct Memory {
    ram: [u8; 0x2000],
    rom: Rom,
    ppu: Ppu,
}

impl Memory {
    pub fn new(rom: Rom, ppu: Ppu) -> Self {
        Memory {
            ram: [0u8; 0x2000],
            rom,
            ppu
        }
    }

    pub fn get_byte_at(&mut self, addr: u16) -> u8 {
        match addr {
            // 0x0000-0x07FF mirrored at 0x0800, 0x1000, and 0x18000.
            0x0000..=0x1FFF => {
                let actual_addr = addr % 0x0800;
                self.ram[usize::from(actual_addr)]
            },
            0x2000..=0x3FFF => {
                // 0x2000-0x2007 mirrored from 0x2000-0x4000
                let actual_addr = ((addr - 0x2000) % 0x8) + 0x2000;
                match actual_addr {
                    0x2000 | 0x2001 | 0x2003..=0x2007 => panic!("Not allowed to read from 0x{:04x}", addr),
                    0x2002 => self.ppu.status,
                    0x2007 => panic!("Reading a byte from VRAM at current address"),
                    _ => panic!(),
                }
            },
            0x8000..=0xFFFF => {
                let mut rom_access_addr = addr - 0x8000;
                if self.rom.prg_rom.len() <= 0x4000 { //if only one rom bank, should be mirrored
                    rom_access_addr %= 0x4000;
                }
                self.rom.prg_rom[usize::from(rom_access_addr)]
            },
            _ => panic!("Don't know how to read from 0x{:05x}", addr),
        }
    }

    pub fn get_word_at(&mut self, addr: u16) -> u16 {
        let low = self.get_byte_at(addr);
        let high = self.get_byte_at(addr + 1);
        ((high as u16) << 8) | (low as u16)
    }

    pub fn set_byte_at(&mut self, addr: u16, val: u8) {
        match addr {
            // 0x0000-0x07FF mirrored at 0x0800, 0x1000, and 0x18000.
            0x0000..=0x1FFF => {
                let actual_addr = addr % 0x0800;
                self.ram[usize::from(actual_addr)] = val;
            },
            0x2000..=0x3FFF => {
                // 0x2000-0x2007 mirrored from 0x2000-0x4000
                let actual_addr = ((addr - 0x2000) % 0x8) + 0x2000;
                match actual_addr {
                    0x2000 => self.ppu.ctrl_1 = val,
                    0x2001 => self.ppu.ctrl_2 = val,
                    0x2002 => panic!("Not allowed to write to 0x{:04x}", addr),
                    0x2003 => self.ppu.spr_addr = val,
                    0x2004 => panic!("Writing to SPR-RAM at 0x{:04x}", self.ppu.spr_addr),
                    0x2005 => self.ppu.vram_1 = val,
                    0x2006 => self.ppu.vram_2 = val,
                    0x2007 => panic!("Writing a byte to VRAM at current address"),
                    _ => panic!(),
                }
            },
            0x8000..=0xFFFF => {
                let mut rom_access_addr = addr - 0x8000;
                if self.rom.prg_rom.len() <= 0x4000 { //if only one rom bank, should be mirrored
                    rom_access_addr %= 0x4000;
                }
                self.rom.prg_rom[usize::from(rom_access_addr)] = val;
            },
            _ => panic!("Don't know how to write to 0x{:05x}", addr),
        }
    }

    pub fn set_word_at(&mut self, addr: u16, val: u16) {
        let low = (val & 0b11111111) as u8;
        let high = ((val >> 8) & 0b11111111) as u8;

        self.set_byte_at(addr, low);
        self.set_byte_at(addr + 1, high);
    }
}
