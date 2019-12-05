use std::convert::TryFrom;
use std::fs::File;
use std::io::{Error, ErrorKind, Read};

pub struct Rom {
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
}

impl Rom {
    pub fn new(filename: &str) -> Result<Rom, std::io::Error> {
        let mut f = File::open(filename)?;

        // Assume header starts at "byte 0"
        let mut header: [u8; 16] = [0; 16];

        f.read_exact(&mut header)?;

        if header[0..4] != *b"NES\x1a" {
            return Err(Error::new(ErrorKind::InvalidData, "Rom had invalid header"));
        }

        let prg_rom_banks = header[4];
        let chr_rom_banks = header[5];
        let rom_ctrl_byte_1 = header[6];
        let rom_ctrl_byte_2 = header[7];
        let prg_ram_banks = std::cmp::min(header[8], 1); //assume 1 page exists even when 0

        if header[9..16] != [0; 7] {
            println!("{:?}", &header[9..16]);
            return Err(Error::new(ErrorKind::InvalidData, "Bytes 9-15 of header are not all 0"));
        }

        let prg_bytes = u32::try_from(prg_rom_banks).unwrap() * 16 * 1024;
        let chr_bytes = u32::try_from(chr_rom_banks).unwrap() * 8 * 1024;

        let mut prg_rom = vec![0u8; usize::try_from(prg_bytes).unwrap()];

        f.read_exact(&mut prg_rom)?;

        Ok(
            Rom {
                prg_rom,
                chr_rom: Vec::new(),
            }
        )
    }

    pub fn dump_opcodes(&self) {
        for &val in self.prg_rom.iter() {
            match val {
                0x69 | 0x65 | 0x75 | 0x6D | 0x7D | 0x79 | 0x61 | 0x71 => println!("ADD"),
                0x29 | 0x25 | 0x35 | 0x2D | 0x3D | 0x39 | 0x21 | 0x31 => println!("AND"),
                0x0A | 0x06 | 0x16 | 0x0E | 0x1E => println!("ASL"),
                0x90 => println!("BCC"),
                0xB0 => println!("BCS"),
                0xF0 => println!("BEQ"),
                0x24 | 0x2C => println!("BIT"),
                0x30 => println!("BMI"),
                0xD0 => println!("BNE"),
                0x10 => println!("BPL"),
                0x00 => println!("BRK"),
                0x50 => println!("BVC"),
                0x70 => println!("BVS"),
                0x18 => println!("CLC"),
                0xD8 => println!("CLD"),
                0x58 => println!("CLI"),
                0xB8 => println!("CLV"),
                0xC9 | 0xC5 | 0xD5 | 0xCD | 0xDD | 0xD9 | 0xC1 | 0xD1 => println!("CMP"),
                0xE0 | 0xE4 | 0xEC => println!("CPX"),
                0xC0 | 0xC4 | 0xCC => println!("CPY"),
                0xC6 | 0xD6 | 0xCE | 0xDE => println!("DEC"),
                0xCA => println!("DEX"),
                0x88 => println!("DEY"),
                0x49 | 0x45 | 0x55 | 0x4D | 0x5D | 0x59 | 0x41 | 0x51  => println!("EOR"),
                0xE6 | 0xF6 | 0xEE | 0xFE => println!("INC"),
                0xE8 => println!("INX"),
                0xC8 => println!("INY"),
                0x4C | 0x6C => println!("JMP"),
                0x20 => println!("JSR"),
                0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xB9 | 0xA1 | 0xB1 => println!("LDA"),
                0xA2 | 0xA6 | 0xB6 | 0xAE | 0xBE => println!("LDX"),
                0xA0 | 0xA4 | 0xB4 | 0xAc | 0xBC => println!("LDY"),
                0x4A | 0x46 | 0x56 | 0x4E | 0x5E => println!("LSR"),
                0xEA => println!("NOP"),
                0x09 | 0x05 | 0x15 | 0x0D | 0x1D | 0x19 | 0x01 | 0x11 => println!("ORA"),
                0x48 => println!("PHA"),
                0x08 => println!("PHP"),
                0x68 => println!("PLA"),
                0x28 => println!("PLP"),
                0x2A | 0x26 | 0x36 | 0x2E | 0x3E => println!("ROL"),
                0x6A | 0x66 | 0x76 | 0x6E | 0x7E => println!("ROR"),
                0x40 => println!("RTI"),
                0x60 => println!("RTS"),
                0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 => println!("SBC"),
                0x38 => println!("SEC"),
                0xF8 => println!("SED"),
                0x78 => println!("SEI"),
                0x85 | 0x95 | 0x8D | 0x9D | 0x99 | 0x81 | 0x91 => println!("STA"),
                0x86 | 0x96 | 0x8E => println!("STX"),
                0x84 | 0x94 | 0x8C => println!("STY"),
                0xAA => println!("TAX"),
                0xA8 => println!("TAY"),
                0xBA => println!("TSX"),
                0x8A => println!("TXA"),
                0x9A => println!("TXS"),
                0x98 => println!("TA"),
               _ => println!("{:08b}", val),
           }
       }
    }
}
