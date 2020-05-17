use crate::controller::Controller;
use crate::ppu::Ppu;
use crate::rom::Rom;

use std::cell::RefCell;
use std::rc::Rc;

pub struct Bus {
    pub ram: [u8; 0x2000],
    pub rom: Rom,
    pub ppu: Ppu,
	pub controller: Controller,
}

impl Bus {
    pub fn new(rom: Rom, ppu: Ppu, controller: Controller) -> Self {
        let mut res = Self {
            ram: [0u8; 0x2000],
            rom,
            ppu,
			controller
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
                        self.ppu.ppustatus &= 0b01111111; // clear vblank when we read 0x2002
                        self.ppu.ppuscroll = 0;
                        self.ppu.ppuaddr = 0;
                        self.ppu.two_write_partial = false;
                        result
                    },
					0x2003 => panic!("Not allowed to read from 0x2003"),
					0x2004 => {
						self.ppu.get_oam_byte_at(self.ppu.oamaddr)
					},
                    0x2005..=0x2006 => panic!("Not allowed to read from 0x{:04X}", addr),
                    0x2007 => {
						let addr = self.ppu.ppuaddr;
                        let data = self.ppu.get_vram_byte_at(self.ppu.ppuaddr);

						// If not reading from palette, the results are buffered (we return the
						// result of the LAST read). When reading from palette we still update
						// buffer, but just return the actual value.
						let returned_data = if (addr % 0x4000) < 0x3F00 {
							let buffered_data = self.ppu.ppudata_buffer;
							self.ppu.ppudata_buffer = data;
							buffered_data
						} else {
							// Directly from the docs: When reading while the VRAM address is in the
							// range 0-$3EFF (i.e., before the palettes), the read will return the
							// contents of an internal read buffer. This internal buffer is updated
							// only when reading PPUDATA, and so is preserved across frames. After
							// the CPU reads and gets the contents of the internal buffer, the PPU will
							// immediately update the internal buffer with the byte at the current VRAM
							// address. Thus, after setting the VRAM address, one should first read
							// this register and discard the result.
							//
							// Reading palette data from $3F00-$3FFF works differently. The palette data
							// is placed immediately on the data bus, and hence no dummy read is required.
							// Reading the palettes still updates the internal buffer though, but the data
							// placed in it is the mirrored nametable data that would appear "underneath"
							// the palette. (Checking the PPU memory map should make this clearer.)
							//
							// Buffered data is actually set to the data that WOULD appear in the
							// mirrored nametable "underneath" this palette. If your brain is tiny
							// like mine, take a gander at
							// https://wiki.nesdev.com/w/index.php/PPU_memory_map. The nametable is
							// mirrored at a size of 0x1000, so since we already know we're gonna
							// have to look at the mirorred nametable (we're working with palette
							// now), just subtract 0x1000 and call it a day.

							self.ppu.ppudata_buffer = self.ppu.get_vram_byte_at(self.ppu.ppuaddr - 0x1000);

							data
						};

                        self.ppu.ppuaddr += if (self.ppu.ppuctrl & (1 << 2)) == 0 {
                            0x1
                        } else {
                            0x20
                        };

						returned_data
                    },
                    _ => panic!(),
                }
            },
            0x4000..=0x4017 => {
				match addr {
					0x4016 => {
						self.controller.read() as u8
					},
					0x4017 => {
						//ignore read from controller 2
						0
					},
					_ => panic!("Don't know how to read from 0x{:05x}", addr)
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
                // 0x2000-0x2007 mirrored in 0x2000-0x4000
                let actual_addr = ((addr - 0x2000) % 0x8) + 0x2000;
                match actual_addr {
                    0x2000 => self.ppu.ppuctrl = val,
                    0x2001 => self.ppu.ppumask = val,
                    0x2002 => panic!("Not allowed to write to 0x{:04x}", addr),
                    0x2003 => self.ppu.oamaddr = val,
                    0x2004 => {
						self.ppu.set_oam_byte_at(self.ppu.oamaddr, val);
						// After writing, we need to increment oamaddr
						self.ppu.oamaddr = self.ppu.oamaddr.wrapping_add(1);
					},
                    0x2005 => {
                        match self.ppu.two_write_partial {
                            false => self.ppu.ppuscroll = ((val as u16) << 8),
                            true => self.ppu.ppuscroll |= (val as u16),
                        };
                        self.ppu.two_write_partial = !self.ppu.two_write_partial;
                    },
                    0x2006 => {
                        match self.ppu.two_write_partial {
                            false => self.ppu.ppuaddr = ((val as u16) << 8),
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
                        };
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
					0x4016 => {
						self.controller.set_strobe(val & 0x1 != 0);
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
			addr => println!("Ignoring write of {} to 0x{:05x} for now", char::from(val), addr),
        }
    }

    pub fn set_word_at(&mut self, addr: u16, val: u16) {
        let low = (val & 0b11111111) as u8;
        let high = ((val >> 8) & 0b11111111) as u8;

        self.set_byte_at(addr, low);
        self.set_byte_at(addr + 1, high);
    }
}
