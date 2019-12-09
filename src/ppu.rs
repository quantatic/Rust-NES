pub struct Ppu {
    pub ctrl_1: u8,
    pub ctrl_2: u8,
    pub status: u8,
    pub spr_addr: u8,
    pub vram_1: u8,
    pub vram_2: u8,
    pub vram: [u8; 0x4000],
}

impl Ppu {
    pub fn new() -> Self {
        Self {
            ctrl_1: 0x0,
            ctrl_2: 0x0,
            status: 0x0,
            spr_addr: 0x0,
            vram_1: 0x0,
            vram_2: 0x0,
            vram: [0; 0x4000],
        }
    }
}
