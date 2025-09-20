// https://rgbds.gbdev.io/docs/v0.9.4/gbz80.7 - reference manual

use crate::state::{GameState, Register, Flags};

// TODO Load instructions
fn ld_r8_r8(game_state: &mut GameState, r1: Register, r2: Register) {
    game_state.set_register8(r1, game_state.get_register8(r2));
}

// Arithmetic

// Add Instructions

// returns 8-bit sum value, bit 3 overflow, and bit 7 overflow

fn add8(a: u8, b: u8, carry_in: u8) -> (u8, bool, bool) {
    let sum = (a as u16) + (b as u16) + (carry_in as u16);
    let trunc_sum = sum as u8;

    let half_carry = ((a & 0xF) + (b & 0xF) + carry_in) > 0xF;
    let carry_out = sum > 0xFF;

    (trunc_sum, half_carry, carry_out)
}

fn general_add_a_n8(game_state: &mut GameState, val: u8, carry_on: bool) {
    let c: u8 = if carry_on && game_state.get_flags().C { 1 } else { 0 } ;
    let (result, half_carry, carry_out) = add8(game_state.get_register8(Register::A), val, c);

    game_state.set_register8(Register::A, result);

    let new_flags = Flags{
	Z: result == 0,
	N: false,
	H: half_carry,
	C: carry_out
    };

    game_state.set_flags(&new_flags);
}
    
fn adc_a_r8(game_state: &mut GameState, r: Register) {
    general_add_a_n8(game_state, game_state.get_register8(r), true);
}

fn adc_a_hladdr(game_state: &mut GameState) {
    general_add_a_n8(game_state, game_state.read(game_state.get_register16(Register::HL)), true);
}

fn adc_a_n8(game_state: &mut GameState, val: u8) {
    general_add_a_n8(game_state, val, true);
}

fn add_a_r8(game_state: &mut GameState, r: Register) {
    general_add_a_n8(game_state, game_state.get_register8(r), false);
}

fn add_a_hladdr(game_state: &mut GameState) {
    general_add_a_n8(game_state, game_state.read(game_state.get_register16(Register::HL)), false);
}

fn add_a_n8(game_state: &mut GameState, val: u8) {
    general_add_a_n8(game_state, val, false);
}

// returns 16-bit sum value, bit 11 overflow, and bit 15 overflow
fn add16(a: u16, b: u16) -> (u16, bool, bool) {
    let sum = (a as u32) + (b as u32);

    let half_carry = ((a & 0xFFF) + (b & 0xFFF)) > 0xFFF;
    let carry_out = sum > 0xFFFF;

    (sum as u16, half_carry, carry_out)
}

fn add_hl_r16(game_state: &mut GameState, r: Register) {
    let (result, half_carry, carry_out) = add16(game_state.get_register16(Register::HL), game_state.get_register16(r));
    game_state.set_register16(Register::HL, result);

    let new_flags = Flags{
	Z: game_state.get_flags().Z,
	N: false,
	H: half_carry,
	C: carry_out
    };

    game_state.set_flags(&new_flags);
}

// Subtract Instructions

// 8-bit difference, bit 4 borrow, final borrow
fn sub8(a: u8, b: u8, carry_in: u8) -> (u8, bool, bool) {
    let result = (a as u16) - (b as u16);

    let half_borrow = (a & 0xF) < (b & 0xF) + carry_in;
    let borrow = (a as u16) < (b as u16 + carry_in as u16);

    (result as u8, half_borrow, borrow)
}

fn general_sub_a_n8(game_state: &mut GameState, val: u8, borrow_on: bool) {
    let c: u8 = if borrow_on && game_state.get_flags().C { 1 } else { 0 } ;
    let (result, half_borrow, borrow) = sub8(game_state.get_register8(Register::A), val, c);

    game_state.set_register8(Register::A, result);

    let new_flags = Flags{
	Z: result == 0,
	N: true,
	H: half_borrow,
	C: borrow
    };

    game_state.set_flags(&new_flags);
}

fn sbc_a_r8(game_state: &mut GameState, r: Register) {
    general_sub_a_n8(game_state, game_state.get_register8(r), true);
}

fn sbc_a_hladdr(game_state: &mut GameState) {
    general_sub_a_n8(game_state, game_state.read(game_state.get_register16(Register::HL)), true);
}

fn sbc_a_n8(game_state: &mut GameState, val: u8) {
    general_sub_a_n8(game_state, val, true);
}

fn sub_a_r8(game_state: &mut GameState, r: Register) {
    general_sub_a_n8(game_state, game_state.get_register8(r), false);
}

fn sub_a_hladdr(game_state: &mut GameState) {
    general_sub_a_n8(game_state, game_state.read(game_state.get_register16(Register::HL)), false);
}

fn sub_a_n8(game_state: &mut GameState, val: u8) {
    general_sub_a_n8(game_state, val, false);
}

// TODO Compare Instructions

// TODO Decrease Instructions

// TODO Increase Instructions


// TODO Bitwise Logic

// TODO Bit Flag

// TODO Bit Shift

// TODO Control Flow

// TODO Carry Flag

// TODO Stack Manipulation

fn add_hl_sp(game_state: &mut GameState) {
    add_hl_r16(game_state, Register::SP);
}

// 16-bit signed sum, bit 11 overflow, bit 15 overflow - Not Needed
/*
fn signed_add16(a: i16, b: i16) -> (i16, bool, bool) {
    let (result, half_carry, carry_out) = add16(a as u16, b as u16);

    let signed_overflow = carry_out ^ ((a as u16 & 0x8000) + (b as u16 & 0x8000) > 0x8000);

    (result as i16, half_carry, signed_overflow)
}
*/

// returns 16-bit sum value, bit 3 overflow, and bit 7 overflow
fn add16_special(a: u16, b: u16) -> (u16, bool, bool) {
    let sum = (a as u32) + (b as u32);

    let half_carry = ((a & 0xF) + (b & 0xF)) > 0xF;
    let carry_out = sum > 0xFF;

    (sum as u16, half_carry, carry_out)
}

fn add_sp_e8(game_state: &mut GameState, e: i8) {
    let (result, half_carry, carry_out) = add16_special(game_state.get_register16(Register::SP), e as i16 as u16);
    game_state.set_register16(Register::SP, result);

    let new_flags = Flags{
	Z: game_state.get_flags().Z,
	N: false,
	H: half_carry,
	C: carry_out
    };

    game_state.set_flags(&new_flags);
}

// TODO Interrupts

// TODO Misc
