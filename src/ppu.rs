// Reference Manual - https://gbdev.io/pandocs/Graphics.html
use crate::constants::*;
use crate::state::GameState;

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
	let sprite_height = if game_state.get_lcdc() & LCDC_TILE_SIZE == 0 { 8 } else { 16 };

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

    pub fn gen_scanline(&mut self, game_state: &mut GameState) -> [u8; 160] {
	let mut result: [u8; 160] = [0; 160];
	let ly = game_state.get_ly();
	let scx = game_state.get_scx();
	let scy = game_state.get_scy();
	let td_mode = game_state.get_lcdc() & LCDC_TILE_DATA == 0;

	for x_screen in 0..160 {
	    let bg_x = (scx as u16 + x_screen) % 256;
	    let bg_y = (scy + ly) as u16 % 256;
	    let t_x = bg_x / 8;
	    let t_y = bg_y / 8;
	    let i_in_tmap = t_y * 8 + t_x;
	    let tile_index = game_state.get_tile_index(i_in_tmap);

	    let tile;
	    if td_mode { // 0x9000 addressing mode
		let tile_addr;
		if tile_index <= 127 {
		    tile_addr = 0x8000 + (tile_index as u16 * 16);
		} else {
		    tile_addr = 0x8800 + ((tile_index - 128) as u16 * 16);
		}
		tile = game_state.get_tile_from_addr(tile_addr);
	    } else { // 0x8000 addressing mode
		let tile_addr = 0x8000 + (tile_index as u16 * 16);
		tile = game_state.get_tile_from_addr(tile_addr);
	    }

	    let x_tile = bg_x % 8;
	    let y_tile = bg_y % 8;

	    let tile_row = tile[y_tile as usize];
	    let upper_byte = (tile_row & 0xFF00) >> 8;
	    let lower_byte = tile_row & 0x00FF;
	    let upper_bit = (upper_byte >> (7 - x_tile)) & 1;
	    let lower_bit = (lower_byte >> (7 - x_tile)) & 1;
	    let bg_val = ((upper_bit << 1) | lower_bit) as u8;

	    // TODO Window pixel

	    // TODO Sprite pixel
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
