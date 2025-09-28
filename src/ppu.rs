use crate::state::GameState;

const DOTS_PER_SL: u16 = 456;
const VISIBLE_SL: u8 = 144;
const MAX_SL: u8 = 153;
const SIZE_BIT: u8 = 0b0000_0100;

struct OamEntry {
    y_pos: u8,
    x_pos: u8,
    tile_index: u8,
    attrs: u8,
}

pub struct PPU {
    dot_counter: u128,
    active_sprites: [OamEntry; 10],
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
    pub fn oam_scan(&mut self, game_state: &mut GameState) {
	let mut count = 0;
	let sprite_height = if self.lcdc & SIZE_BIT == 0 { 8 } else { 16 };

	for i in 0..40 {
	    if count == 10 {
		break;
	    }

	    let sprite_loc = (i*4) as u8;
	    let obj_entry = game_state.get_oam_entry(sprite_loc);
	    let y_min = obj_entry[0].wrapping_sub(16);
	    let y_max = y_min + sprite_height;

	    if y_min <= self.ly && self.ly <= y_max {
		self.active_sprites[count] = OamEntry{
		    y_pos: obj_entry[0],
		    x_pos: obj_entry[1],
		    tile_index: obj_entry[2],
		    attrs: obj_entry[3]
		};
		count += 1;
	    }
	}
    }

    pub fn step(&mut self, cycles: u8, game_state: &mut GameState) {
	self.dot_counter += cycles as u128;
	while self.dot_counter >= DOTS_PER_SL as u128 {
	    self.dot_counter -= DOTS_PER_SL as u128;
	    self.ly += 1;
	    if self.ly == VISIBLE_SL {

		// TODO Generate scanline and send to minifb buffer
		self.oam_scan(game_state);
		self.dot_counter += 80;




	    } else if self.ly > MAX_SL {
		self.ly = 0
	    }
	}
    }
}
