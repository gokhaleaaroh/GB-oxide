use crate::constants::*;
use std::fs;

fn generate_16bit(lsb: u8, msb: u8) -> u16 {
    ((msb as u16) << 8) | (lsb as u16)
}

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
    pub C: bool,
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
            sp: 0xFFEE,
        }
    }
}

// TODO Finish this struct
struct IORegisters {
    joyp: u8,
    lcdc: u8,
    ly: u8,
    lyc: u8,
    stat: u8,
    scy: u8,
    scx: u8,
    wy: u8,
    wx: u8,
    bgp: u8,
    obp0: u8,
    obp1: u8,
}

impl IORegisters {
    fn reset_registers() -> Self {
        Self {
            joyp: 0,
            lcdc: 0,
            ly: 0,
            lyc: 0,
            stat: 0,
            scy: 0,
            scx: 0,
            wy: 0,
            wx: 0,
            bgp: 0,
            obp0: 0,
            obp1: 0,
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
    ime: bool,
    i_enable: u8,
    i_flag: u8,
    registers: Registers,
    io_registers: IORegisters,
    memory: Memory,
    pc_moved: bool,
    cycles: u128,
}

impl Gameboy {
    fn reset_gb() -> Self {
        Self {
            ime: false,
	    i_enable: 0,
	    i_flag: 0,
            registers: Registers::reset_registers(),
            io_registers: IORegisters::reset_registers(),
            memory: Memory::reset_memory(),
            pc_moved: false,
            cycles: 0,
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
            cart: Cartridge::load_rom(path)?,
        })
    }

    pub fn get_register8(&self, reg: Register) -> u8 {
        match reg {
            Register::A => self.gb.registers.a,
            Register::F => self.gb.registers.f,
            Register::B => self.gb.registers.b,
            Register::C => self.gb.registers.c,
            Register::D => self.gb.registers.d,
            Register::E => self.gb.registers.e,
            Register::H => self.gb.registers.h,
            Register::L => self.gb.registers.l,
            _ => 0,
        }
    }

    pub fn get_register16(&self, reg: Register) -> u16 {
        match reg {
            Register::AF => generate_16bit(
                self.get_register8(Register::F),
                self.get_register8(Register::A),
            ),
            Register::BC => generate_16bit(
                self.get_register8(Register::C),
                self.get_register8(Register::B),
            ),
            Register::DE => generate_16bit(
                self.get_register8(Register::E),
                self.get_register8(Register::D),
            ),
            Register::HL => generate_16bit(
                self.get_register8(Register::L),
                self.get_register8(Register::H),
            ),
            Register::PC => self.gb.registers.pc,
            Register::SP => self.gb.registers.sp,
            _ => 0,
        }
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
            _ => (),
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
            Register::SP => self.gb.registers.sp = val,

            _ => (),
        }
    }

    pub fn set_flags(&mut self, flags: &Flags) {
        if flags.Z {
            self.gb.registers.f |= FLAG_Z;
        } else {
            self.gb.registers.f &= !FLAG_Z;
        }

        if flags.N {
            self.gb.registers.f |= FLAG_N;
        } else {
            self.gb.registers.f &= !FLAG_N;
        }

        if flags.H {
            self.gb.registers.f |= FLAG_H;
        } else {
            self.gb.registers.f &= !FLAG_H;
        }

        if flags.C {
            self.gb.registers.f |= FLAG_C;
        } else {
            self.gb.registers.f &= !FLAG_C;
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
            0x0000..=0x3FFF => self.cart.rom[addr as usize],

            0x4000..=0x7FFF => {
                // TODO Implement Bank switching
                self.cart.rom[addr as usize]
            }

            0x8000..=0x9FFF => self.gb.memory.vram[addr as usize - 0x8000],

            0xA000..=0xBFFF => {
                // TODO External RAM
                0xFF
            }

            0xC000..=0xDFFF => self.gb.memory.wram[addr as usize - 0xC000],

            0xE000..=0xFDFF => self.gb.memory.wram[addr as usize - 0xE000],

            0xFE00..=0xFE9F => self.gb.memory.oam[addr as usize - 0xFE00],

	    0xFF0F => self.gb.i_flag,

            // TODO IO Registers and other memory mapped stuff
	    0xFF40 => self.gb.io_registers.lcdc,

	    0xFF41 => self.gb.io_registers.stat,

	    0xFF42 => self.gb.io_registers.scy,

	    0xFF43 => self.gb.io_registers.scy,

	    0xFF4A => self.gb.io_registers.wy,

	    0xFF4B => self.gb.io_registers.wx,

            0xFF80..=0xFFFE => self.gb.memory.hram[addr as usize - 0xFF80],

	    0xFFFF => self.gb.i_enable,

            _ => 0xFF,
        }
    }

    pub fn write(&mut self, value: u8, addr: u16) {
        match addr {
            0x0000..=0x7FFF => (), // Read-Only!

            0x8000..=0x9FFF => self.gb.memory.vram[addr as usize - 0x8000] = value,

            0xA000..=0xBFFF => {
                // TODO External RAM
                ()
            }

            0xC000..=0xDFFF => self.gb.memory.wram[addr as usize - 0xC000] = value,

            0xE000..=0xFDFF => self.gb.memory.wram[addr as usize - 0xE000] = value,

            0xFE00..=0xFE9F => self.gb.memory.oam[addr as usize - 0xFE00] = value,

            // TODO IO Registers and other memory mapped stuff
	    0xFF40 => self.gb.io_registers.lcdc = value,

	    0xFF41 => self.gb.io_registers.stat = value,

	    0xFF42 => self.gb.io_registers.scy = value,

	    0xFF43 => self.gb.io_registers.scy = value,

	    0xFF4A => self.gb.io_registers.wy = value,

	    0xFF4B => self.gb.io_registers.wx = value,

            0xFF80..=0xFFFE => self.gb.memory.hram[addr as usize - 0xFF80] = value,



            _ => (),
        }
    }

    pub fn set_interrupts(&mut self, on: bool) {
        self.gb.ime = on;
    }

    pub fn pc_moved(&mut self) -> bool {
        self.gb.pc_moved
    }

    pub fn set_pc_moved(&mut self, val: bool) {
        self.gb.pc_moved = val;
    }

    pub fn update_clock(&mut self, add_cycles: u8) {
        self.gb.cycles += add_cycles as u128;
    }

    pub fn get_oam_entry(&self, loc: u8) -> [u8; 4] {
        let l = (loc % 0xA0) as usize;
        return [
            self.gb.memory.oam[l],
            self.gb.memory.oam[l + 1],
            self.gb.memory.oam[l + 2],
            self.gb.memory.oam[l + 3],
        ];
    }

    pub fn get_lcdc(&self) -> u8 {
        return self.gb.io_registers.lcdc;
    }

    pub fn get_ly(&self) -> u8 {
        return self.gb.io_registers.ly;
    }

    pub fn get_scx(&self) -> u8 {
        return self.gb.io_registers.scx;
    }

    pub fn get_scy(&self) -> u8 {
        return self.gb.io_registers.scy;
    }

    pub fn get_wx(&self) -> u8 {
        return self.gb.io_registers.wx;
    }

    pub fn get_wy(&self) -> u8 {
        return self.gb.io_registers.wy;
    }

    pub fn inc_ly(&mut self, amount: u8) {
        self.gb.io_registers.ly = (self.gb.io_registers.ly + amount) % 154;
    }

    pub fn set_ly(&mut self, val: u8) {
        self.gb.io_registers.ly = val;
    }

    pub fn get_tile_index(&self, tile_in_map: u16) -> u8 {
        if self.gb.io_registers.lcdc & LCDC_TILE_MAP == 0 {
            return self.read(0x9800 + tile_in_map);
        } else {
            return self.read(0x9C00 + tile_in_map);
        }
    }

    pub fn get_tile_from_addr(&self, addr: u16) -> [u16; 8] {
        let mut result = [0u16; 8];
        for i in 0..8 {
            let byte1 = self.gb.memory.vram[(addr + 2 * i) as usize - 0x8000];
            let byte2 = self.gb.memory.vram[(addr + 2 * i + 1) as usize - 0x8000];
            result[i as usize] = ((byte2 as u16) << 8) | (byte1 as u16);
        }
        result
    }
}
