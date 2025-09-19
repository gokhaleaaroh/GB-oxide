use std::fs;

pub enum Register {
    A,
    F,
    AF,
    B,
    C,
    BC,
    D,
    E,
    DE,
    H,
    L,
    HL,
    PC,
    SP,
}

pub struct Registers {
    a: u8,
    f: u8,
    b: u8,
    c: u8,
    d: u8,
    e: u8,
    h: u8,
    l: u8,
    pc: u16,
    sp: u16,
}

impl Registers {
    pub fn reset_registers() -> Self {
	Self {
	    a: 0x01,
	    f: 0xB0,
	    b: 0x00,
	    c: 0x13,
	    d: 0x00,
	    e: 0xD8,
	    h: 0x01,
	    l: 0x4D,
	    pc: 0x0100,
	    sp: 0xFFEE
	}
    }

    /*
    pub fn get_register(&mut self, reg: &Register) -> u16 {
	match reg {
	    Register::A => self.a,

	    Register::F => self.f,

	    Register::AF => {
		self.a = ((val >> 8) & 0x0F) as u8;
		self.f = (val & 0x0F) as u8;
	    }

	    Register::B => self.b = val as u8,

	    Register::C => self.c = val as u8,

	    Register::BC => {
		self.b = ((val >> 8) & 0x0F) as u8;
		self.c = (val & 0x0F) as u8;
	    }    

	    Register::D => self.d = val as u8,

	    Register::E => self.e = val as u8,

	    Register::DE => {
		self.d = ((val >> 8) & 0x0F) as u8;
		self.e = (val & 0x0F) as u8;
	    }

	    Register::H => self.h = val as u8,

	    Register::L => self.l = val as u8,

	    Register::HL => {
		self.h = ((val >> 8) & 0x0F) as u8;
		self.l = (val & 0x0F) as u8;
	    }

	    Register::PC => self.pc = val,
	    Register::SP => self.pc = val,
	}
    }
    */

    pub fn update_register8(&mut self, reg: &Register, val: u8) {
	match reg {
	    Register::A => self.a = val,

	    Register::F => self.f = val,

	    Register::B => self.b = val,

	    Register::C => self.c = val,

	    Register::D => self.d = val,

	    Register::E => self.e = val,

	    Register::H => self.h = val,

	    Register::L => self.l = val,

	    _ => ()
	}
    }

    pub fn update_register16(&mut self, reg: &Register, val: u16) {
	match reg {
	    Register::AF => {
		self.a = ((val >> 8) & 0x0F) as u8;
		self.f = (val & 0x0F) as u8;
	    }

	    Register::BC => {
		self.b = ((val >> 8) & 0x0F) as u8;
		self.c = (val & 0x0F) as u8;
	    }    

	    Register::DE => {
		self.d = ((val >> 8) & 0x0F) as u8;
		self.e = (val & 0x0F) as u8;
	    }

	    Register::HL => {
		self.h = ((val >> 8) & 0x0F) as u8;
		self.l = (val & 0x0F) as u8;
	    }

	    Register::PC => self.pc = val,
	    Register::SP => self.pc = val,

	    _ => ()
	}
    }

}

pub struct Memory {
    wram: [u8; 0x2000],
    vram: [u8; 0x2000],
    oam: [u8; 0xA0],
    hram: [u8; 0x7F],
}

pub struct Gameboy {
    pub registers: Registers,
    pub memory: Memory,
}


pub enum MbcType {
    RomOnly, 
    Mbc1,
    Mbc2,
    Mbc3,
    Mbc5,
    Unknown,
}

pub struct Cartridge {
    pub rom: Vec<u8>,
    pub ram: Vec<u8>,
    pub mbc: MbcType,
    pub current_bank: usize,
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

}

pub struct GameState {
    pub gb: Gameboy,
    pub cart: Cartridge,
}

pub fn read(game_state: &GameState, addr: u16) -> u8 {
    match addr {
	0x0000..=0x3FFF => {
	    game_state.cart.rom[addr as usize]
	}
	
	0x4000..=0x7FFF => { // TODO Implement Bank switching
	    game_state.cart.rom[addr as usize]
	}

	0x8000..=0x9FFF => {
	    game_state.gb.memory.vram[addr as usize - 0x8000]
	}

	0xA000..=0xBFFF => { // TODO External RAM
	    0xFF
	}

	0xC000..=0xDFFF => {
	    game_state.gb.memory.wram[addr as usize - 0xC000]
	}

	0xE000..=0xFDFF => {
	    game_state.gb.memory.wram[addr as usize - 0xE000]
	}

	0xFE00..=0xFE9F => {
	    game_state.gb.memory.oam[addr as usize - 0xFE00]
	}

	0xFF80..=0xFFFE => {
	    game_state.gb.memory.hram[addr as usize - 0xFF80]
	}

	_ => 0xFF
    }
}

pub fn write(value: u8, game_state: &GameState, addr: u16) -> bool {
    false
}
