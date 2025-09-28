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
}

impl PPU {
    pub fn oam_scan(&mut self, game_state: &mut GameState) {
	let mut count = 0;
	let sprite_height = if game_state.get_lcdc() & SIZE_BIT == 0 { 8 } else { 16 };

	for i in 0..40 {
	    if count == 10 {
		break;
	    }

	    let sprite_loc = (i*4) as u8;
	    let obj_entry = game_state.get_oam_entry(sprite_loc);
	    let y_min = obj_entry[0].wrapping_sub(16);
	    let y_max = y_min + sprite_height;
	    let ly = game_state.get_ly();

	    if y_min <= ly &&  ly <= y_max {
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

    pub fn gen_scanline(&mut self) -> [u32; 160] {
	let mut result: [u32; 160] = [0; 160];

	for pixel in 0..160 {

	    
	}

	result
	
    }

    pub fn step(&mut self, cycles: u8, game_state: &mut GameState) {
	self.dot_counter += cycles as u128;
	while self.dot_counter >= DOTS_PER_SL as u128 {
	    self.dot_counter -= DOTS_PER_SL as u128;
	    game_state.inc_ly(1);
	    let new_ly = game_state.get_ly();
	    if  new_ly == VISIBLE_SL {

		// TODO Generate scanline and send to minifb buffer
		self.oam_scan(game_state);
		self.dot_counter += 80;


	    } else if new_ly > MAX_SL {
		game_state.set_ly(0);
	    }
	}
    }
}
