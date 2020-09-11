use std::convert::TryFrom;
use std::fs::File;
use std::io::{Error, ErrorKind, Read};

#[derive(Debug)]
pub struct Rom {
    pub prg_rom: Vec<u8>,
    pub chr_rom: Vec<u8>,
    pub mapper_number: u8,
    pub mirroring: MirroringType,
    pub battery_backed_ram: bool,
    pub trainer: bool,
}

#[derive(Clone, Copy, Debug)]
pub enum MirroringType {
    Horizontal,
    Vertical,
    FourScreen,
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
        let _prg_ram_banks = std::cmp::max(header[8], 1); //assume 1 bank exists even when 0

        if header[9..16] != [0; 7] {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Bytes 9-15 of header are not all 0",
            ));
        }

        if (rom_ctrl_byte_2 & (0b00001111)) != 0 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                "Bits 0-3 of Rom Control Byte 2 are not all 0",
            ));
        }

        let prg_bytes = u32::try_from(prg_rom_banks).unwrap() * 0x4000;
        let chr_bytes = u32::try_from(chr_rom_banks).unwrap() * 0x2000;

        let mut prg_rom = vec![0u8; usize::try_from(prg_bytes).unwrap()];
        f.read_exact(&mut prg_rom)?;

        let mut chr_rom = vec![0u8; usize::try_from(chr_bytes).unwrap()];
        f.read_exact(&mut chr_rom)?;

        // Ensure that we've read all the necessary data from the file correctly, to the end of the
        // ROM file.
        let file_metadata = f.metadata()?;
        assert!(file_metadata.len() == u64::from(prg_bytes + chr_bytes + 0x10));

        let res = Rom {
            prg_rom,
            chr_rom,
            mapper_number: (rom_ctrl_byte_2 & 0b11110000) | ((rom_ctrl_byte_1 & 0b11110000) >> 4),
            mirroring: if (rom_ctrl_byte_1 & (1 << 3)) != 0 {
                MirroringType::FourScreen
            } else if (rom_ctrl_byte_1 & (1 << 0)) == 0 {
                MirroringType::Horizontal
            } else {
                MirroringType::Vertical
            },
            battery_backed_ram: (rom_ctrl_byte_1 & (1 << 1)) != 0,
            trainer: (rom_ctrl_byte_1 & (1 << 2)) != 0,
        };
        println!("Mirroring: {:?}", res.mirroring);
        println!("Mapper Number: {}", res.mapper_number);
        Ok(res)
    }
}
