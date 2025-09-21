use crate::state::{GameState, Register};
use crate::instructions::*;

type InstructionWrapper = Box<dyn Fn(&mut GameState)>;

struct Decoder {
    non_prefix_opcodes: [InstructionWrapper; 16],
    // cb_prefix_opcodes:  [InstructionWrapper; 256]
}

impl Decoder {
    fn initialize() -> Self {
	Self {
	    non_prefix_opcodes: [
		Box::new(move |_: &mut GameState| {}), // 0x00
		Box::new(move |game_state: &mut GameState| ld_r16_n16(game_state, Register::BC)), // 0x01
		Box::new(move |game_state: &mut GameState| ld_r16addr_a(game_state, Register::BC)), // 0x02
		Box::new(move |game_state: &mut GameState| inc_r16(game_state, Register::BC)), // 0x03    
		Box::new(move |game_state: &mut GameState| inc_r8(game_state, Register::B)), // 0x04
		Box::new(move |game_state: &mut GameState| dec_r8(game_state, Register::B)), // 0x05
		Box::new(move |game_state: &mut GameState| ld_r8_n8(game_state, Register::B)), // 0x06
		Box::new(move |game_state: &mut GameState| rlc_a(game_state)), // 0x07
		Box::new(move |game_state: &mut GameState| ld_n16addr_sp(game_state)), // 0x08
		Box::new(move |game_state: &mut GameState| add_hl_r16(game_state, Register::BC)), // 0x09
		Box::new(move |game_state: &mut GameState| ld_a_r16addr(game_state, Register::BC)), // 0x0A
		Box::new(move |game_state: &mut GameState| dec_r16(game_state, Register::BC)), // 0x0B
		Box::new(move |game_state: &mut GameState| inc_r8(game_state, Register::C)), // 0x0C
		Box::new(move |game_state: &mut GameState| dec_r8(game_state, Register::C)), // 0x0D
		Box::new(move |game_state: &mut GameState| ld_r8_n8(game_state, Register::C)), // 0x0E
		Box::new(move |game_state: &mut GameState| rrc_a(game_state)), // 0x0F
	    ]
	}
    }

}
