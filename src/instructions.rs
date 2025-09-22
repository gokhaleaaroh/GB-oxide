// https://rgbds.gbdev.io/docs/v0.9.4/gbz80.7 - reference manual

use crate::state::{Flags, GameState, Register};

// TODO Implement PC updates

// Load instructions
pub fn ld_r8_r8(game_state: &mut GameState, r1: Register, r2: Register) {
    game_state.set_register8(r1, game_state.get_register8(r2));
}

pub fn ld_r8_n8(game_state: &mut GameState, r1: Register) {
    let val = game_state.read(game_state.get_register16(Register::PC) + 1);
    game_state.set_register8(r1, val);
}

pub fn ld_r16_n16(game_state: &mut GameState, r1: Register) {
    let lsb = game_state.read(game_state.get_register16(Register::PC) + 1);
    let msb = game_state.read(game_state.get_register16(Register::PC) + 2);
    let val = ((msb as u16) << 8) | (lsb as u16);
    game_state.set_register16(r1, val);
}

pub fn ld_hladdr_r8(game_state: &mut GameState, r: Register) {
    game_state.write(game_state.get_register8(r), game_state.get_register16(Register::HL));
}

pub fn ld_hladdr_n8(game_state: &mut GameState) {
    let val = game_state.read(game_state.get_register16(Register::PC) + 1);
    game_state.write(val, game_state.get_register16(Register::HL));
}

pub fn ld_r8_hladdr(game_state: &mut GameState, r: Register) {
    game_state.set_register8(r, game_state.read(game_state.get_register16(Register::HL)));
}

pub fn ld_r16addr_a(game_state: &mut GameState, r: Register) {
    game_state.write(game_state.get_register8(Register::A), game_state.get_register16(r));
}

pub fn ld_n16addr_a(game_state: &mut GameState) {
    let lsb = game_state.read(game_state.get_register16(Register::PC) + 1);
    let msb = game_state.read(game_state.get_register16(Register::PC) + 2);
    let addr = ((msb as u16) << 8) | (lsb as u16);
    game_state.write(game_state.get_register8(Register::A), addr);
}

pub fn ldh_n16addr_a(game_state: &mut GameState) {
    let lsb = game_state.read(game_state.get_register16(Register::PC) + 1);
    let msb = game_state.read(game_state.get_register16(Register::PC) + 2);
    let addr = ((msb as u16) << 8) | (lsb as u16);
    if 0xFF00 <= addr {
	game_state.write(game_state.get_register8(Register::A), addr);
    }
}

pub fn ldh_caddr_a(game_state: &mut GameState) {
    game_state.write(game_state.get_register8(Register::A), 0xFF00 + game_state.get_register8(Register::C) as u16);
}

pub fn ld_a_r16addr(game_state: &mut GameState, r: Register) {
    game_state.set_register8(Register::A, game_state.read(game_state.get_register16(r)));
}

pub fn ld_a_n16addr(game_state: &mut GameState) {
    let lsb = game_state.read(game_state.get_register16(Register::PC) + 1);
    let msb = game_state.read(game_state.get_register16(Register::PC) + 2);
    let addr = ((msb as u16) << 8) | (lsb as u16);
    game_state.set_register8(Register::A, game_state.read(addr));
}

pub fn ldh_a_n16addr(game_state: &mut GameState) {
    let lsb = game_state.read(game_state.get_register16(Register::PC) + 1);
    let msb = game_state.read(game_state.get_register16(Register::PC) + 2);
    let addr = ((msb as u16) << 8) | (lsb as u16);
    if 0xFF00 <= addr {
	game_state.set_register8(Register::A, game_state.read(addr));
    }
}

pub fn ldh_a_caddr(game_state: &mut GameState) {
    game_state.set_register8(Register::A, game_state.read(game_state.get_register8(Register::C) as u16 + 0xFF00));
}

pub fn ld_hliaddr_a(game_state: &mut GameState) {
    ld_hladdr_r8(game_state, Register::A);
    game_state.set_register16(Register::HL, game_state.get_register16(Register::HL) + 1);
}

pub fn ld_hldaddr_a(game_state: &mut GameState) {
    ld_hladdr_r8(game_state, Register::A);
    game_state.set_register16(Register::HL, game_state.get_register16(Register::HL) - 1);
}

pub fn ld_a_hld(game_state: &mut GameState) {
    ld_a_r16addr(game_state, Register::HL);
    game_state.set_register16(Register::HL, game_state.get_register16(Register::HL) - 1);
}

pub fn ld_a_hli(game_state: &mut GameState) {
    ld_a_r16addr(game_state, Register::HL);
    game_state.set_register16(Register::HL, game_state.get_register16(Register::HL) + 1);
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
    
pub fn adc_a_r8(game_state: &mut GameState, r: Register) {
    general_add_a_n8(game_state, game_state.get_register8(r), true);
}

pub fn adc_a_hladdr(game_state: &mut GameState) {
    general_add_a_n8(game_state, game_state.read(game_state.get_register16(Register::HL)), true);
}

pub fn adc_a_n8(game_state: &mut GameState) {
    let val = game_state.read(game_state.get_register16(Register::PC) + 1);
    general_add_a_n8(game_state, val, true);
}

pub fn add_a_r8(game_state: &mut GameState, r: Register) {
    general_add_a_n8(game_state, game_state.get_register8(r), false);
}

pub fn add_a_hladdr(game_state: &mut GameState) {
    general_add_a_n8(game_state, game_state.read(game_state.get_register16(Register::HL)), false);
}

pub fn add_a_n8(game_state: &mut GameState) {
    let val = game_state.read(game_state.get_register16(Register::PC) + 1);
    general_add_a_n8(game_state, val, false);
}

// returns 16-bit sum value, bit 11 overflow, and bit 15 overflow
fn add16(a: u16, b: u16) -> (u16, bool, bool) {
    let sum = (a as u32) + (b as u32);

    let half_carry = ((a & 0xFFF) + (b & 0xFFF)) > 0xFFF;
    let carry_out = sum > 0xFFFF;

    (sum as u16, half_carry, carry_out)
}

pub fn add_hl_r16(game_state: &mut GameState, r: Register) {
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

fn general_sub_a_n8(game_state: &mut GameState, val: u8, borrow_on: bool, discard: bool) {
    let c: u8 = if borrow_on && game_state.get_flags().C { 1 } else { 0 } ;
    let (result, half_borrow, borrow) = sub8(game_state.get_register8(Register::A), val, c);

    if !discard { game_state.set_register8(Register::A, result); }

    let new_flags = Flags{
	Z: result == 0,
	N: true,
	H: half_borrow,
	C: borrow
    };

    game_state.set_flags(&new_flags);
}

pub fn sbc_a_r8(game_state: &mut GameState, r: Register) {
    general_sub_a_n8(game_state, game_state.get_register8(r), true, false);
}

pub fn sbc_a_hladdr(game_state: &mut GameState) {
    general_sub_a_n8(game_state, game_state.read(game_state.get_register16(Register::HL)), true, false);
}

pub fn sbc_a_n8(game_state: &mut GameState) {
    let val = game_state.read(game_state.get_register16(Register::PC) + 1);
    general_sub_a_n8(game_state, val, true, false);
}

pub fn sub_a_r8(game_state: &mut GameState, r: Register) {
    general_sub_a_n8(game_state, game_state.get_register8(r), false, false);
}

pub fn sub_a_hladdr(game_state: &mut GameState) {
    general_sub_a_n8(game_state, game_state.read(game_state.get_register16(Register::HL)), false, false);
}

pub fn sub_a_n8(game_state: &mut GameState) {
    let val = game_state.read(game_state.get_register16(Register::PC) + 1);
    general_sub_a_n8(game_state, val, false, false);
}

// Compare Instructions
pub fn cp_a_r8(game_state: &mut GameState, r: Register) {
    general_sub_a_n8(game_state, game_state.get_register8(r), false, true);
}

pub fn cp_a_hladdr(game_state: &mut GameState) {
    general_sub_a_n8(game_state, game_state.read(game_state.get_register16(Register::HL)), false, true);
}

pub fn cp_a_n8(game_state: &mut GameState) {
    let val = game_state.read(game_state.get_register16(Register::PC) + 1);
    general_sub_a_n8(game_state, val, false, true);
}

// Decrease Instructions
pub fn dec_r8(game_state: &mut GameState, r: Register) {
    let (result, half_borrow, _) = sub8(game_state.get_register8(r), 1, 0);
    game_state.set_register8(r, result);

    let new_flags = Flags{
	Z: result == 0,
	N: true,
	H: half_borrow,
	C: game_state.get_flags().C
    };

    game_state.set_flags(&new_flags);
}

pub fn dec_hladdr(game_state: &mut GameState) {
    let addr = game_state.get_register16(Register::HL);
    let (result, half_borrow, _) = sub8(game_state.read(addr), 1, 0);
    game_state.write(result, addr);

    let new_flags = Flags{
	Z: result == 0,
	N: true,
	H: half_borrow,
	C: game_state.get_flags().C
    };

    game_state.set_flags(&new_flags);
}

pub fn dec_r16(game_state: &mut GameState, r: Register) {
    game_state.set_register16(r, game_state.get_register16(r) - (1 as u16));
}
    
// Increase Instructions
pub fn inc_r8(game_state: &mut GameState, r: Register) {
    let (result, half_carry, _) = add8(game_state.get_register8(r), 1, 0);

    game_state.set_register8(r, result);

    let new_flags = Flags{
	Z: result == 0,
	N: false,
	H: half_carry,
	C: game_state.get_flags().C
    };

    game_state.set_flags(&new_flags);
}

pub fn inc_hladdr(game_state: &mut GameState) {
    let addr = game_state.get_register16(Register::HL);
    let (result, half_carry, _) = add8(game_state.read(addr), 1, 0);
    game_state.write(result, addr);

    let new_flags = Flags{
	Z: result == 0,
	N: false,
	H: half_carry,
	C: game_state.get_flags().C
    };

    game_state.set_flags(&new_flags);
}

pub fn inc_r16(game_state: &mut GameState, r: Register) {
    game_state.set_register16(r, game_state.get_register16(r) + (1 as u16));
}

// Bitwise Logic
fn general_and_a(game_state: &mut GameState, val: u8) -> u8 {
    let result = game_state.get_register8(Register::A) & val;
    let new_flags = Flags {
	Z: result == 0,
	N: false,
	H: true,
	C: false
    };

    game_state.set_flags(&new_flags);
    result
}

pub fn and_a_r8(game_state: &mut GameState, r: Register) {
    let result = general_and_a(game_state, game_state.get_register8(r));
    game_state.set_register8(Register::A, result);
}

pub fn and_a_hladdr(game_state: &mut GameState) {
    let addr = game_state.get_register16(Register::HL);
    let result = general_and_a(game_state, game_state.read(addr));
    game_state.write(result, addr);
}

pub fn and_a_n8(game_state: &mut GameState) {
    let val = game_state.read(game_state.get_register16(Register::PC) + 1);
    let result = general_and_a(game_state, val);
    game_state.set_register8(Register::A, result);
}

pub fn cpl(game_state: &mut GameState) {
    game_state.set_register8(Register::A, !game_state.get_register8(Register::A));

    let old_flags = game_state.get_flags();

    let new_flags = Flags {
	Z: old_flags.Z,
	N: true,
	H: true,
	C: old_flags.C
    };

    game_state.set_flags(&new_flags);
}

fn general_or_a(game_state: &mut GameState, val: u8, exclusive: bool) -> u8 {
    let a = game_state.get_register8(Register::A);
    let result =  if exclusive { a ^ val } else { a | val };
    let new_flags = Flags {
	Z: result == 0,
	N: false,
	H: false,
	C: false
    };

    game_state.set_flags(&new_flags);
    result
}

pub fn or_a_r8(game_state: &mut GameState, r: Register) {
    let result = general_or_a(game_state, game_state.get_register8(r), false);
    game_state.set_register8(Register::A, result);
}

pub fn or_a_hladdr(game_state: &mut GameState) {
    let addr = game_state.get_register16(Register::HL);
    let result = general_or_a(game_state, game_state.read(addr), false);
    game_state.write(result, addr);
}

pub fn or_a_n8(game_state: &mut GameState) {
    let val = game_state.read(game_state.get_register16(Register::PC) + 1);
    let result = general_or_a(game_state, val, false);
    game_state.set_register8(Register::A, result);
}

pub fn xor_a_r8(game_state: &mut GameState, r: Register) {
    let result = general_or_a(game_state, game_state.get_register8(r), true);
    game_state.set_register8(Register::A, result);
}

pub fn xor_a_hladdr(game_state: &mut GameState) {
    let addr = game_state.get_register16(Register::HL);
    let result = general_or_a(game_state, game_state.read(addr), true);
    game_state.write(result, addr);
}

pub fn xor_a_n8(game_state: &mut GameState) {
    let val = game_state.read(game_state.get_register16(Register::PC) + 1);
    let result = general_or_a(game_state, val, true);
    game_state.set_register8(Register::A, result);
}

// Bit Flag
pub fn bit_u3_r8(game_state: &mut GameState, u: u8,  r: Register) {
    if u > 7  { return }
    let zero_flag = (game_state.get_register8(r) & (1 << u)) == 0;
    let new_flags = Flags {
	Z: zero_flag,
	N: false,
	H: true,
	C: game_state.get_flags().C
    };

    game_state.set_flags(&new_flags);
}

pub fn bit_u3_hladdr(game_state: &mut GameState, u: u8) {
    if u > 7  { return }
    let zero_flag = (game_state.read(game_state.get_register16(Register::HL)) & (1 << u)) == 0;
    let new_flags = Flags {
	Z: zero_flag,
	N: false,
	H: true,
	C: game_state.get_flags().C
    };

    game_state.set_flags(&new_flags);
}

pub fn res_u3_r8(game_state: &mut GameState, u: u8, r: Register) {
    if u > 7 { return }
    game_state.set_register8(r, game_state.get_register8(r) & !(1 << u));
}

pub fn res_u3_hladdr(game_state: &mut GameState, u: u8) {
    if u > 7 { return }
    let addr = game_state.get_register16(Register::HL);
    game_state.write(game_state.read(addr) & !(1 << u), addr);
}

pub fn set_u3_r8(game_state: &mut GameState, u: u8, r: Register) {
    if u > 7 { return }
    game_state.set_register8(r, game_state.get_register8(r) | (1 << u));
}

pub fn set_u3_hladdr(game_state: &mut GameState, u: u8) {
    if u > 7 { return }
    let addr = game_state.get_register16(Register::HL);
    game_state.write(game_state.read(addr) | (1 << u), addr);
}

// Bit Shift
fn general_rl(game_state: &mut GameState, val: u8, check_zero: bool) -> u8 {
    let old_c = if game_state.get_flags().C { 1 } else { 0 };
    let new_c = val >> 7;
    let result = (val << 1) | old_c;
    let new_flags = Flags {
	Z: result == 0 && check_zero,
	N: false,
	H: false,
	C: new_c == 1
    };
    game_state.set_flags(&new_flags);
    result
}

pub fn rl_r8(game_state: &mut GameState, r: Register) {
    let result = general_rl(game_state, game_state.get_register8(r), true);
    game_state.set_register8(r, result);
}

pub fn rl_hladdr(game_state: &mut GameState) {
    let addr = game_state.get_register16(Register::HL);
    let result = general_rl(game_state, game_state.read(addr), true);
    game_state.write(result, addr);
}

pub fn rl_a(game_state: &mut GameState) {
    let result = general_rl(game_state, game_state.get_register8(Register::A), false);
    game_state.set_register8(Register::A, result);
}

fn general_rlc(game_state: &mut GameState, val: u8, check_zero: bool) -> u8 {
    let old_msb = val >> 7;
    let result = (val << 1) | old_msb;
    let new_flags = Flags {
	Z: result == 0 && check_zero,
	N: false,
	H: false,
	C: old_msb == 1
    };
    game_state.set_flags(&new_flags);
    result
}

pub fn rlc_r8(game_state: &mut GameState, r: Register) {
    let result = general_rlc(game_state, game_state.get_register8(r), true);
    game_state.set_register8(r, result);
}

pub fn rlc_hladdr(game_state: &mut GameState) {
    let addr = game_state.get_register16(Register::HL);
    let result = general_rlc(game_state, game_state.read(addr), true);
    game_state.write(result, addr);
}

pub fn rlca(game_state: &mut GameState) {
    let result = general_rlc(game_state, game_state.get_register8(Register::A), false);
    game_state.set_register8(Register::A, result);
}

fn general_rr(game_state: &mut GameState, val: u8, check_zero: bool) -> u8 {
    let old_c = if game_state.get_flags().C { 1 } else { 0 };
    let new_c = val & 1;
    let result = (val >> 1) | (old_c << 7);
    let new_flags = Flags {
	Z: result == 0 && check_zero,
	N: false,
	H: false,
	C: new_c == 1
    };
    game_state.set_flags(&new_flags);
    result
}

pub fn rr_r8(game_state: &mut GameState, r: Register) {
    let result = general_rr(game_state, game_state.get_register8(r), true);
    game_state.set_register8(r, result);
}

pub fn rr_hladdr(game_state: &mut GameState) {
    let addr = game_state.get_register16(Register::HL);
    let result = general_rr(game_state, game_state.read(addr), true);
    game_state.write(result, addr);
}

pub fn rr_a(game_state: &mut GameState) {
    let result = general_rr(game_state, game_state.get_register8(Register::A), false);
    game_state.set_register8(Register::A, result);
}

fn general_rrc(game_state: &mut GameState, val: u8, check_zero: bool) -> u8 {
    let old_lsb = val & 1;
    let result = (val >> 1) | (old_lsb << 7);
    let new_flags = Flags {
	Z: result == 0 && check_zero,
	N: false,
	H: false,
	C: old_lsb == 1
    };
    game_state.set_flags(&new_flags);
    result
}

pub fn rrc_r8(game_state: &mut GameState, r: Register) {
    let result = general_rrc(game_state, game_state.get_register8(r), true);
    game_state.set_register8(r, result);
}

pub fn rrc_hladdr(game_state: &mut GameState) {
    let addr = game_state.get_register16(Register::HL);
    let result = general_rrc(game_state, game_state.read(addr), true);
    game_state.write(result, addr);
}

pub fn rrc_a(game_state: &mut GameState) {
    let result = general_rrc(game_state, game_state.get_register8(Register::A), false);
    game_state.set_register8(Register::A, result);
}

fn general_sla(game_state: &mut GameState, val: u8) -> u8 {
    let old_msb = val >> 7;
    let result = val << 1;
    let new_flags = Flags {
	Z: result == 0,
	N: false,
	H: false,
	C: old_msb == 1
    };
    game_state.set_flags(&new_flags);
    result
}

pub fn sla_r8(game_state: &mut GameState, r: Register) {
    let result = general_sla(game_state, game_state.get_register8(r));
    game_state.set_register8(r, result);
}

pub fn sla_hladdr(game_state: &mut GameState) {
    let addr = game_state.get_register16(Register::HL);
    let result = general_sla(game_state, game_state.read(addr));
    game_state.write(result, addr);
}

fn general_sra(game_state: &mut GameState, val: u8) -> u8 {
    let old_lsb = val & 1;
    let msb_mask = val & 0x80;
    let result = (val >> 1) | msb_mask;
    let new_flags = Flags {
	Z: result == 0,
	N: false,
	H: false,
	C: old_lsb == 1
    };
    game_state.set_flags(&new_flags);
    result
}

pub fn sra_r8(game_state: &mut GameState, r: Register) {
    let result = general_sra(game_state, game_state.get_register8(r));
    game_state.set_register8(r, result);
}

pub fn sra_hladdr(game_state: &mut GameState) {
    let addr = game_state.get_register16(Register::HL);
    let result = general_sra(game_state, game_state.read(addr));
    game_state.write(result, addr);
}

fn general_srl(game_state: &mut GameState, val: u8) -> u8 {
    let old_lsb = val & 1;
    let result = val >> 1;
    let new_flags = Flags {
	Z: result == 0,
	N: false,
	H: false,
	C: old_lsb == 1
    };
    game_state.set_flags(&new_flags);
    result
}

pub fn srl_r8(game_state: &mut GameState, r: Register) {
    let result = general_srl(game_state, game_state.get_register8(r));
    game_state.set_register8(r, result);
}

pub fn srl_hladdr(game_state: &mut GameState) {
    let addr = game_state.get_register16(Register::HL);
    let result = general_srl(game_state, game_state.read(addr));
    game_state.write(result, addr);
}

fn swap_general(game_state: &mut GameState, val: u8) -> u8 {
    let lower_mask = val << 4;
    let upper_mask = val >> 4;
    let result = lower_mask | upper_mask;
    let new_flags = Flags {
	Z: result == 0,
	N: false,
	H: false,
	C: false
    };
    game_state.set_flags(&new_flags);
    result
}

pub fn swap_r8(game_state: &mut GameState, r: Register) {
    let result = swap_general(game_state, game_state.get_register8(r));
    game_state.set_register8(r, result);
}

pub fn swap_hladdr(game_state: &mut GameState) {
    let addr = game_state.get_register16(Register::HL);
    let result = swap_general(game_state, game_state.read(addr));
    game_state.write(result, addr);
}

// TODO Control Flow
pub fn call_n16(game_state: &mut GameState) {
    let next_addr = game_state.get_register16(Register::PC) + 3;
    let lsb = (next_addr & 0x00FF) as u8;
    let msb = (next_addr >> 8) as u8;
    dec_sp(game_state);
    game_state.write(msb,  game_state.get_register16(Register::SP));
    dec_sp(game_state);
    game_state.write(lsb,  game_state.get_register16(Register::SP));
    jp_n16(game_state);
}

pub fn call_cc(game_state: &mut GameState, z: bool, n: bool, c: bool) {
    let flags = game_state.get_flags();
    if ((n && z) && (flags.N && flags.Z)) || ((n && c) && (flags.N && flags.C)) || (z && flags.Z) || (c && flags.C) {
	call_n16(game_state);
    }
}

pub fn jp_hl(game_state: &mut GameState) {
    game_state.set_register16(Register::PC, game_state.get_register16(Register::HL));
}

pub fn jp_n16(game_state: &mut GameState) {
    let lsb = game_state.read(game_state.get_register16(Register::PC) + 1);
    let msb = game_state.read(game_state.get_register16(Register::PC) + 2);
    let jump_addr = ((msb as u16) << 8) | (lsb as u16);
    game_state.set_register16(Register::PC, jump_addr);
}

pub fn jp_cc(game_state: &mut GameState, z: bool, n: bool, c: bool) {
    let flags = game_state.get_flags();
    if ((n && z) && (flags.N && flags.Z)) || ((n && c) && (flags.N && flags.C)) || (z && flags.Z) || (c && flags.C) {
	jp_n16(game_state);
    }
}

pub fn jr_n16(game_state: &mut GameState) {
    let lsb = game_state.read(game_state.get_register16(Register::PC) + 1);
    let msb = game_state.read(game_state.get_register16(Register::PC) + 2);
    let jump_addr = ((game_state.get_register16(Register::PC) + 3) as u32 as i32) + ((((msb as u16) << 8) | (lsb as u16)) as i16 as i32);
    game_state.set_register16(Register::PC, jump_addr as u16);
}

pub fn jr_cc(game_state: &mut GameState, z: bool, n: bool, c: bool) {
    let flags = game_state.get_flags();
    if ((n && z) && (flags.N && flags.Z)) || ((n && c) && (flags.N && flags.C)) || (z && flags.Z) || (c && flags.C) {
	jr_n16(game_state);
    }
}

pub fn ret(game_state: &mut GameState) {
    pop_r16(game_state, Register::PC);
}

pub fn reti(game_state: &mut GameState) {
    game_state.set_interrupts(true);
    pop_r16(game_state, Register::PC);
}

pub fn ret_cc(game_state: &mut GameState, z: bool, n: bool, c: bool) {
    let flags = game_state.get_flags();
    if ((n && z) && (flags.N && flags.Z)) || ((n && c) && (flags.N && flags.C)) || (z && flags.Z) || (c && flags.C) {
	ret(game_state);
    }
}

pub fn rst_vec(game_state: &mut GameState, vec: u8) {
    let next_addr = game_state.get_register16(Register::PC) + 1;
    let n_lsb = (next_addr & 0x00FF) as u8;
    let n_msb = (next_addr >> 8) as u8;
    dec_sp(game_state);
    game_state.write(n_msb,  game_state.get_register16(Register::SP));
    dec_sp(game_state);
    game_state.write(n_lsb,  game_state.get_register16(Register::SP));
    let lsb = vec;
    let msb = 0u8;
    let jump_addr = ((msb as u16) << 8) | (lsb as u16);
    game_state.set_register16(Register::PC, jump_addr);
}

// Carry Flag
pub fn ccf(game_state: &mut GameState)  {
    let old_flags = game_state.get_flags();
    let new_flags = Flags {
	Z: old_flags.Z,
	N: false,
	H: false,
	C: !old_flags.C
    };
    game_state.set_flags(&new_flags);
}

pub fn scf(game_state: &mut GameState)  {
    let old_flags = game_state.get_flags();
    let new_flags = Flags {
	Z: old_flags.Z,
	N: false,
	H: false,
	C: true
    };
    game_state.set_flags(&new_flags);
}

// TODO Stack Manipulation

pub fn add_hl_sp(game_state: &mut GameState) {
    add_hl_r16(game_state, Register::SP);
}

// returns 16-bit sum value, bit 3 overflow, and bit 7 overflow
fn add16_special(a: u16, b: u16) -> (u16, bool, bool) {
    let sum = (a as u32) + (b as u32);

    let half_carry = ((a & 0xF) + (b & 0xF)) > 0xF;
    let carry_out = sum > 0xFF;

    (sum as u16, half_carry, carry_out)
}

pub fn add_sp_e8(game_state: &mut GameState) {
    let e = game_state.read(game_state.get_register16(Register::PC) + 1) as i8;
    let (result, half_carry, carry_out) = add16_special(game_state.get_register16(Register::SP), e as i16 as u16);
    game_state.set_register16(Register::SP, result);

    let new_flags = Flags{
	Z: false,
	N: false,
	H: half_carry,
	C: carry_out
    };

    game_state.set_flags(&new_flags);
}

pub fn dec_sp(game_state: &mut GameState) {
    game_state.set_register16(Register::SP, game_state.get_register16(Register::SP) - 1);
}

pub fn inc_sp(game_state: &mut GameState) {
    game_state.set_register16(Register::SP, game_state.get_register16(Register::SP) + 1);
}

pub fn ld_hl_spe8(game_state: &mut GameState) {
    let e = game_state.read(game_state.get_register16(Register::PC) + 1) as i8;
    let (result, half_carry, carry_out) = add16_special(game_state.get_register16(Register::SP), e as i16 as u16);
    game_state.set_register16(Register::SP, result);
    game_state.set_register16(Register::HL, result);
    
    let new_flags = Flags{
	Z: false,
	N: false,
	H: half_carry,
	C: carry_out
    };

    game_state.set_flags(&new_flags);
}

pub fn ld_n16addr_sp(game_state: &mut GameState) {
    let lsb = game_state.read(game_state.get_register16(Register::PC) + 1);
    let msb = game_state.read(game_state.get_register16(Register::PC) + 2);
    let addr = ((msb as u16) << 8) | (lsb as u16);
    let sp_val = game_state.get_register16(Register::SP);
    let val1 =  (sp_val & 0xFF) as u8;
    let val2 = (sp_val >> 8) as u8;
    game_state.write(val1, addr);
    game_state.write(val2, addr + 1);
}

pub fn ld_sp_n16addr(game_state: &mut GameState) {
    let lsb = game_state.read(game_state.get_register16(Register::PC) + 1);
    let msb = game_state.read(game_state.get_register16(Register::PC) + 2);
    let val = ((msb as u16) << 8) | (lsb as u16);
    game_state.set_register16(Register::SP, val);
}

pub fn ld_sp_hl(game_state: &mut GameState) {
    game_state.set_register16(Register::SP, game_state.get_register16(Register::HL));
}


pub fn pop_r16(game_state: &mut GameState, r: Register) {
    
}

pub fn push_r16(game_state: &mut GameState, r: Register) {

}


// Interrupts
pub fn di(game_state: &mut GameState) {
    game_state.set_interrupts(false);
}

pub fn ei(game_state: &mut GameState) {
    game_state.set_interrupts(true);
}

// TODO Misc
pub fn daa(game_state: &mut GameState) {

}

pub fn stop(game_state: &mut GameState) {
}

// TODO HALT Implementation
pub fn halt(game_state: &mut GameState) {
}
