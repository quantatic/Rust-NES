use crate::ppu::Ppu;

use crate::rom::Rom;

pub struct Bus {
    pub ram: [u8; 0x2000],
    pub rom: Rom,
    pub ppu: Ppu,
}

impl Bus {
    pub fn new(rom: Rom, ppu: Ppu) -> Self {
        let mut res = Self {
            ram: [0u8; 0x2000],
            rom,
            ppu
        };

        for i in 0..res.rom.chr_rom.len() {
            res.ppu.vram[i] = res.rom.chr_rom[i];
        }

        res
    }

    pub fn get_byte_at(&mut self, addr: u16) -> u8 {
        match addr {
            // 0x0000-0x07FF mirrored at 0x0800, 0x1000, and 0x18000.
            0x0000..=0x1FFF => {
                let actual_addr = addr % 0x0800;
                self.ram[usize::from(actual_addr)]
            },
            0x2000..=0x3FFF => {
                // 0x2000-0x2007 mirrored in 0x2000-0x4000
                let actual_addr = ((addr - 0x2000) % 0x8) + 0x2000;
                match actual_addr {
                    0x2000 | 0x2001  => panic!("Not allowed to read from 0x{:04x}", addr),
                    // Reading from ppustatus register clears bit 7 (v-blank)
                    0x2002 => {
                        let result = self.ppu.ppustatus;
                        self.ppu.ppustatus &= 0b01111111;
                        self.ppu.ppuscroll = 0;
                        self.ppu.ppuaddr = 0;
                        self.ppu.two_write_partial = false;
                        result
                    },
                    0x2003..=0x2006 => panic!("Not allowed to read from 0x{:04X}", addr),
                    0x2007 => {
                        self.ppu.get_vram_byte_at(self.ppu.ppuaddr)
                    },
                    _ => panic!(),
                }
            },
            0x4000..=0x4017 => {
                //println!("Ignoring read from 0x{:05x} for now", addr);
                0x0
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
                // 0x2000-0x2007 mirrored in 0x2000-0x4000
                let actual_addr = ((addr - 0x2000) % 0x8) + 0x2000;
                match actual_addr {
                    0x2000 => self.ppu.ppuctrl = val,
                    0x2001 => self.ppu.ppumask = val,
                    0x2002 => panic!("Not allowed to write to 0x{:04x}", addr),
                    0x2003 => self.ppu.oamaddr = val,
                    0x2004 => panic!("Writing to SPR-RAM at 0x{:04x}", self.ppu.oamdata),
                    0x2005 => {
                        match self.ppu.two_write_partial {
                            false => self.ppu.ppuscroll |= ((val as u16) << 8),
                            true => self.ppu.ppuscroll |= (val as u16),
                        };
                        self.ppu.two_write_partial = !self.ppu.two_write_partial;
                    },
                    0x2006 => {
                        match self.ppu.two_write_partial {
                            false => self.ppu.ppuaddr |= ((val as u16) << 8),
                            true => self.ppu.ppuaddr |= (val as u16),
                        };
                        self.ppu.two_write_partial = !self.ppu.two_write_partial;
                    },
                    0x2007 => {
                        assert!(!self.ppu.two_write_partial);
                        self.ppu.set_vram_byte_at(self.ppu.ppuaddr, val);
                        // Check bit 2 of ctrl1 to see how much to increment ppuaddr by
                        self.ppu.ppuaddr += if (self.ppu.ppuctrl & (1 << 2)) == 0 {
                            0x1
                        } else {
                            0x20
                        }
                    },
                    _ => panic!(),
                }
            },
            0x4000..=0x4017 => {
                match addr {
                    // Direct memory access (DMA)
                    0x4014 => {
                        let start_addr = (val as u16) << 8;
                        for i in 0..=255 {
                            let data = self.get_byte_at(start_addr + u16::from(i));
                            self.ppu.set_oam_byte_at(self.ppu.oamaddr.wrapping_add(i), data);
                        }
                    },
                    _ => { }//println!("Ignoring write to 0x{:05x} for now", addr),
                }
            }
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
