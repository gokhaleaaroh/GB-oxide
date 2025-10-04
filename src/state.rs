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

pub enum CC {
    Z,
    NZ,
    C,
    NC,
}

struct Joypad {
    a_button: bool,
    b_button: bool,
    start_button: bool,
    select_button: bool,
    up_button: bool,
    down_button: bool,
    left_button: bool,
    right_button: bool,
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
            sp: 0xFFFE,
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
            joyp: 0xCF,
            lcdc: 0x91,
            ly: 0,
            lyc: 0,
            stat: 0x85,
            scy: 0,
            scx: 0,
            wy: 0,
            wx: 0,
            bgp: 0xFC,
            obp0: 0xFF,
            obp1: 0xFF,
        }
    }
}

struct TimerRegisters {
    div: u16,
    tima: u8,
    tma: u8,
    tac: u8,
}

impl TimerRegisters {
    fn reset_registers() -> Self {
        Self {
            div: 0x0000,
            tima: 0x00,
            tma: 0x00,
            tac: 0xF8,
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
    dma: u8,
    joypad: Joypad,
    registers: Registers,
    io_registers: IORegisters,
    timer_registers: TimerRegisters,
    available_cycles: u16,
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
            dma: 0,
            joypad: Joypad {
                a_button: false,
                b_button: false,
                start_button: false,
                select_button: false,
                up_button: false,
                down_button: false,
                left_button: false,
                right_button: false,
            },
            registers: Registers::reset_registers(),
            io_registers: IORegisters::reset_registers(),
            timer_registers: TimerRegisters::reset_registers(),
            available_cycles: 0,
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
                self.gb.registers.a = (val >> 8) as u8;
                self.gb.registers.f = (val & 0x00FF) as u8;
            }

            Register::BC => {
                self.gb.registers.b = (val >> 8) as u8;
                self.gb.registers.c = (val & 0x00FF) as u8;
            }

            Register::DE => {
                self.gb.registers.d = (val >> 8) as u8;
                self.gb.registers.e = (val & 0x00FF) as u8;
            }

            Register::HL => {
                self.gb.registers.h = (val >> 8) as u8;
                self.gb.registers.l = (val & 0x00FF) as u8;
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

    fn dma_oam(&mut self, value: u8) {
        self.gb.dma = value;
        let start = (value as u16) << 8;
        for i in 0..160 {
            self.gb.memory.oam[i] = self.read(start + i as u16);
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

            0xFF00 => {
                // print!("Read joypad, Returning val: ");
                let mut button_bits = 0b0000_1111;
                if self.gb.joypad.a_button {
                    // println!("A WAS PRESSED");
                    button_bits &= 0b1111_1110;
                }

                if self.gb.joypad.b_button {
                    // println!("B WAS PRESSED");
                    button_bits &= 0b1111_1101;
                }

                if self.gb.joypad.select_button {
                    // println!("SELECT WAS PRESSED");
                    button_bits &= 0b1111_1011;
                }

                if self.gb.joypad.start_button {
                    //  println!("START WAS PRESSED");
                    button_bits &= 0b1111_0111;
                }

                let mut d_pad_bits = 0x0F;
                if self.gb.joypad.right_button {
                    d_pad_bits &= 0b1111_1110;
                }

                if self.gb.joypad.left_button {
                    d_pad_bits &= 0b1111_1101;
                }

                if self.gb.joypad.up_button {
                    d_pad_bits &= 0b1111_1011;
                }

                if self.gb.joypad.down_button {
                    d_pad_bits &= 0b1111_0111;
                }

                let select_buttons = self.gb.io_registers.joyp & 0b0010_0000;
                let select_d_pad = self.gb.io_registers.joyp & 0b0001_0000;

                if select_buttons == 0 && select_d_pad == 0 {
                    (self.gb.io_registers.joyp | 0x0F) & (button_bits & d_pad_bits)
                } else if select_buttons == 0 {
                    (self.gb.io_registers.joyp | 0x0F) & button_bits
                } else if select_d_pad == 0 {
                    (self.gb.io_registers.joyp | 0x0F) & d_pad_bits
                } else {
                    self.gb.io_registers.joyp | 0x0F
                }
            }

            0xFF04 => (self.gb.timer_registers.div >> 8) as u8,

            0xFF05 => self.gb.timer_registers.tima,

            0xFF06 => self.gb.timer_registers.tma,

            0xFF07 => self.gb.timer_registers.tac,

            0xFF0F => self.gb.i_flag,

            // TODO IO Registers and other memory mapped stuff
            0xFF40 => self.gb.io_registers.lcdc,

            0xFF41 => self.gb.io_registers.stat,

            0xFF42 => self.gb.io_registers.scy,

            0xFF43 => self.gb.io_registers.scx,

            0xFF44 => self.gb.io_registers.ly,

            0xFF45 => self.gb.io_registers.lyc,

            0xFF46 => self.gb.dma,

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

            0x8000..=0x9FFF => {
                // println!(
                //     "WRITING TO VRAM addr: 0x{:04X} value: 0x{:02X}",
                //     addr, value
                // );
                self.gb.memory.vram[addr as usize - 0x8000] = value
            }

            0xA000..=0xBFFF => {
                // TODO External RAM
                ()
            }

            0xC000..=0xDFFF => self.gb.memory.wram[addr as usize - 0xC000] = value,

            0xE000..=0xFDFF => self.gb.memory.wram[addr as usize - 0xE000] = value,

            0xFE00..=0xFE9F => self.gb.memory.oam[addr as usize - 0xFE00] = value,

            0xFF00 => {
                self.gb.io_registers.joyp =
                    (self.gb.io_registers.joyp & 0b1100_1111) | (value & 0b0011_0000);
            }

            0xFF04 => self.gb.timer_registers.div = 0,

            0xFF05 => self.gb.timer_registers.tima = value,

            0xFF06 => self.gb.timer_registers.tma = value,

            0xFF07 => {
                println!("Writing: 0b{:08b} to TAC", value);
                self.gb.timer_registers.tac = value;
            }

            0xFF0F => self.gb.i_flag = value,

            // TODO IO Registers and other memory mapped stuff
            0xFF40 => self.gb.io_registers.lcdc = value,

            0xFF41 => self.gb.io_registers.stat = value,

            0xFF42 => self.gb.io_registers.scy = value,

            0xFF43 => self.gb.io_registers.scx = value,

            0xFF45 => self.gb.io_registers.lyc = value,

            0xFF46 => self.dma_oam(value),

            0xFF4A => self.gb.io_registers.wy = value,

            0xFF4B => self.gb.io_registers.wx = value,

            0xFF80..=0xFFFE => {
                // println!("WRITING TO HRAM addr: 0x{:04X} value: 0x{:02X}", addr, value);
                self.gb.memory.hram[addr as usize - 0xFF80] = value
            }

            0xFFFF => self.gb.i_enable = value,
            _ => (),
        }
    }

    pub fn set_interrupts(&mut self, on: bool) {
        self.gb.ime = on;
    }

    pub fn get_interrupts(&self) -> bool {
        self.gb.ime
    }

    pub fn get_i_flag(&self) -> u8 {
        self.gb.i_flag
    }

    pub fn get_i_enable(&self) -> u8 {
        self.gb.i_enable
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

    pub fn inc_div(&mut self, amount: u8) {
        self.gb.timer_registers.div = self.gb.timer_registers.div.wrapping_add(amount as u16);
    }

    pub fn get_tac(&self) -> u8 {
        self.gb.timer_registers.tac
    }

    pub fn inc_available_cycles(&mut self, val: u16) {
        assert!(self.gb.available_cycles <= 0xFFFF - val);
        self.gb.available_cycles += val;
    }

    pub fn dec_available_cycles(&mut self, val: u16) {
        assert!(self.gb.available_cycles >= val);
        self.gb.available_cycles -= val;
    }

    pub fn get_available_cycles(&self) -> u16 {
        self.gb.available_cycles
    }

    pub fn update_tima(&mut self) {
        if self.gb.timer_registers.tima > 0xFF - 1 {
            self.gb.timer_registers.tima = self.gb.timer_registers.tma;
            // Timer Interrupt
            self.write(self.read(0xFF0F) | INT_TIMER, 0xFF0F);
        } else {
            self.gb.timer_registers.tima += 1;
        }
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

    pub fn update_joypad(
        &mut self,
        a: bool,
        b: bool,
        start: bool,
        select: bool,
        up: bool,
        down: bool,
        left: bool,
        right: bool,
    ) {
        self.gb.joypad.a_button = a;
        self.gb.joypad.b_button = b;
        self.gb.joypad.start_button = start;
        self.gb.joypad.select_button = select;
        self.gb.joypad.up_button = up;
        self.gb.joypad.down_button = down;
        self.gb.joypad.left_button = left;
        self.gb.joypad.right_button = right;
    }

    // pub fn print_oam(&self) {
    //     for i in 0..160 {
    //         print!("0x{:02X}", self.gb.memory.oam[i]);
    //     }
    //     println!();
    // }
}
