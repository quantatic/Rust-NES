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
}
