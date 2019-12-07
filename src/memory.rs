use crate::rom::Rom;

pub struct Memory {
    ram: [u8; 0x2000],
    rom: Rom
}

impl Memory {
    pub fn new(rom: Rom) -> Self {
        Memory {
            ram: [0u8; 0x2000],
            rom
        }
    }

    pub fn get_byte_at(&self, addr: u16) -> u8 {
        match addr {
            // 0x0000-0x07FF mirrored at 0x0800, 0x1000, and 0x18000.
            0x0000..=0x1FFF => {
                let actual_addr = addr % 0x0800;
                self.ram[usize::from(actual_addr)]
            }
            0x8000..=0xFFFF => {
                let mut rom_access_addr = addr - 0x8000;
                if self.rom.prg_rom.len() <= 0x4000 { //if only one rom bank, should be mirrored
                    rom_access_addr %= 0x4000;
                }
                self.rom.prg_rom[usize::from(rom_access_addr)]
            },
            _ => 0x0,
        }
    }

    pub fn get_word_at(&self, addr: u16) -> u16 {
        let low = self.get_byte_at(addr);
        let high = self.get_byte_at(addr + 1);
        ((high as u16) << 8) | (low as u16)
    }

    pub fn set_byte_at(&mut self, addr: u16, val: u8) { }
    pub fn set_word_at(&mut self, addr: u16, val: u16) { }
}
