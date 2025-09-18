use std::fs;
// use std::io::Read;
use std::error::Error;

static MEM_SIZE: usize = 0x10000;

pub enum MbcType {
    RomOnly, 
    Mbc1,
    Mbc2,
    Mbc3,
    Mbc5,
    Unknown,
}

struct Cartridge {
    rom: Vec<u8>,
    ram: Vec<u8>,
    mbc: MbcType,
    current_bank: usize,
}

impl Cartridge {
    pub fn load_rom(path: &str) -> std::io::Result<Self> {
	let rom = fs::read(path)?;

	// TODO Read ROM header to figure out MBC type and RAM size
	let mbc = MbcType::RomOnly;
	let ram_size = 0x2000;

	Ok(Self {
	    rom,
	    ram: vec![0; ram_size],
	    mbc,
	    current_bank: 1,
	})
    }

    pub fn read(&self, addr: u16) -> u8 {
	match addr {
	    0x0000..=0x3FFF => {
		self.rom[addr as usize]
	    }

	    0x4000..=0x7FFF => { // TODO Implement Bank switching
		self.rom[addr as usize]
	    }
	    _ => 0xFF

	}
    }
}

fn main() -> Result<(), Box<dyn Error>> {

    let cart = Cartridge::load_rom("roms/tetris.gb")?;

    for i in 0..10 {
	println!("{}th byte: {:02X}", i + 1, cart.rom[i]);
    }

    
    println!();

    Ok(())
}
