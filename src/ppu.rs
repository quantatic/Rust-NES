use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

const SCALE: u32 = 2;

pub const PALETTE: [Color; 0x40] = [
    Color { r: 0x75, g: 0x75, b: 0x75, a: 0xFF }, //0x00
    Color { r: 0x27, g: 0x1B, b: 0x8F, a: 0xFF }, //0x01
    Color { r: 0x00, g: 0x00, b: 0xAB, a: 0xFF }, //0x02
    Color { r: 0x47, g: 0x00, b: 0x9F, a: 0xFF }, //0x03
    Color { r: 0x8F, g: 0x00, b: 0x77, a: 0xFF }, //0x04
    Color { r: 0xAB, g: 0x00, b: 0x13, a: 0xFF }, //0x05
    Color { r: 0xA7, g: 0x00, b: 0x00, a: 0xFF }, //0x06
    Color { r: 0x7F, g: 0x0B, b: 0x00, a: 0xFF }, //0x07
    Color { r: 0x43, g: 0x2F, b: 0x00, a: 0xFF }, //0X08
    Color { r: 0x00, g: 0x47, b: 0x00, a: 0xFF }, //0X09
    Color { r: 0x00, g: 0x51, b: 0x00, a: 0xFF }, //0X0A
    Color { r: 0x00, g: 0x3F, b: 0x17, a: 0xFF }, //0X0B
    Color { r: 0x1B, g: 0x3F, b: 0x5F, a: 0xFF }, //0X0C
    Color { r: 0x00, g: 0x00, b: 0x00, a: 0xFF }, //0X0D
    Color { r: 0x00, g: 0x00, b: 0x00, a: 0xFF }, //0X0E
    Color { r: 0x00, g: 0x00, b: 0x00, a: 0xFF }, //0X0F
    Color { r: 0xBC, g: 0xBC, b: 0xBC, a: 0xFF }, //0X10
    Color { r: 0x00, g: 0x73, b: 0xEF, a: 0xFF }, //0X11
    Color { r: 0x23, g: 0x3B, b: 0xEF, a: 0xFF }, //0X12
    Color { r: 0x83, g: 0x00, b: 0xF3, a: 0xFF }, //0X13
    Color { r: 0xBF, g: 0x00, b: 0xBF, a: 0xFF }, //0X14
    Color { r: 0xE7, g: 0x00, b: 0x5B, a: 0xFF }, //0X15
    Color { r: 0xDB, g: 0x2B, b: 0x00, a: 0xFF }, //0X16
    Color { r: 0xCB, g: 0x4F, b: 0x0F, a: 0xFF }, //0X17
    Color { r: 0x8B, g: 0x73, b: 0x00, a: 0xFF }, //0X18
    Color { r: 0x00, g: 0x97, b: 0x00, a: 0xFF }, //0X19
    Color { r: 0x00, g: 0xAB, b: 0x00, a: 0xFF }, //0X1A
    Color { r: 0x00, g: 0x93, b: 0x3B, a: 0xFF }, //0X1B
    Color { r: 0x00, g: 0x83, b: 0x8B, a: 0xFF }, //0X1C
    Color { r: 0x00, g: 0x00, b: 0x00, a: 0xFF }, //0X1D
    Color { r: 0x00, g: 0x00, b: 0x00, a: 0xFF }, //0X1E
    Color { r: 0x00, g: 0x00, b: 0x00, a: 0xFF }, //0X1F
    Color { r: 0xFF, g: 0xFF, b: 0xFF, a: 0xFF }, //0X20
    Color { r: 0x3F, g: 0xBf, b: 0xFF, a: 0xFF }, //0X21
    Color { r: 0x5F, g: 0x97, b: 0xFF, a: 0xFF }, //0X22
    Color { r: 0xA7, g: 0x8B, b: 0xFD, a: 0xFF }, //0X23
    Color { r: 0xF7, g: 0x7B, b: 0xFF, a: 0xFF }, //0X24
    Color { r: 0xFF, g: 0x77, b: 0xB7, a: 0xFF }, //0X25
    Color { r: 0xFF, g: 0x77, b: 0x63, a: 0xFF }, //0X26
    Color { r: 0xFF, g: 0x9B, b: 0x3B, a: 0xFF }, //0X27
    Color { r: 0xF3, g: 0xBF, b: 0x3F, a: 0xFF }, //0X28
    Color { r: 0x83, g: 0xD3, b: 0x13, a: 0xFF }, //0X29
    Color { r: 0x4F, g: 0xDF, b: 0x4B, a: 0xFF }, //0X2A
    Color { r: 0x58, g: 0xF8, b: 0x98, a: 0xFF }, //0X2B
    Color { r: 0x00, g: 0xEB, b: 0xDB, a: 0xFF }, //0X2C
    Color { r: 0x00, g: 0x00, b: 0x00, a: 0xFF }, //0X2D
    Color { r: 0x00, g: 0x00, b: 0x00, a: 0xFF }, //0X2E
    Color { r: 0x00, g: 0x00, b: 0x00, a: 0xFF }, //0X2F
    Color { r: 0xFF, g: 0xFF, b: 0xFF, a: 0xFF }, //0X30
    Color { r: 0xAB, g: 0xE7, b: 0xFF, a: 0xFF }, //0X31
    Color { r: 0xC7, g: 0xD7, b: 0xFF, a: 0xFF }, //0X32
    Color { r: 0xD7, g: 0xCB, b: 0xFF, a: 0xFF }, //0X33
    Color { r: 0xFF, g: 0xC7, b: 0xFF, a: 0xFF }, //0X34
    Color { r: 0xFF, g: 0xC7, b: 0xDB, a: 0xFF }, //0X35
    Color { r: 0xFF, g: 0xBF, b: 0xB3, a: 0xFF }, //0X36
    Color { r: 0xFF, g: 0xDB, b: 0xAB, a: 0xFF }, //0X37
    Color { r: 0xFF, g: 0xE7, b: 0xA3, a: 0xFF }, //0X38
    Color { r: 0xE3, g: 0xFF, b: 0xA3, a: 0xFF }, //0X39
    Color { r: 0xAB, g: 0xF3, b: 0xBF, a: 0xFF }, //0X3A
    Color { r: 0xB3, g: 0xFF, b: 0xCF, a: 0xFF }, //0X3B
    Color { r: 0x9F, g: 0xFF, b: 0xF3, a: 0xFF }, //0X3C
    Color { r: 0x00, g: 0x00, b: 0x00, a: 0xFF }, //0X3D
    Color { r: 0x00, g: 0x00, b: 0x00, a: 0xFF }, //0X3E
    Color { r: 0x00, g: 0x00, b: 0x00, a: 0xFF }, //0X3F
];

pub struct Ppu {
    pub ppuctrl: u8,
    pub ppumask: u8,
    pub ppustatus: u8,
    pub oamaddr: u8,
    pub oamdata: u8,
	pub ppudata_buffer: u8,
    pub ppuscroll: u16,
    pub ppuaddr: u16,
    pub two_write_partial: bool,
    pub vram: [u8; 0x4000],
    pub oam: [u8; 0x100],
    pub canvas: Canvas<Window>,
    pub scanline: u16,
    pub cycle: u16,
    pub nmi_waiting: bool,
	pub tmp: u32
}

impl Ppu {
    pub fn new(video_subsystem: &sdl2::VideoSubsystem) -> Self {
        let window = video_subsystem
            .window("NES Terminal Window", 256 * SCALE, 240 * SCALE)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window
            .into_canvas()
            .build()
            .unwrap();

        Self {
            ppuctrl: 0x0,
            ppumask: 0x0,
            ppustatus: 0x0,
            oamaddr: 0x0,
            oamdata: 0x0,
			ppudata_buffer: 0x0,
            ppuscroll: 0x0,
            ppuaddr: 0x0,
            two_write_partial: false,
            vram: [0; 0x4000],
            oam: [0; 0x100],
            canvas,
            scanline: 0x0,
            cycle: 0x0,
            nmi_waiting: false,
			tmp: 0
        }
    }

    pub fn get_vram_byte_at(&mut self, addr: u16) -> u8 {
        let mut actual_addr = addr % 0x4000;

        // palettes are mirrored from 0x3F00 to 0x4000 every 0x20 bytes
        if actual_addr >= 0x3F00 && actual_addr < 0x4000 {
            actual_addr = ((actual_addr - 0x3F00) % 0x20) + 0x3F00;
        }

		// Assume we're doing full nametable mirroring (for now)
        if actual_addr >= 0x2000 && actual_addr < 0x3000 {
			actual_addr = ((actual_addr - 0x2000) % 0x400) + 0x2000;
        }

		// Data at addresses 0x3000-0x3EFF mirrors 0x2000-0x2EFF
		if actual_addr >= 0x3000 && actual_addr < 0x3F00 {
			actual_addr -= 0x1000;
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

		// Assume we're doing full nametable mirroring (for now)
        if actual_addr >= 0x2000 && actual_addr < 0x3000 {
			actual_addr = ((actual_addr - 0x2000) % 0x400) + 0x2000;
        }

		// Mirror 0x3F0{0,4,8,C} at 0x3F1{0,4,8,C}
		if actual_addr >= 0x3F00 && actual_addr % 0x4 == 0 {
			let old_addr = actual_addr;
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

    fn get_pixel_at(&mut self, x: u16, y: u16) -> Color {
        let tile_x = x / 8;
        let tile_y = y / 8;
        let nametable_idx = u16::from(tile_x) + (u16::from(tile_y) * 32);
        let nametable_base = match self.ppuctrl & 0b00000011 {
            0x0 => 0x2000,
            0x1 => 0x2400,
            0x2 => 0x2800,
            0x3 => 0x2C00,
            _ => panic!(),
        };

        let pattern_idx = self.get_vram_byte_at(nametable_base + u16::from(nametable_idx));

		let sprite_pattern_table_base = if self.ppuctrl & (1 << 3) == 0 {
			0x0000
		} else {
			0x1000
		};

		let background_pattern_table_base = if self.ppuctrl & (1 << 4) == 0 {
			0x0000
		} else {
			0x1000
		};

		let sprite_size = self.ppuctrl & (1 << 5);

		let grayscale = self.ppumask & (1 << 0) != 0;
		if grayscale {
			panic!("don't know how to handle grayscale!");
		}

		if sprite_size != 0 {
			panic!("uh oh! don't know how to handle sprite size: {}", sprite_size);
		}

        let pattern_0 = self.get_vram_byte_at(background_pattern_table_base + (u16::from(pattern_idx) * 16) + u16::from(y % 8));
        let pattern_1 = self.get_vram_byte_at(background_pattern_table_base + (u16::from(pattern_idx) * 16) + u16::from(y % 8) + 8);

        // bit offset into pattern_0 and pattern_1 are overlaid

        let pattern_low = ((pattern_0 >> (7 - (x % 8))) & 0x1) |
            (((pattern_1 >> (7 - (x % 8))) & 0x1) << 1);

        let attribute_x = x / 32;
        let attribute_y = y / 32;
        let attribute_idx = u16::from(attribute_x + (attribute_y * 8));
        let attribute_data = self.get_vram_byte_at(0x23C0 + attribute_idx);
        // -------
        // |00|10|
        // -------
        // |01|11|
        // -------
        let pattern_high = match ((tile_x % 4) / 2, (tile_y % 4) / 2) {
            (0, 0) => (attribute_data >> 0) & 0b00000011,
            (1, 0) => (attribute_data >> 2) & 0b00000011,
            (0, 1) => (attribute_data >> 4) & 0b00000011,
            (1, 1) => (attribute_data >> 6) & 0b00000011,
            _ => panic!(),
        };

		// If background rendering enabled, get the background pattern offset here. Otherwise,
		// the offset will always be 0 (for the default background).
        let background_pattern_final = if self.ppumask & (1 << 3) != 0 {
			pattern_low | (pattern_high << 2)
		} else {
			0
		};

        let background_palette_idx = self.get_vram_byte_at(0x3F00 + u16::from(background_pattern_final));

        for sprite_idx in 0..64usize {
            let sprite_y = self.oam[sprite_idx * 4] as u16;
            let pattern_idx = self.oam[sprite_idx * 4 + 1];
            let attributes = self.oam[sprite_idx * 4 + 2];
            let sprite_x = self.oam[sprite_idx * 4 + 3] as u16;

            if x >= sprite_x && x < (sprite_x + 8) && y >= sprite_y && y < (sprite_y + 8) {
				let sprite_height_offset = if attributes & (1 << 7) == 0 {
					y - sprite_y
				} else {
					7 - (y - sprite_y)
				};

                let pattern_0 = self.get_vram_byte_at((u16::from(pattern_idx) * 16) + u16::from(sprite_height_offset));
                let pattern_1 = self.get_vram_byte_at((u16::from(pattern_idx) * 16) + u16::from(sprite_height_offset) + 8);

				// We use this to calculate the offset into this "strip" of sprite data (the sprite
				// pos along the x-axis). By default, this is simply actual x - sprite start x.
				// In the case where the sprite is horizontally flipped (bit 7 of attributes is
				// active), we need to flip this value.
				let sprite_strip_offset = if attributes & (1 << 6) == 0 {
					7 - (x - sprite_x)
				} else {
					x - sprite_x
				};

                // bit offset into pattern_0 and pattern_1 are overlaid
                let pattern_low = ((pattern_0 & (1 << sprite_strip_offset)) >> sprite_strip_offset) |
                                   (((pattern_1 & (1 << sprite_strip_offset)) >> sprite_strip_offset) << 1);

                let pattern_high = attributes & 0b00000011;
                let pattern_final = pattern_low | (pattern_high << 2);

				// If the low bits 2 bits of the sprite idx (just the bits derived from the
				// pattern), the sprite at this point is transparent.
				if pattern_low == 0 {
					continue;
				}

                let palette_idx = self.get_vram_byte_at(0x3F10 + u16::from(pattern_final));

                return PALETTE[usize::from(palette_idx)];
            }
        }

        PALETTE[usize::from(background_palette_idx)]
    }

    pub fn step(&mut self) {

		// Cycle is equal to dot + 1. 

		// If we're at the part of the screen to be rendered: render!
		if self.scanline < 240 {
			if self.cycle > 0 && self.cycle <= 256 {
				/* Psuedo-draw */
				let dot = self.cycle - 1;

				let curr_pixel_color = self.get_pixel_at(dot, self.scanline);
				self.canvas.set_draw_color(curr_pixel_color);
				self.canvas.fill_rect(Rect::new(dot as i32 * SCALE as i32, self.scanline as i32 * SCALE as i32, SCALE, SCALE)).unwrap();
			}
		}

		if self.cycle == 1 {
			if self.scanline == 241 {
				self.ppustatus |= (1 << 7); // set vblank at dot 1 of scanline 241
				self.nmi_waiting = (self.ppuctrl & (1 << 7)) != 0; //Nmi only occurs on vblank if ppuctrl bit 7 is set
			} else if self.scanline == 261 {
				self.ppustatus &= !(1 << 7); // clear vblank at dot 1 of scanline 261 (pre-render line)
			}
		}

		//self.tmp += 1;
		self.cycle += 1;

		if self.cycle > 340 {
			self.cycle = 0;
			self.scanline += 1;

			if self.scanline > 261 {
				self.scanline = 0;
				self.canvas.present();
				//println!("Last PPU frame was {} ticks!", self.tmp);
				//self.tmp = 0;
			}
		}

		// OAMADDR gets set to 0 during ticks 257-320 of pre-render and visible scanlines
		if (self.scanline == 261 || self.scanline <= 239) && (self.cycle >= 257 && self.cycle <= 320) {
			self.oamaddr = 0;
		}

    }
}
