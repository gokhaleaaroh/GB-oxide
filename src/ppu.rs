// Reference Manual - https://gbdev.io/pandocs/Graphics.html
use crate::constants::*;
use crate::state::GameState;

#[derive(Clone, Copy)]
struct OamEntry {
    y_pos: u8,
    x_pos: u8,
    tile_index: u8,
    attrs: u8,
}

fn gb_color_to_u32(code: u8) -> u32 {
    match code {
        0b00 => 0xFFFFFFFF, // white
        0b01 => 0xFF9BB7FF, // light blue
        0b10 => 0xFF4863A0, // medium blue
        0b11 => 0xFF0A0A40, // dark navy
        // 0b00 => 0xFFFFFFFF,
        // 0b01 => 0xFFd186a8,
        // 0b10 => 0xFF64c41a,
        // 0b11 => 0xFF294f4e,
        _ => 0xFFFF00FF,
    }
}

fn get_tile_pixel(
    lcdc: u8,
    tile_index: u8,
    x_tile: u8,
    y_tile: u8,
    game_state: &mut GameState,
    sprite: bool,
) -> u8 {
    let tile;
    let tile_addr;
    if lcdc & LCDC_TILE_BG_DATA == 0 && !sprite {
        // 0x9000 addressing mode
        if tile_index <= 127 {
            tile_addr = 0x8000 + (tile_index as u16 * 16);
        } else {
            tile_addr = 0x8800 + ((tile_index - 128) as u16 * 16);
        }
    } else {
        // 0x8000 addressing mode
        tile_addr = 0x8000 + (tile_index as u16 * 16);
    }
    tile = game_state.get_tile_from_addr(tile_addr);
    let tile_row = tile[y_tile as usize];
    let upper_byte = (tile_row & 0xFF00) >> 8;
    let lower_byte = tile_row & 0x00FF;
    let upper_bit = (upper_byte >> (7 - x_tile)) & 1;
    let lower_bit = (lower_byte >> (7 - x_tile)) & 1;
    ((upper_bit << 1) | lower_bit) as u8
}

pub struct PPU {
    dot_counter: u128,
    active_sprites: [Option<OamEntry>; 10],
    pub current_fb: Vec<u32>,
}

impl PPU {
    pub fn initialize() -> Self {
        const NONE: Option<OamEntry> = None;
        Self {
            dot_counter: 0,
            active_sprites: [NONE; 10],
            current_fb: vec![0; 144 * 160],
        }
    }

    fn reset_active_entries(&mut self) {
        const NONE: Option<OamEntry> = None;
        self.active_sprites = [NONE; 10];
    }

    fn oam_scan(&mut self, game_state: &mut GameState) {
        let mut count = 0;
        let sprite_height = if game_state.get_lcdc() & LCDC_TILE_SIZE == 0 {
            7
        } else {
            15
        };

        for i in 0..40 {
            if count == 10 {
                break;
            }

            let sprite_loc = (i * 4) as u8;
            let obj_entry = game_state.get_oam_entry(sprite_loc);
            if (obj_entry[0] <= 8 && obj_entry[0] + sprite_height < 16) || (obj_entry[0] >= 160) {
                continue;
            }

            let y_min = obj_entry[0] - 16;
            let y_max = y_min + sprite_height;
            let ly = game_state.get_ly();

            if y_min <= ly && ly <= y_max {
                self.active_sprites[count] = Some(OamEntry {
                    y_pos: obj_entry[0],
                    x_pos: obj_entry[1],
                    tile_index: obj_entry[2],
                    attrs: obj_entry[3],
                });
                count += 1;
            }
        }
    }

    fn gen_scanline(&self, game_state: &mut GameState) -> [u8; 160] {
        let mut result: [u8; 160] = [0; 160];
        let ly = game_state.get_ly();
        let scx = game_state.get_scx();
        let scy = game_state.get_scy();
        let wx = game_state.get_wx();
        let wy = game_state.get_wy();
        let lcdc = game_state.get_lcdc();

        for x_screen in 0..160u8 {
            let mut final_pix;

            if lcdc & LCDC_WIN_ON != 0 && (ly >= wy && x_screen >= wx - 7) {
                // Window enabled
                let win_x = x_screen - wx + 7;
                let win_y = ly - wy;
                let t_x = win_x / 8;
                let t_y = win_y / 8;
                let x_tile = (win_x % 8) as u8;
                let y_tile = (win_y % 8) as u8;
                let i_in_tmap = (t_y * 32) as u16 + t_x as u16;
                let tile_index = game_state.get_tile_index(i_in_tmap);
                final_pix = get_tile_pixel(lcdc, tile_index, x_tile, y_tile, game_state, false);
            } else {
                // Only compute BG if Window pixel is off
                let bg_x = (scx as u16 + x_screen as u16) % 256;
                let bg_y = (scy as u16 + ly as u16) % 256;
                let t_x = bg_x / 8;
                let t_y = bg_y / 8;
                let i_in_tmap = (t_y * 32) as u16 + t_x as u16;
                let tile_index = game_state.get_tile_index(i_in_tmap);
                let x_tile = (bg_x % 8) as u8;
                let y_tile = (bg_y % 8) as u8;
                final_pix = get_tile_pixel(lcdc, tile_index, x_tile, y_tile, game_state, false);
            }

            for i in 0..10 {
                if self.active_sprites[i].is_none() {
                    continue;
                }
                let sprite_top = self.active_sprites[i].unwrap().y_pos - 16;
                // println!("New sprite_top: {sprite_top}");
                let sprite_left = self.active_sprites[i].unwrap().x_pos - 8;
                let sprite_height = if game_state.get_lcdc() & LCDC_TILE_SIZE == 0 {
                    7
                } else {
                    15
                };

                if (sprite_top <= ly && ly <= sprite_top + sprite_height)
                    && (sprite_left <= x_screen && x_screen <= sprite_left + 7)
                {
                    // sprite in line
                    let attrs = self.active_sprites[i].unwrap().attrs;
                    let y_flip = attrs & SPRITE_Y_FLIP != 0;
                    let v_offset = if y_flip {
                        sprite_height - (ly - sprite_top)
                    } else {
                        ly - sprite_top
                    };
                    let x_flip = attrs & SPRITE_X_FLIP != 0;
                    let h_offset = if x_flip {
                        7 - (x_screen - sprite_left)
                    } else {
                        x_screen - sprite_left
                    };
                    let tile_index;
                    if sprite_height == 7 {
                        tile_index = self.active_sprites[i].unwrap().tile_index;
                    } else {
                        tile_index = (self.active_sprites[i].unwrap().tile_index & 0b1111_1110)
                            + (if v_offset >= 8 { 1 } else { 0 });
                    }

                    let pix_val =
                        get_tile_pixel(lcdc, tile_index, h_offset, v_offset, game_state, true);

                    if pix_val != 0 {
                        if attrs & SPRITE_PRIORITY == 0 {
                            final_pix = pix_val;
                        } else {
                            if final_pix == 0 {
                                final_pix = pix_val;
                            }
                        }
                        break;
                    }
                }
            }

            result[x_screen as usize] = final_pix;
        }

        result
    }

    // return true if new frame is ready
    pub fn step(&mut self, cycles: u8, game_state: &mut GameState) -> bool {
        self.dot_counter += cycles as u128;
        while self.dot_counter >= DOTS_PER_SL as u128 {
            self.dot_counter -= DOTS_PER_SL as u128;
            let ly = game_state.get_ly();
            if ly < VISIBLE_SL {
                self.oam_scan(game_state);
                let next_scanline = self.gen_scanline(game_state);

                for i in 0..160 {
                    self.current_fb[(ly as u16 * 160u16 + i) as usize] =
                        gb_color_to_u32(next_scanline[i as usize]);
                }
            }
            game_state.inc_ly(1);
            self.reset_active_entries();
            if ly + 1 == VISIBLE_SL {
                // VBLANK
                game_state.write(game_state.read(0xFF0F) | INT_VBLANK, 0xFF0F);
                // println!("VBLANK");
                return true;
            }

            if ly + 1 > MAX_SL {
                game_state.set_ly(0);
            }
        }
        false
    }
}
