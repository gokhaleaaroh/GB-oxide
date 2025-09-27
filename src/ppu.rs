const DOTS_PER_SL: u16 = 456;
const VISIBLE_SL: u8 = 144;
const MAX_SL: u8 = 153;

pub struct PPU {
    dot_counter: u128,
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

impl PPU {
    pub fn step(&mut self, cycles: u8) {
	self.dot_counter += cycles as u128;
	while self.dot_counter >= DOTS_PER_SL as u128 {
	    self.dot_counter -= DOTS_PER_SL as u128;
	    self.ly += 1;
	    if self.ly == VISIBLE_SL {
	    } else if self.ly > MAX_SL {
		self.ly = 0
	    }
	}
    }
}
