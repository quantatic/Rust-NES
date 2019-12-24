use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;

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
    pub ppuscroll: u16,
    pub ppuaddr: u16,
    pub two_write_partial: bool,
    pub vram: [u8; 0x4000],
    pub oam: [u8; 0x100],
    pub canvas: Canvas<Window>,
    pub events: sdl2::EventPump,
    pub scanline: u8,
    pub dot: u8,
    pub nmi_waiting: bool,
}

impl Ppu {
    pub fn new(sdl_context: &sdl2::Sdl) -> Self {
        let video_subsystem = sdl_context
            .video()
            .unwrap();

        let window = video_subsystem
            .window("NES Terminal Window", 256, 240)
            .position_centered()
            .build()
            .unwrap();

        let canvas = window
            .into_canvas()
            .build()
            .unwrap();

        let events = sdl_context
            .event_pump()
            .unwrap();

        Self {
            ppuctrl: 0x0,
            ppumask: 0x0,
            ppustatus: 0x0,
            oamaddr: 0x0,
            oamdata: 0x0,
            ppuscroll: 0x0,
            ppuaddr: 0x0,
            two_write_partial: false,
            vram: [0; 0x4000],
            oam: [0; 0x100],
            canvas,
            events,
            scanline: 0x0,
            dot: 0x0,
            nmi_waiting: false,
        }
    }

    pub fn get_vram_byte_at(&mut self, addr: u16) -> u8 {
        let mut actual_addr = addr % 0x4000;

        // palettes are mirrored from 0x3F00 to 0x4000 every 0x20 bytes
        if actual_addr >= 0x3F00 {
            actual_addr = ((actual_addr - 0x3F00) % 0x20) + 0x3F00;
        }

        self.vram[usize::from(actual_addr)]
    }

    pub fn set_vram_byte_at(&mut self, addr: u16, val: u8) {
        let mut actual_addr = addr % 0x4000;
        // palettes are mirrored from 0x3F00 to 0x4000 every 0x20 bytes

        if actual_addr >= 0x3F00 {
            actual_addr = ((actual_addr - 0x3F00) % 0x20) + 0x3F00;
        }

        if actual_addr >= 0x27C0 && actual_addr <= 0x27FF {
            actual_addr -= 0x0400;
        }

        self.vram[usize::from(actual_addr)] = val;
    }

    pub fn get_oam_byte_at(&mut self, addr: u8) -> u8 {
        self.oam[usize::from(addr)]
    }

    pub fn set_oam_byte_at(&mut self, addr: u8, val: u8) {
        self.oam[usize::from(addr)] = val;
    }

    pub fn check_for_exit(&mut self) {
        for event in self.events.poll_iter() {
            if let Event::Quit{ .. } = event {
                panic!("Exiting!");
            }
        }
    }

    fn get_pixel_at(&mut self, x: u8, y: u8) -> Color {
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

        let background_pattern_table_base = match (self.ppuctrl >> 4) & 0b00000001 {
            0x0 => 0x0000,
            0x1 => 0x1000,
            _ => panic!(),
        };

        let pattern_0 = self.get_vram_byte_at(background_pattern_table_base + (u16::from(pattern_idx) * 16) + u16::from(y % 8));
        let pattern_1 = self.get_vram_byte_at(background_pattern_table_base + (u16::from(pattern_idx) * 16) + u16::from(y % 8) + 8);

        // bit offset into pattern_0 and pattern_1 are overlaid

        let pattern_low = ((pattern_0 >> (7 - (x % 8))) & 0b00000001) |
            (((pattern_1 >> (7 - (x % 8))) & 0b00000001) << 1);

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

        let pattern_final = pattern_low | (pattern_high << 2);

        let palette_idx = self.get_vram_byte_at(0x3F00 + u16::from(pattern_final));

        let mut sprite_here = false;
        for sprite_idx in 0..16u8 {
            let sprite_y = self.oam[usize::from(sprite_idx * 4)];
            let pattern_idx = self.oam[usize::from(sprite_idx * 4 + 1)];
            let attributes = self.oam[usize::from(sprite_idx * 4 + 2)];
            let sprite_x = self.oam[usize::from(sprite_idx * 4 + 3)];

            if x >= sprite_x && x < (sprite_x + 8) && y >= sprite_y && y < (sprite_y + 8) {
                return PALETTE[0x15];
                let pattern_0 = self.get_vram_byte_at((u16::from(pattern_idx) * 16) + u16::from(y - sprite_y));
                let pattern_1 = self.get_vram_byte_at((u16::from(pattern_idx) * 16) + u16::from(y - sprite_y) + 8);

                // bit offset into pattern_0 and pattern_1 are overlaid
                let pattern_low = ((pattern_0 & (1 << (x - sprite_x))) >> (x - sprite_x)) |
                                   (((pattern_1 & (1 << (x - sprite_x))) >> (x - sprite_x)) << 1);
                let pattern_high = attributes & 0b00000011;
                let pattern_final = pattern_low | (pattern_high << 2);
                let palette_idx = self.get_vram_byte_at(0x3F10 + u16::from(pattern_final));

                return PALETTE[usize::from(palette_idx)];
            }
        }

        PALETTE[usize::from(palette_idx)]
    }

    pub fn step(&mut self) {
        /* Psuedo-draw */
        let curr_pixel_color = self.get_pixel_at(self.dot, self.scanline);
        self.canvas.set_draw_color(curr_pixel_color);
        self.canvas.fill_rect(Rect::new(self.dot as i32, self.scanline as i32, 1, 1)).unwrap();

        let (new_dot, hblank) = self.dot.overflowing_add(1);
        self.dot = new_dot;
        if hblank {
            self.scanline += 1;
            if self.scanline == 240 {
                //Nmi only occurs on vblank if ppuctrl bit 7 is set
                self.nmi_waiting = (self.ppuctrl & (1 << 7)) != 0;
                self.scanline = 0;
                self.canvas.present();
                self.ppustatus |= (1 << 7);
            }
        }

    }
}
