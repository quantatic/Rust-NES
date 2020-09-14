use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

use crate::rom;

use std::convert::TryFrom;

const SCALE: u32 = 3;

pub const PALETTE: [Color; 0x40] = [
    Color {
        r: 0x75,
        g: 0x75,
        b: 0x75,
        a: 0xFF,
    }, //0x00
    Color {
        r: 0x27,
        g: 0x1B,
        b: 0x8F,
        a: 0xFF,
    }, //0x01
    Color {
        r: 0x00,
        g: 0x00,
        b: 0xAB,
        a: 0xFF,
    }, //0x02
    Color {
        r: 0x47,
        g: 0x00,
        b: 0x9F,
        a: 0xFF,
    }, //0x03
    Color {
        r: 0x8F,
        g: 0x00,
        b: 0x77,
        a: 0xFF,
    }, //0x04
    Color {
        r: 0xAB,
        g: 0x00,
        b: 0x13,
        a: 0xFF,
    }, //0x05
    Color {
        r: 0xA7,
        g: 0x00,
        b: 0x00,
        a: 0xFF,
    }, //0x06
    Color {
        r: 0x7F,
        g: 0x0B,
        b: 0x00,
        a: 0xFF,
    }, //0x07
    Color {
        r: 0x43,
        g: 0x2F,
        b: 0x00,
        a: 0xFF,
    }, //0X08
    Color {
        r: 0x00,
        g: 0x47,
        b: 0x00,
        a: 0xFF,
    }, //0X09
    Color {
        r: 0x00,
        g: 0x51,
        b: 0x00,
        a: 0xFF,
    }, //0X0A
    Color {
        r: 0x00,
        g: 0x3F,
        b: 0x17,
        a: 0xFF,
    }, //0X0B
    Color {
        r: 0x1B,
        g: 0x3F,
        b: 0x5F,
        a: 0xFF,
    }, //0X0C
    Color {
        r: 0x00,
        g: 0x00,
        b: 0x00,
        a: 0xFF,
    }, //0X0D
    Color {
        r: 0x00,
        g: 0x00,
        b: 0x00,
        a: 0xFF,
    }, //0X0E
    Color {
        r: 0x00,
        g: 0x00,
        b: 0x00,
        a: 0xFF,
    }, //0X0F
    Color {
        r: 0xBC,
        g: 0xBC,
        b: 0xBC,
        a: 0xFF,
    }, //0X10
    Color {
        r: 0x00,
        g: 0x73,
        b: 0xEF,
        a: 0xFF,
    }, //0X11
    Color {
        r: 0x23,
        g: 0x3B,
        b: 0xEF,
        a: 0xFF,
    }, //0X12
    Color {
        r: 0x83,
        g: 0x00,
        b: 0xF3,
        a: 0xFF,
    }, //0X13
    Color {
        r: 0xBF,
        g: 0x00,
        b: 0xBF,
        a: 0xFF,
    }, //0X14
    Color {
        r: 0xE7,
        g: 0x00,
        b: 0x5B,
        a: 0xFF,
    }, //0X15
    Color {
        r: 0xDB,
        g: 0x2B,
        b: 0x00,
        a: 0xFF,
    }, //0X16
    Color {
        r: 0xCB,
        g: 0x4F,
        b: 0x0F,
        a: 0xFF,
    }, //0X17
    Color {
        r: 0x8B,
        g: 0x73,
        b: 0x00,
        a: 0xFF,
    }, //0X18
    Color {
        r: 0x00,
        g: 0x97,
        b: 0x00,
        a: 0xFF,
    }, //0X19
    Color {
        r: 0x00,
        g: 0xAB,
        b: 0x00,
        a: 0xFF,
    }, //0X1A
    Color {
        r: 0x00,
        g: 0x93,
        b: 0x3B,
        a: 0xFF,
    }, //0X1B
    Color {
        r: 0x00,
        g: 0x83,
        b: 0x8B,
        a: 0xFF,
    }, //0X1C
    Color {
        r: 0x00,
        g: 0x00,
        b: 0x00,
        a: 0xFF,
    }, //0X1D
    Color {
        r: 0x00,
        g: 0x00,
        b: 0x00,
        a: 0xFF,
    }, //0X1E
    Color {
        r: 0x00,
        g: 0x00,
        b: 0x00,
        a: 0xFF,
    }, //0X1F
    Color {
        r: 0xFF,
        g: 0xFF,
        b: 0xFF,
        a: 0xFF,
    }, //0X20
    Color {
        r: 0x3F,
        g: 0xBF,
        b: 0xFF,
        a: 0xFF,
    }, //0X21
    Color {
        r: 0x5F,
        g: 0x97,
        b: 0xFF,
        a: 0xFF,
    }, //0X22
    Color {
        r: 0xA7,
        g: 0x8B,
        b: 0xFD,
        a: 0xFF,
    }, //0X23
    Color {
        r: 0xF7,
        g: 0x7B,
        b: 0xFF,
        a: 0xFF,
    }, //0X24
    Color {
        r: 0xFF,
        g: 0x77,
        b: 0xB7,
        a: 0xFF,
    }, //0X25
    Color {
        r: 0xFF,
        g: 0x77,
        b: 0x63,
        a: 0xFF,
    }, //0X26
    Color {
        r: 0xFF,
        g: 0x9B,
        b: 0x3B,
        a: 0xFF,
    }, //0X27
    Color {
        r: 0xF3,
        g: 0xBF,
        b: 0x3F,
        a: 0xFF,
    }, //0X28
    Color {
        r: 0x83,
        g: 0xD3,
        b: 0x13,
        a: 0xFF,
    }, //0X29
    Color {
        r: 0x4F,
        g: 0xDF,
        b: 0x4B,
        a: 0xFF,
    }, //0X2A
    Color {
        r: 0x58,
        g: 0xF8,
        b: 0x98,
        a: 0xFF,
    }, //0X2B
    Color {
        r: 0x00,
        g: 0xEB,
        b: 0xDB,
        a: 0xFF,
    }, //0X2C
    Color {
        r: 0x00,
        g: 0x00,
        b: 0x00,
        a: 0xFF,
    }, //0X2D
    Color {
        r: 0x00,
        g: 0x00,
        b: 0x00,
        a: 0xFF,
    }, //0X2E
    Color {
        r: 0x00,
        g: 0x00,
        b: 0x00,
        a: 0xFF,
    }, //0X2F
    Color {
        r: 0xFF,
        g: 0xFF,
        b: 0xFF,
        a: 0xFF,
    }, //0X30
    Color {
        r: 0xAB,
        g: 0xE7,
        b: 0xFF,
        a: 0xFF,
    }, //0X31
    Color {
        r: 0xC7,
        g: 0xD7,
        b: 0xFF,
        a: 0xFF,
    }, //0X32
    Color {
        r: 0xD7,
        g: 0xCB,
        b: 0xFF,
        a: 0xFF,
    }, //0X33
    Color {
        r: 0xFF,
        g: 0xC7,
        b: 0xFF,
        a: 0xFF,
    }, //0X34
    Color {
        r: 0xFF,
        g: 0xC7,
        b: 0xDB,
        a: 0xFF,
    }, //0X35
    Color {
        r: 0xFF,
        g: 0xBF,
        b: 0xB3,
        a: 0xFF,
    }, //0X36
    Color {
        r: 0xFF,
        g: 0xDB,
        b: 0xAB,
        a: 0xFF,
    }, //0X37
    Color {
        r: 0xFF,
        g: 0xE7,
        b: 0xA3,
        a: 0xFF,
    }, //0X38
    Color {
        r: 0xE3,
        g: 0xFF,
        b: 0xA3,
        a: 0xFF,
    }, //0X39
    Color {
        r: 0xAB,
        g: 0xF3,
        b: 0xBF,
        a: 0xFF,
    }, //0X3A
    Color {
        r: 0xB3,
        g: 0xFF,
        b: 0xCF,
        a: 0xFF,
    }, //0X3B
    Color {
        r: 0x9F,
        g: 0xFF,
        b: 0xF3,
        a: 0xFF,
    }, //0X3C
    Color {
        r: 0x00,
        g: 0x00,
        b: 0x00,
        a: 0xFF,
    }, //0X3D
    Color {
        r: 0x00,
        g: 0x00,
        b: 0x00,
        a: 0xFF,
    }, //0X3E
    Color {
        r: 0x00,
        g: 0x00,
        b: 0x00,
        a: 0xFF,
    }, //0X3F
];

pub struct Ppu {
    pub ppuctrl: u8,
    pub ppumask: u8,
    pub ppustatus: u8,
    pub oamaddr: u8,
    pub oamdata: u8,
    pub ppudata_buffer: u8,
    pub ppuscroll: u16, // also called t, or temporary vram address, in docs.
    pub ppuaddr: u16,   // also called v, or current vram address, in docs
    pub fine_x: u8,     // fine x scroll of the ppu
    pub two_write_partial: bool,
    pub vram: [u8; 0x4000],
    pub oam: [u8; 0x100],
    pub canvas: Canvas<Window>,
    pub scanline: u16,
    pub cycle: u16,
    pub nmi_waiting: bool,
    pub even_frame: bool,
    pub pattern_table_shift_low: u16, // the low byte of this is where the parallel input is "shifted" in (latched)
    pub pattern_table_shift_high: u16,
    pub attribute_table_palette_shift_low: u8,
    pub attribute_table_palette_latch_low: bool,
    pub attribute_table_palette_shift_high: u8,
    pub attribute_table_palette_latch_high: bool,
    pub decoded_nametable_byte: u8,
    pub decoded_attribute_table_bit_high: bool,
    pub decoded_attribute_table_bit_low: bool,
    pub decoded_pattern_table_low: u8,
    pub decoded_pattern_table_high: u8,
    pub mirror_type: rom::MirroringType,
}

impl Ppu {
    pub fn new(video_subsystem: &sdl2::VideoSubsystem) -> Self {
        let window = video_subsystem
            .window("NES Terminal Window", 256 * SCALE, 240 * SCALE)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        Self {
            ppuctrl: 0x0,
            ppumask: 0x0,
            ppustatus: 0x0,
            oamaddr: 0x0,
            oamdata: 0x0,
            ppudata_buffer: 0x0,
            ppuscroll: 0x0,
            ppuaddr: 0x0,
            fine_x: 0x0,
            two_write_partial: false,
            vram: [0; 0x4000],
            oam: [0; 0x100],
            canvas,
            scanline: 0x0,
            cycle: 0x0,
            nmi_waiting: false,
            even_frame: false,
            pattern_table_shift_low: 0,
            pattern_table_shift_high: 0,
            attribute_table_palette_shift_low: 0,
            attribute_table_palette_latch_low: false,
            attribute_table_palette_shift_high: 0,
            attribute_table_palette_latch_high: false,
            decoded_nametable_byte: 0,
            decoded_attribute_table_bit_high: false,
            decoded_attribute_table_bit_low: false,
            decoded_pattern_table_low: 0,
            decoded_pattern_table_high: 0,
            mirror_type: rom::MirroringType::FourScreen,
        }
    }

    pub fn get_vram_byte_at(&self, addr: u16) -> u8 {
        let mut actual_addr = addr % 0x4000;

        // palettes are mirrored from 0x3F00 to 0x4000 every 0x20 bytes
        if actual_addr >= 0x3F00 && actual_addr < 0x4000 {
            actual_addr = ((actual_addr - 0x3F00) % 0x20) + 0x3F00;
        }

        // Data at addresses 0x3000-0x3EFF mirrors 0x2000-0x2EFF
        if actual_addr >= 0x3000 && actual_addr < 0x3F00 {
            actual_addr -= 0x1000;
        }

        if actual_addr >= 0x2000 && actual_addr < 0x3000 {
            match self.mirror_type {
                rom::MirroringType::FourScreen => actual_addr &= !0x0C00,
                rom::MirroringType::Horizontal => actual_addr &= !0x0400,
                rom::MirroringType::Vertical => actual_addr &= !0x0800,
            };
        }

        // Mirror 0x3F0{0,4,8,C} at 0x3F1{0,4,8,C}
        if actual_addr >= 0x3F00 && actual_addr < 0x4000 && actual_addr % 0x4 == 0 {
            actual_addr = ((actual_addr - 0x3F00) % 0x10) + 0x3F00;
        }

        self.vram[usize::from(actual_addr)]
    }

    pub fn set_vram_byte_at(&mut self, addr: u16, val: u8) {
        let mut actual_addr = addr % 0x4000;

        // palettes are mirrored from 0x3F00 to 0x4000 every 0x20 bytes
        if actual_addr >= 0x3F00 {
            actual_addr = ((actual_addr - 0x3F00) % 0x20) + 0x3F00;
        }

        // Data at addresses 0x3000-0x3EFF mirrors 0x2000-0x2EFF
        if actual_addr >= 0x3000 && actual_addr < 0x3F00 {
            actual_addr -= 0x1000;
        }

        if actual_addr >= 0x2000 && actual_addr < 0x3000 {
            match self.mirror_type {
                rom::MirroringType::FourScreen => actual_addr &= !0x0C00,
                rom::MirroringType::Horizontal => actual_addr &= !0x0400,
                rom::MirroringType::Vertical => actual_addr &= !0x0800,
            };
        }

        // Mirror 0x3F0{0,4,8,C} at 0x3F1{0,4,8,C}
        if actual_addr >= 0x3F00 && actual_addr < 0x4000 && actual_addr % 0x4 == 0 {
            actual_addr = ((actual_addr - 0x3F00) % 0x10) + 0x3F00;
        }

        self.vram[usize::from(actual_addr)] = val;
    }

    pub fn get_oam_byte_at(&mut self, addr: u8) -> u8 {
        self.oam[usize::from(addr)]
    }

    pub fn set_oam_byte_at(&mut self, addr: u8, val: u8) {
        self.oam[usize::from(addr)] = val;
    }

    fn get_current_pixel(&mut self, scanline: u16, dot: u16) -> Color {
        let palette_x_offset = 7 - self.fine_x; // fine_x of 0 means we want the highest bit of 8-bit attribute_table_palette_shift_{high,low}
        let pattern_x_offset = 15 - self.fine_x; // fine_x of 0 means we actually want the highest bit of 16-bit pattern_table_shift_{high,low}

        // Bits from left->right (bit_1 bit_2 bit_3 bit_4)
        let bit_1 = (self.attribute_table_palette_shift_high >> palette_x_offset) & 0x1;
        let bit_2 = (self.attribute_table_palette_shift_low >> palette_x_offset) & 0x1;
        let bit_3 =
            u8::try_from((self.pattern_table_shift_high >> pattern_x_offset) & 0x1).unwrap();
        let bit_4 = u8::try_from((self.pattern_table_shift_low >> pattern_x_offset) & 0x1).unwrap();

        // If background rendering enabled, get the background pattern offset here. Otherwise,
        // the offset will always be 0 (for the default background).
        let background_pattern_final =
            if self.ppumask & (1 << 3) != 0 && !(bit_3 == 0 && bit_4 == 0) {
                (bit_1 << 3) | (bit_2 << 2) | (bit_3 << 1) | bit_4
            } else {
                0
            };

        if scanline % 8 == 0 || dot % 8 == 0 {
            //return PALETTE[0];
        }

        let background_palette_idx =
            self.get_vram_byte_at(0x3F00 + u16::from(background_pattern_final));

        let square_sprites = (self.ppuctrl & 0x20) >> 5 == 0;

        for sprite_idx in 0..64usize {
            let sprite_y = self.oam[sprite_idx * 4] as u16 + 1; // "Sprite data is delayed by one scanline; you must subtract 1 from the sprite's Y coordinate before writing it here."
            let pattern_idx = self.oam[sprite_idx * 4 + 1]; // As such, we must add 1 to the sprite's stored Y address to compensate for this.
            let attributes = self.oam[sprite_idx * 4 + 2];
            let sprite_x = self.oam[sprite_idx * 4 + 3] as u16;

            if square_sprites
                && dot >= sprite_x
                && dot < (sprite_x + 8)
                && scanline >= sprite_y
                && scanline < (sprite_y + 8)
            {
                let sprite_height_offset = if attributes & (1 << 7) == 0 {
                    scanline - sprite_y
                } else {
                    7 - (scanline - sprite_y)
                };

                let sprite_pattern_start =
                    ((u16::from(self.ppuctrl) << 9) & 0x1000) | (u16::from(pattern_idx) << 4);

                let pattern_0 = self.get_vram_byte_at(sprite_pattern_start + sprite_height_offset);
                let pattern_1 =
                    self.get_vram_byte_at(sprite_pattern_start + sprite_height_offset + 8);

                // We use this to calculate the offset into this "strip" of sprite data (the sprite
                // pos along the x-axis). By default, this is simply actual x - sprite start x.
                // In the case where the sprite is horizontally flipped (bit 7 of attributes is
                // active), we need to flip this value.
                let sprite_strip_offset = if attributes & (1 << 6) == 0 {
                    7 - (dot - sprite_x)
                } else {
                    dot - sprite_x
                };

                // bit offset into pattern_0 and pattern_1 are overlaid
                let pattern_low = ((pattern_0 >> sprite_strip_offset) & 0x1)
                    | (((pattern_1 >> sprite_strip_offset) & 0x1) << 1);

                let pattern_high = attributes & 0b00000011;
                let pattern_final = pattern_low | (pattern_high << 2);

                // If the low bits 2 bits of the sprite idx (just the bits derived from the
                // pattern), the sprite at this point is transparent.
                if pattern_low == 0 {
                    continue;
                }

                // If we hit a sprite, set sprite 0 hit
                if sprite_idx == 0 && pattern_low != 0 && background_pattern_final != 0 {
                    self.ppustatus |= 1 << 6;
                }

                let palette_idx = self.get_vram_byte_at(0x3F10 + u16::from(pattern_final));

                return PALETTE[usize::from(palette_idx)];
            } else if !square_sprites
                && dot >= sprite_x
                && dot < (sprite_x + 8)
                && scanline >= sprite_y
                && scanline < (sprite_y + 16)
            {
                let sprite_height_offset = if attributes & (1 << 7) == 0 {
                    scanline - sprite_y
                } else {
                    15 - (scanline - sprite_y)
                };

                let sprite_pattern_start = ((u16::from(pattern_idx) << 12) & 0x1000)
                    | ((u16::from(pattern_idx) << 4) & 0x0FE0)
                    | ((sprite_height_offset << 1) & 0x10);

                let pattern_0 =
                    self.get_vram_byte_at(sprite_pattern_start + (sprite_height_offset & 0x7));
                let pattern_1 =
                    self.get_vram_byte_at(sprite_pattern_start + (sprite_height_offset & 0x7) + 8);

                // We use this to calculate the offset into this "strip" of sprite data (the sprite
                // pos along the x-axis). By default, this is simply actual x - sprite start x.
                // In the case where the sprite is horizontally flipped (bit 7 of attributes is
                // active), we need to flip this value.
                let sprite_strip_offset = if attributes & (1 << 6) == 0 {
                    7 - (dot - sprite_x)
                } else {
                    dot - sprite_x
                };

                // bit offset into pattern_0 and pattern_1 are overlaid
                let pattern_low = ((pattern_0 >> sprite_strip_offset) & 0x1)
                    | (((pattern_1 >> sprite_strip_offset) & 0x1) << 1);

                let pattern_high = attributes & 0b00000011;
                let pattern_final = pattern_low | (pattern_high << 2);

                // If the low bits 2 bits of the sprite idx (just the bits derived from the
                // pattern), the sprite at this point is transparent.
                if pattern_low == 0 {
                    continue;
                }

                // If we hit a sprite, set sprite 0 hit
                if sprite_idx == 0 && pattern_low != 0 && background_pattern_final != 0 {
                    self.ppustatus |= 1 << 6;
                }

                let palette_idx = self.get_vram_byte_at(0x3F10 + u16::from(pattern_final));

                return PALETTE[usize::from(palette_idx)];
            }
        }

        PALETTE[usize::from(background_palette_idx)]
    }

    // pre-render scanline happens at 261
    // dot 0 is cycle 0
    pub fn step(&mut self) -> bool {
        // If we're at the part of the screen to be rendering:
        if self.scanline <= 239 {
            if self.cycle >= 2 && self.cycle <= 257 {
                /* Psuedo-draw */

                let dot = self.cycle - 2;
                let curr_pixel_color = self.get_current_pixel(self.scanline, dot);

                self.canvas.set_draw_color(curr_pixel_color);
                self.canvas
                    .fill_rect(Rect::new(
                        dot as i32 * SCALE as i32,
                        self.scanline as i32 * SCALE as i32,
                        SCALE,
                        SCALE,
                    ))
                    .unwrap();
            }
        }

        // If rendering is enabled:
        if self.ppumask & 0x18 != 0 {
            // We only make memory accesses to PPU when rendering is active and on scanline 0-239
            // or 261 (pre-render scanline)
            if self.scanline <= 239 || self.scanline == 261 {
                if (self.cycle >= 2 && self.cycle <= 257)
                    || (self.cycle >= 322 && self.cycle <= 337)
                {
                    self.pattern_table_shift_low <<= 1;
                    self.pattern_table_shift_high <<= 1;

                    // If latch is set, make sure we set the bit that would be shifted in
                    self.attribute_table_palette_shift_low <<= 1;
                    if self.attribute_table_palette_latch_low {
                        self.attribute_table_palette_shift_low |= 0x01;
                    }

                    self.attribute_table_palette_shift_high <<= 1;
                    if self.attribute_table_palette_latch_high {
                        self.attribute_table_palette_shift_high |= 0x01;
                    }

                    if self.cycle % 8 == 0 {
                        self.coarse_x_increment();
                        self.decode_pattern_table_high();
                    } else if self.cycle % 8 == 1 {
                        self.reload_shift_registers();
                    } else if self.cycle % 8 == 3 {
                        self.decode_nametable_byte();
                    } else if self.cycle % 8 == 4 {
                        self.decode_attribute_table_byte();
                    } else if self.cycle % 8 == 6 {
                        self.decode_pattern_table_low();
                    }
                }
            }

            if self.scanline <= 239 || self.scanline == 261 {
                if self.cycle == 256 {
                    self.fine_y_increment();
                } else if self.cycle == 257 {
                    self.ppuaddr &= !0x041F; // assign all bits related to horizontal position from ppuscroll to ppuaddr
                    self.ppuaddr |= self.ppuscroll & 0x041F;

                    // Update PPUCTRL nametable select to keep in sync
                    self.ppuctrl &= !0x1;
                    self.ppuctrl |= ((self.ppuaddr >> 10) & 0x1) as u8;
                }
            }

            if self.scanline == 261 {
                if self.cycle >= 280 && self.cycle <= 304 {
                    // Reload vertical scroll bits
                    self.ppuaddr &= !0x7BE0;
                    self.ppuaddr |= self.ppuscroll & 0x7BE0;

                    // Update PPUCTRL nametable select to keep in sync
                    self.ppuctrl &= !0x2;
                    self.ppuctrl |= ((self.ppuaddr >> 10) & 0x3) as u8;
                } else if self.cycle == 339 && !self.even_frame {
                    // On odd frames, we skip right from (339, 261) to (0, 0) -> skip a cycle
                    self.cycle += 1;
                }
            }
        }

        if self.cycle == 1 {
            if self.scanline == 241 {
                self.ppustatus |= 1 << 7; // set vblank at cycle 1 of scanline 241
                self.nmi_waiting = (self.ppuctrl >> 7) & 0x1 != 0; //Nmi only occurs on vblank if ppuctrl bit 7 is set
            } else if self.scanline == 261 {
                self.ppustatus &= !(1 << 6); // clear sprite 0 hit at cycle 1 of scaline 261 (pre-render line)
                self.ppustatus &= !(1 << 7); // clear vblank at cycle 1 of scanline 261 (pre-render line)
            }
        }

        // OAMADDR gets set to 0 during ticks 257-320 of pre-render and visible scanlines
        if (self.scanline == 261 || self.scanline < 240) && (self.cycle >= 257 && self.cycle <= 320)
        {
            self.oamaddr = 0;
        }

        // Handle actually incrementing cycles and scanlines
        self.cycle += 1;

        if self.cycle > 340 {
            self.cycle = 0;
            self.scanline += 1;
            if self.scanline > 261 {
                self.scanline = 0;
                self.even_frame = !self.even_frame;
                self.canvas.present();
                return true;
            }
        }

        false
    }

    fn reload_shift_registers(&mut self) {
        // Clear the low 8 bits of the shift registers.
        self.pattern_table_shift_low &= 0xFF00;
        self.pattern_table_shift_high &= 0xFF00;

        // Set the low 8 bits of the pattern table shift registers.
        self.pattern_table_shift_low |= u16::from(self.decoded_pattern_table_low);
        self.pattern_table_shift_high |= u16::from(self.decoded_pattern_table_high);

        // Set the low and high shift registers from the corresponding data (single bit latch ->
        // 8-bit shift register).
        self.attribute_table_palette_latch_low = self.decoded_attribute_table_bit_low;
        self.attribute_table_palette_latch_high = self.decoded_attribute_table_bit_high;
    }

    fn coarse_x_increment(&mut self) {
        let mut coarse_x = self.ppuaddr & 0x1F; // extract coarse x
        if coarse_x == 31 {
            coarse_x = 0;
            self.ppuaddr ^= 0x400; // swap nametable between possible x locations (either left or right)
        } else {
            coarse_x += 1;
        }

        self.ppuaddr = (self.ppuaddr & !0x1F) | coarse_x; // put coarse_x back into ppuaddr
    }

    fn fine_y_increment(&mut self) {
        if self.ppuaddr & 0x7000 != 0x7000 {
            // if fine y < 7
            self.ppuaddr += 0x1000; // bump fine y by adding 0x1000 to ppuaddr (we can safely do this, because fine y < 7)
        } else {
            self.ppuaddr &= !0x7000; // reset fine y to 0
            let mut coarse_y = (self.ppuaddr & 0x3E0) >> 5; // extract coarse y value from ppuaddr (bits 6-10)
            if coarse_y == 29 {
                coarse_y = 0; // reset coarse y to 0
                self.ppuaddr ^= 0x800; // swap nametable value between possible y locations (either top or bottom)
            } else if coarse_y == 31 {
                coarse_y = 0; // assign coarse_y to 0 still, but don't switch nametable. We get here when only coarse_y gets manually set.
            } else {
                coarse_y += 1; // otherwise, just increment coarse y
            }

            self.ppuaddr = (self.ppuaddr & !0x3E0) | (coarse_y << 5); // put coarse_y back into ppuaddr
        }
    }

    fn decode_nametable_byte(&mut self) {
        let nametable_byte_addr = 0x2000 | (self.ppuaddr & 0xFFF); //coarse x scroll, coarse y scroll, and nametable select are all we need for address of tile

        self.decoded_nametable_byte = self.get_vram_byte_at(nametable_byte_addr);
    }

    fn decode_attribute_table_byte(&mut self) {
        let attribute_byte_addr = 0x23C0
            | (self.ppuaddr & 0x0C00)
            | ((self.ppuaddr >> 4) & 0x38)
            | ((self.ppuaddr >> 2) & 0x07); // mangle portions of ppuaddr to form the attribute address

        let decoded_attribute_table_byte = self.get_vram_byte_at(attribute_byte_addr);

        // -------
        // |00|10|
        // -------
        // |01|11|
        // -------
        let tile_attribute_coord_x = (self.ppuaddr & 0x2) >> 1;
        let tile_attribute_coord_y = (self.ppuaddr & 0x40) >> 6;

        let pattern_high = match (tile_attribute_coord_x, tile_attribute_coord_y) {
            (0, 0) => decoded_attribute_table_byte & 0x3,
            (1, 0) => (decoded_attribute_table_byte >> 2) & 0x3,
            (0, 1) => (decoded_attribute_table_byte >> 4) & 0x3,
            (1, 1) => (decoded_attribute_table_byte >> 6) & 0x3,
            _ => unreachable!(),
        };

        // Set the low and high decoded attribute table bits (will be loaded into latches when
        // shift registers reload).
        self.decoded_attribute_table_bit_low = pattern_high & 0x1 != 0;
        self.decoded_attribute_table_bit_high = pattern_high & 0x2 != 0;
    }

    fn decode_pattern_table_low(&mut self) {
        let background_pattern_table_base = if self.ppuctrl & (1 << 4) == 0 {
            0x0000
        } else {
            0x1000
        };

        let fine_y = (self.ppuaddr & 0x7000) >> 12; // extract fine y value for current pixel

        self.decoded_pattern_table_low = self.get_vram_byte_at(
            background_pattern_table_base + (u16::from(self.decoded_nametable_byte) * 16) + fine_y,
        );
        // Set the low and high shift registers from the corresponding strips of pixels
    }

    fn decode_pattern_table_high(&mut self) {
        let background_pattern_table_base = if self.ppuctrl & (1 << 4) == 0 {
            0x0000
        } else {
            0x1000
        };

        let fine_y = (self.ppuaddr & 0x7000) >> 12; // extract fine y value for current pixel

        self.decoded_pattern_table_high = self.get_vram_byte_at(
            background_pattern_table_base
                + (u16::from(self.decoded_nametable_byte) * 16)
                + fine_y
                + 8,
        );
    }
}
