use std::fs;

#[derive(Copy, Clone, Debug)]
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

pub struct Flags {
    pub Z: bool,
    pub N: bool,
    pub H: bool,
    pub C: bool
}

const FLAG_Z: u8 = 0b1000_0000;
const FLAG_N: u8 = 0b0100_0000;
const FLAG_H: u8 = 0b0010_0000;
const FLAG_C: u8 = 0b0001_0000;

struct Registers {
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
    fn reset_registers() -> Self {
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
}

struct Memory {
    wram: [u8; 0x2000],
    vram: [u8; 0x2000],
    oam: [u8; 0xA0],
    hram: [u8; 0x7F],
}

impl Memory {
    fn reset_memory() -> Self {
	Self {
	    wram: [0; 0x2000],
	    vram: [0; 0x2000],
	    oam: [0; 0xA0],
	    hram: [0; 0x7F],
	}
    }
}

struct Gameboy {
    IME: bool,
    registers: Registers,
    memory: Memory,
    pc_moved: bool
}

impl Gameboy {
    fn reset_gb() -> Self {
	Self {
	    IME: false,
	    registers: Registers::reset_registers(),
	    memory: Memory::reset_memory(),
	    pc_moved: false
	}
    }
}

enum MbcType {
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
}

pub struct GameState {
    gb: Gameboy,
    cart: Cartridge,
}

impl GameState {
    pub fn start_game(path: &str) -> std::io::Result<Self> {
	Ok(Self {
	    gb: Gameboy::reset_gb(),
	    cart: Cartridge::load_rom(path)?
	})
    }

    pub fn get_register8(&self, reg: Register) -> u8 {
	0
    }

    pub fn get_register16(&self, reg: Register) -> u16 {
	0
    }

    pub fn set_register8(&mut self, reg: Register, val: u8) {
	match reg {
	    Register::A => self.gb.registers.a = val,

	    Register::F => self.gb.registers.f = val,

	    Register::B => self.gb.registers.b = val,

	    Register::C => self.gb.registers.c = val,

	    Register::D => self.gb.registers.d = val,

	    Register::E => self.gb.registers.e = val,

	    Register::H => self.gb.registers.h = val,

	    Register::L => self.gb.registers.l = val,

	    _ => ()
	}
    }

    pub fn set_register16(&mut self, reg: Register, val: u16) {
	match reg {
	    Register::AF => {
		self.gb.registers.a = ((val >> 8) & 0x0F) as u8;
		self.gb.registers.f = (val & 0x0F) as u8;
	    }

	    Register::BC => {
		self.gb.registers.b = ((val >> 8) & 0x0F) as u8;
		self.gb.registers.c = (val & 0x0F) as u8;
	    }    

	    Register::DE => {
		self.gb.registers.d = ((val >> 8) & 0x0F) as u8;
		self.gb.registers.e = (val & 0x0F) as u8;
	    }

	    Register::HL => {
		self.gb.registers.h = ((val >> 8) & 0x0F) as u8;
		self.gb.registers.l = (val & 0x0F) as u8;
	    }

	    Register::PC => self.gb.registers.pc = val,
	    Register::SP => self.gb.registers.pc = val,

	    _ => ()
	}
    }

    pub fn set_flags(&mut self, flags: &Flags) {
	if flags.Z {
	    self.gb.registers.f |= FLAG_Z;
	}

	if flags.N {
	    self.gb.registers.f |= FLAG_N;
	}

	if flags.H {
	    self.gb.registers.f |= FLAG_H;
	}

	if flags.C {
	    self.gb.registers.f |= FLAG_C;
	}
    }

    pub fn get_flags(&self) -> Flags {
	Flags {
	    Z: self.gb.registers.f & FLAG_Z != 0,
	    N: self.gb.registers.f & FLAG_N != 0,
	    H: self.gb.registers.f & FLAG_H != 0,
	    C: self.gb.registers.f & FLAG_C != 0,
	}
    }

    pub fn read(&self, addr: u16) -> u8 {
	match addr {
	    0x0000..=0x3FFF => {
		self.cart.rom[addr as usize]
	    }
	    
	    0x4000..=0x7FFF => { // TODO Implement Bank switching
		self.cart.rom[addr as usize]
	    }

	    0x8000..=0x9FFF => {
		self.gb.memory.vram[addr as usize - 0x8000]
	    }

	    0xA000..=0xBFFF => { // TODO External RAM
		0xFF
	    }

	    0xC000..=0xDFFF => {
		self.gb.memory.wram[addr as usize - 0xC000]
	    }

	    0xE000..=0xFDFF => {
		self.gb.memory.wram[addr as usize - 0xE000]
	    }

	    0xFE00..=0xFE9F => {
		self.gb.memory.oam[addr as usize - 0xFE00]
	    }

	    0xFF80..=0xFFFE => {
		self.gb.memory.hram[addr as usize - 0xFF80]
	    }

	    _ => 0xFF
	}
    }

    pub fn write(&mut self, value: u8,  addr: u16) -> bool {
	false
    }

    pub fn set_interrupts(&mut self, on: bool) {
	self.gb.IME = on;
    } 

    pub fn pc_moved(&mut self) -> bool {
	self.gb.pc_moved
    }

    pub fn set_pc_moved(&mut self, val: bool) {
	self.gb.pc_moved = false;
    }

}

