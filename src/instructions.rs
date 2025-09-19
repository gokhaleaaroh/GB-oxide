use crate::state::{GameState, Register, Flags};

// Arithmetic

// returns 8-bit sum value, bit 3 carry, and bit 7 carry
fn add8(a: u8, b: u8, carry_in: u8) -> (u8, bool, bool) {
    let sum = (a as u16) + (b as u16) + (carry_in as u16);
    let trunc_sum = sum as u8;

    let half_carry = ((a & 0xF) + (b & 0xF) + carry_in) > 0xF;
    let carry_out = sum > 0xFF;

    (trunc_sum, half_carry, carry_out)
}
    
fn adc_a_r8(game_state: &mut GameState, r: Register) {
    let c: u8 = if game_state.get_flags().C { 1 } else { 0 } ;
    let (result, half_carry, carry_out) = add8(game_state.get_register8(Register::A), game_state.get_register8(r), c);

    game_state.set_register8(r, result);

    let new_flags = Flags{
	Z: result == 0,
	N: false,
	H: half_carry,
	C: carry_out
    };

    game_state.set_flags(&new_flags);
}


// Load instructions
fn ld_r8_r8(game_state: &mut GameState, r1: Register, r2: Register) {
    game_state.set_register8(r1, game_state.get_register8(r2));
}
