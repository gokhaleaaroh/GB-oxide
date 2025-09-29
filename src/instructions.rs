// https://rgbds.gbdev.io/docs/v0.9.4/gbz80.7 - reference manual

use crate::constants::{INT_JOYPAD, INT_LCD, INT_SERIAL, INT_TIMER, INT_VBLANK};
use crate::state::{Flags, GameState, Register, CC};

// Load instructions
pub fn ld_r8_r8(game_state: &mut GameState, r1: Register, r2: Register) -> u8 {
    game_state.set_register8(r1, game_state.get_register8(r2));
    1
}

pub fn ld_r8_n8(game_state: &mut GameState, r1: Register) -> u8 {
    let val = game_state.read(game_state.get_register16(Register::PC) + 1);
    game_state.set_register8(r1, val);
    2
}

pub fn ld_r16_n16(game_state: &mut GameState, r1: Register) -> u8 {
    let lsb = game_state.read(game_state.get_register16(Register::PC) + 1);
    let msb = game_state.read(game_state.get_register16(Register::PC) + 2);
    let val = ((msb as u16) << 8) | (lsb as u16);
    game_state.set_register16(r1, val);
    2
}

pub fn ld_hladdr_r8(game_state: &mut GameState, r: Register) -> u8 {
    game_state.write(
        game_state.get_register8(r),
        game_state.get_register16(Register::HL),
    );

    2
}

pub fn ld_hladdr_n8(game_state: &mut GameState) -> u8 {
    let val = game_state.read(game_state.get_register16(Register::PC) + 1);
    game_state.write(val, game_state.get_register16(Register::HL));

    3
}

pub fn ld_r8_hladdr(game_state: &mut GameState, r: Register) -> u8 {
    game_state.set_register8(r, game_state.read(game_state.get_register16(Register::HL)));

    2
}

pub fn ld_r16addr_a(game_state: &mut GameState, r: Register) -> u8 {
    game_state.write(
        game_state.get_register8(Register::A),
        game_state.get_register16(r),
    );

    2
}

pub fn ld_n16addr_a(game_state: &mut GameState) -> u8 {
    let lsb = game_state.read(game_state.get_register16(Register::PC) + 1);
    let msb = game_state.read(game_state.get_register16(Register::PC) + 2);
    let addr = ((msb as u16) << 8) | (lsb as u16);
    game_state.write(game_state.get_register8(Register::A), addr);

    4
}

pub fn ldh_n8addr_a(game_state: &mut GameState) -> u8 {
    let lsb = game_state.read(game_state.get_register16(Register::PC) + 1);
    let addr = 0xFF00 | (lsb as u16);
    game_state.write(game_state.get_register8(Register::A), addr);

    3
}

pub fn ldh_caddr_a(game_state: &mut GameState) -> u8 {
    game_state.write(
        game_state.get_register8(Register::A),
        0xFF00 + game_state.get_register8(Register::C) as u16,
    );

    2
}

pub fn ld_a_r16addr(game_state: &mut GameState, r: Register) -> u8 {
    game_state.set_register8(Register::A, game_state.read(game_state.get_register16(r)));

    2
}

pub fn ld_a_n16addr(game_state: &mut GameState) -> u8 {
    let lsb = game_state.read(game_state.get_register16(Register::PC) + 1);
    let msb = game_state.read(game_state.get_register16(Register::PC) + 2);
    let addr = ((msb as u16) << 8) | (lsb as u16);
    game_state.set_register8(Register::A, game_state.read(addr));
    4
}

pub fn ldh_a_n8addr(game_state: &mut GameState) -> u8 {
    let lsb = game_state.read(game_state.get_register16(Register::PC) + 1);
    let addr = 0xFF00 | (lsb as u16);
    game_state.set_register8(Register::A, game_state.read(addr));
    3
}

pub fn ldh_a_caddr(game_state: &mut GameState) -> u8 {
    game_state.set_register8(
        Register::A,
        game_state.read(game_state.get_register8(Register::C) as u16 + 0xFF00),
    );

    2
}

pub fn ld_hliaddr_a(game_state: &mut GameState) -> u8 {
    ld_hladdr_r8(game_state, Register::A);
    game_state.set_register16(
        Register::HL,
        game_state.get_register16(Register::HL).wrapping_add(1),
    );

    2
}

pub fn ld_hldaddr_a(game_state: &mut GameState) -> u8 {
    ld_hladdr_r8(game_state, Register::A);
    game_state.set_register16(
        Register::HL,
        game_state.get_register16(Register::HL).wrapping_sub(1),
    );

    2
}

pub fn ld_a_hld(game_state: &mut GameState) -> u8 {
    ld_a_r16addr(game_state, Register::HL);
    game_state.set_register16(
        Register::HL,
        game_state.get_register16(Register::HL).wrapping_sub(1),
    );
    2
}

pub fn ld_a_hli(game_state: &mut GameState) -> u8 {
    ld_a_r16addr(game_state, Register::HL);
    game_state.set_register16(
        Register::HL,
        game_state.get_register16(Register::HL).wrapping_add(1),
    );
    2
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
    let c: u8 = if carry_on && game_state.get_flags().C {
        1
    } else {
        0
    };
    let (result, half_carry, carry_out) = add8(game_state.get_register8(Register::A), val, c);

    game_state.set_register8(Register::A, result);

    let new_flags = Flags {
        Z: result == 0,
        N: false,
        H: half_carry,
        C: carry_out,
    };

    game_state.set_flags(&new_flags);
}

pub fn adc_a_r8(game_state: &mut GameState, r: Register) -> u8 {
    general_add_a_n8(game_state, game_state.get_register8(r), true);
    1
}

pub fn adc_a_hladdr(game_state: &mut GameState) -> u8 {
    general_add_a_n8(
        game_state,
        game_state.read(game_state.get_register16(Register::HL)),
        true,
    );
    2
}

pub fn adc_a_n8(game_state: &mut GameState) -> u8 {
    let val = game_state.read(game_state.get_register16(Register::PC) + 1);
    general_add_a_n8(game_state, val, true);
    2
}

pub fn add_a_r8(game_state: &mut GameState, r: Register) -> u8 {
    general_add_a_n8(game_state, game_state.get_register8(r), false);
    1
}

pub fn add_a_hladdr(game_state: &mut GameState) -> u8 {
    general_add_a_n8(
        game_state,
        game_state.read(game_state.get_register16(Register::HL)),
        false,
    );
    2
}

pub fn add_a_n8(game_state: &mut GameState) -> u8 {
    let val = game_state.read(game_state.get_register16(Register::PC) + 1);
    general_add_a_n8(game_state, val, false);
    2
}

// returns 16-bit sum value, bit 11 overflow, and bit 15 overflow
fn add16(a: u16, b: u16) -> (u16, bool, bool) {
    let sum = (a as u32) + (b as u32);

    let half_carry = ((a & 0xFFF) + (b & 0xFFF)) > 0xFFF;
    let carry_out = sum > 0xFFFF;

    (sum as u16, half_carry, carry_out)
}

pub fn add_hl_r16(game_state: &mut GameState, r: Register) -> u8 {
    let (result, half_carry, carry_out) = add16(
        game_state.get_register16(Register::HL),
        game_state.get_register16(r),
    );
    game_state.set_register16(Register::HL, result);

    let new_flags = Flags {
        Z: game_state.get_flags().Z,
        N: false,
        H: half_carry,
        C: carry_out,
    };

    game_state.set_flags(&new_flags);
    2
}

// Subtract Instructions

// 8-bit difference, bit 4 borrow, final borrow
fn sub8(a: u8, b: u8, carry_in: u8) -> (u8, bool, bool) {
    let result = a.wrapping_sub(b);

    let half_borrow = (a & 0xF) < (b & 0xF) + carry_in;
    let borrow = (a as u16) < (b as u16 + carry_in as u16);

    (result, half_borrow, borrow)
}

fn general_sub_a_n8(game_state: &mut GameState, val: u8, borrow_on: bool, discard: bool) {
    let c: u8 = if borrow_on && game_state.get_flags().C {
        1
    } else {
        0
    };
    let (result, half_borrow, borrow) = sub8(game_state.get_register8(Register::A), val, c);

    if !discard {
        game_state.set_register8(Register::A, result);
    }

    let new_flags = Flags {
        Z: result == 0,
        N: true,
        H: half_borrow,
        C: borrow,
    };

    game_state.set_flags(&new_flags);
}

pub fn sbc_a_r8(game_state: &mut GameState, r: Register) -> u8 {
    general_sub_a_n8(game_state, game_state.get_register8(r), true, false);
    1
}

pub fn sbc_a_hladdr(game_state: &mut GameState) -> u8 {
    general_sub_a_n8(
        game_state,
        game_state.read(game_state.get_register16(Register::HL)),
        true,
        false,
    );
    2
}

pub fn sbc_a_n8(game_state: &mut GameState) -> u8 {
    let val = game_state.read(game_state.get_register16(Register::PC) + 1);
    general_sub_a_n8(game_state, val, true, false);
    2
}

pub fn sub_a_r8(game_state: &mut GameState, r: Register) -> u8 {
    general_sub_a_n8(game_state, game_state.get_register8(r), false, false);
    1
}

pub fn sub_a_hladdr(game_state: &mut GameState) -> u8 {
    general_sub_a_n8(
        game_state,
        game_state.read(game_state.get_register16(Register::HL)),
        false,
        false,
    );
    2
}

pub fn sub_a_n8(game_state: &mut GameState) -> u8 {
    let val = game_state.read(game_state.get_register16(Register::PC) + 1);
    general_sub_a_n8(game_state, val, false, false);
    2
}

// Compare Instructions
pub fn cp_a_r8(game_state: &mut GameState, r: Register) -> u8 {
    general_sub_a_n8(game_state, game_state.get_register8(r), false, true);
    1
}

pub fn cp_a_hladdr(game_state: &mut GameState) -> u8 {
    general_sub_a_n8(
        game_state,
        game_state.read(game_state.get_register16(Register::HL)),
        false,
        true,
    );
    2
}

pub fn cp_a_n8(game_state: &mut GameState) -> u8 {
    let val = game_state.read(game_state.get_register16(Register::PC) + 1);
    general_sub_a_n8(game_state, val, false, true);
    2
}

// Decrease Instructions
pub fn dec_r8(game_state: &mut GameState, r: Register) -> u8 {
    if matches!(r, Register::B) {
        println!("Current val of Reg B: {}", game_state.get_register8(r));
    }
    let (result, half_borrow, _) = sub8(game_state.get_register8(r), 1, 0);
    game_state.set_register8(r, result);
    if matches!(r, Register::B) {
        println!("New val of Reg B: {}", game_state.get_register8(r));
    }

    let new_flags = Flags {
        Z: result == 0,
        N: true,
        H: half_borrow,
        C: game_state.get_flags().C,
    };

    game_state.set_flags(&new_flags);
    println!("NEW Z_FLAG: {}", game_state.get_flags().Z);
    1
}

pub fn dec_hladdr(game_state: &mut GameState) -> u8 {
    let addr = game_state.get_register16(Register::HL);
    let (result, half_borrow, _) = sub8(game_state.read(addr), 1, 0);
    game_state.write(result, addr);

    let new_flags = Flags {
        Z: result == 0,
        N: true,
        H: half_borrow,
        C: game_state.get_flags().C,
    };

    game_state.set_flags(&new_flags);
    3
}

pub fn dec_r16(game_state: &mut GameState, r: Register) -> u8 {
    game_state.set_register16(r, game_state.get_register16(r) - (1 as u16));
    2
}

// Increase Instructions
pub fn inc_r8(game_state: &mut GameState, r: Register) -> u8 {
    let (result, half_carry, _) = add8(game_state.get_register8(r), 1, 0);

    game_state.set_register8(r, result);

    let new_flags = Flags {
        Z: result == 0,
        N: false,
        H: half_carry,
        C: game_state.get_flags().C,
    };

    game_state.set_flags(&new_flags);
    1
}

pub fn inc_hladdr(game_state: &mut GameState) -> u8 {
    let addr = game_state.get_register16(Register::HL);
    let (result, half_carry, _) = add8(game_state.read(addr), 1, 0);
    game_state.write(result, addr);

    let new_flags = Flags {
        Z: result == 0,
        N: false,
        H: half_carry,
        C: game_state.get_flags().C,
    };

    game_state.set_flags(&new_flags);
    3
}

pub fn inc_r16(game_state: &mut GameState, r: Register) -> u8 {
    game_state.set_register16(r, game_state.get_register16(r) + (1 as u16));
    2
}

// Bitwise Logic
fn general_and_a(game_state: &mut GameState, val: u8) -> u8 {
    let result = game_state.get_register8(Register::A) & val;
    let new_flags = Flags {
        Z: result == 0,
        N: false,
        H: true,
        C: false,
    };

    game_state.set_flags(&new_flags);
    result
}

pub fn and_a_r8(game_state: &mut GameState, r: Register) -> u8 {
    let result = general_and_a(game_state, game_state.get_register8(r));
    game_state.set_register8(Register::A, result);
    1
}

pub fn and_a_hladdr(game_state: &mut GameState) -> u8 {
    let addr = game_state.get_register16(Register::HL);
    let result = general_and_a(game_state, game_state.read(addr));
    game_state.write(result, addr);
    2
}

pub fn and_a_n8(game_state: &mut GameState) -> u8 {
    let val = game_state.read(game_state.get_register16(Register::PC) + 1);
    let result = general_and_a(game_state, val);
    game_state.set_register8(Register::A, result);
    2
}

pub fn cpl(game_state: &mut GameState) -> u8 {
    game_state.set_register8(Register::A, !game_state.get_register8(Register::A));

    let old_flags = game_state.get_flags();

    let new_flags = Flags {
        Z: old_flags.Z,
        N: true,
        H: true,
        C: old_flags.C,
    };

    game_state.set_flags(&new_flags);
    1
}

fn general_or_a(game_state: &mut GameState, val: u8, exclusive: bool) -> u8 {
    let a = game_state.get_register8(Register::A);
    let result = if exclusive { a ^ val } else { a | val };
    let new_flags = Flags {
        Z: result == 0,
        N: false,
        H: false,
        C: false,
    };

    game_state.set_flags(&new_flags);
    result
}

pub fn or_a_r8(game_state: &mut GameState, r: Register) -> u8 {
    let result = general_or_a(game_state, game_state.get_register8(r), false);
    game_state.set_register8(Register::A, result);
    1
}

pub fn or_a_hladdr(game_state: &mut GameState) -> u8 {
    let addr = game_state.get_register16(Register::HL);
    let result = general_or_a(game_state, game_state.read(addr), false);
    game_state.write(result, addr);
    2
}

pub fn or_a_n8(game_state: &mut GameState) -> u8 {
    let val = game_state.read(game_state.get_register16(Register::PC) + 1);
    let result = general_or_a(game_state, val, false);
    game_state.set_register8(Register::A, result);
    2
}

pub fn xor_a_r8(game_state: &mut GameState, r: Register) -> u8 {
    let result = general_or_a(game_state, game_state.get_register8(r), true);
    game_state.set_register8(Register::A, result);
    1
}

pub fn xor_a_hladdr(game_state: &mut GameState) -> u8 {
    let addr = game_state.get_register16(Register::HL);
    let result = general_or_a(game_state, game_state.read(addr), true);
    game_state.write(result, addr);
    2
}

pub fn xor_a_n8(game_state: &mut GameState) -> u8 {
    let val = game_state.read(game_state.get_register16(Register::PC) + 1);
    let result = general_or_a(game_state, val, true);
    game_state.set_register8(Register::A, result);
    2
}

// Bit Flag
pub fn bit_u3_r8(game_state: &mut GameState, u: u8, r: Register) -> u8 {
    let zero_flag = (game_state.get_register8(r) & (1 << u)) == 0;
    let new_flags = Flags {
        Z: zero_flag,
        N: false,
        H: true,
        C: game_state.get_flags().C,
    };

    game_state.set_flags(&new_flags);
    2
}

pub fn bit_u3_hladdr(game_state: &mut GameState, u: u8) -> u8 {
    let zero_flag = (game_state.read(game_state.get_register16(Register::HL)) & (1 << u)) == 0;
    let new_flags = Flags {
        Z: zero_flag,
        N: false,
        H: true,
        C: game_state.get_flags().C,
    };

    game_state.set_flags(&new_flags);
    3
}

pub fn res_u3_r8(game_state: &mut GameState, u: u8, r: Register) -> u8 {
    game_state.set_register8(r, game_state.get_register8(r) & !(1 << u));
    2
}

pub fn res_u3_hladdr(game_state: &mut GameState, u: u8) -> u8 {
    let addr = game_state.get_register16(Register::HL);
    game_state.write(game_state.read(addr) & !(1 << u), addr);
    4
}

pub fn set_u3_r8(game_state: &mut GameState, u: u8, r: Register) -> u8 {
    game_state.set_register8(r, game_state.get_register8(r) | (1 << u));
    2
}

pub fn set_u3_hladdr(game_state: &mut GameState, u: u8) -> u8 {
    let addr = game_state.get_register16(Register::HL);
    game_state.write(game_state.read(addr) | (1 << u), addr);
    4
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
        C: new_c == 1,
    };
    game_state.set_flags(&new_flags);
    result
}

pub fn rl_r8(game_state: &mut GameState, r: Register) -> u8 {
    let result = general_rl(game_state, game_state.get_register8(r), true);
    game_state.set_register8(r, result);
    2
}

pub fn rl_hladdr(game_state: &mut GameState) -> u8 {
    let addr = game_state.get_register16(Register::HL);
    let result = general_rl(game_state, game_state.read(addr), true);
    game_state.write(result, addr);
    4
}

pub fn rla(game_state: &mut GameState) -> u8 {
    let result = general_rl(game_state, game_state.get_register8(Register::A), false);
    game_state.set_register8(Register::A, result);
    1
}

fn general_rlc(game_state: &mut GameState, val: u8, check_zero: bool) -> u8 {
    let old_msb = val >> 7;
    let result = (val << 1) | old_msb;
    let new_flags = Flags {
        Z: result == 0 && check_zero,
        N: false,
        H: false,
        C: old_msb == 1,
    };
    game_state.set_flags(&new_flags);
    result
}

pub fn rlc_r8(game_state: &mut GameState, r: Register) -> u8 {
    let result = general_rlc(game_state, game_state.get_register8(r), true);
    game_state.set_register8(r, result);
    2
}

pub fn rlc_hladdr(game_state: &mut GameState) -> u8 {
    let addr = game_state.get_register16(Register::HL);
    let result = general_rlc(game_state, game_state.read(addr), true);
    game_state.write(result, addr);
    4
}

pub fn rlca(game_state: &mut GameState) -> u8 {
    let result = general_rlc(game_state, game_state.get_register8(Register::A), false);
    game_state.set_register8(Register::A, result);
    1
}

fn general_rr(game_state: &mut GameState, val: u8, check_zero: bool) -> u8 {
    let old_c = if game_state.get_flags().C { 1 } else { 0 };
    let new_c = val & 1;
    let result = (val >> 1) | (old_c << 7);
    let new_flags = Flags {
        Z: result == 0 && check_zero,
        N: false,
        H: false,
        C: new_c == 1,
    };
    game_state.set_flags(&new_flags);
    result
}

pub fn rr_r8(game_state: &mut GameState, r: Register) -> u8 {
    let result = general_rr(game_state, game_state.get_register8(r), true);
    game_state.set_register8(r, result);
    2
}

pub fn rr_hladdr(game_state: &mut GameState) -> u8 {
    let addr = game_state.get_register16(Register::HL);
    let result = general_rr(game_state, game_state.read(addr), true);
    game_state.write(result, addr);
    4
}

pub fn rra(game_state: &mut GameState) -> u8 {
    let result = general_rr(game_state, game_state.get_register8(Register::A), false);
    game_state.set_register8(Register::A, result);
    1
}

fn general_rrc(game_state: &mut GameState, val: u8, check_zero: bool) -> u8 {
    let old_lsb = val & 1;
    let result = (val >> 1) | (old_lsb << 7);
    let new_flags = Flags {
        Z: result == 0 && check_zero,
        N: false,
        H: false,
        C: old_lsb == 1,
    };
    game_state.set_flags(&new_flags);
    result
}

pub fn rrc_r8(game_state: &mut GameState, r: Register) -> u8 {
    let result = general_rrc(game_state, game_state.get_register8(r), true);
    game_state.set_register8(r, result);
    2
}

pub fn rrc_hladdr(game_state: &mut GameState) -> u8 {
    let addr = game_state.get_register16(Register::HL);
    let result = general_rrc(game_state, game_state.read(addr), true);
    game_state.write(result, addr);
    4
}

pub fn rrca(game_state: &mut GameState) -> u8 {
    let result = general_rrc(game_state, game_state.get_register8(Register::A), false);
    game_state.set_register8(Register::A, result);
    1
}

fn general_sla(game_state: &mut GameState, val: u8) -> u8 {
    let old_msb = val >> 7;
    let result = val << 1;
    let new_flags = Flags {
        Z: result == 0,
        N: false,
        H: false,
        C: old_msb == 1,
    };
    game_state.set_flags(&new_flags);
    result
}

pub fn sla_r8(game_state: &mut GameState, r: Register) -> u8 {
    let result = general_sla(game_state, game_state.get_register8(r));
    game_state.set_register8(r, result);
    2
}

pub fn sla_hladdr(game_state: &mut GameState) -> u8 {
    let addr = game_state.get_register16(Register::HL);
    let result = general_sla(game_state, game_state.read(addr));
    game_state.write(result, addr);
    4
}

fn general_sra(game_state: &mut GameState, val: u8) -> u8 {
    let old_lsb = val & 1;
    let msb_mask = val & 0x80;
    let result = (val >> 1) | msb_mask;
    let new_flags = Flags {
        Z: result == 0,
        N: false,
        H: false,
        C: old_lsb == 1,
    };
    game_state.set_flags(&new_flags);
    result
}

pub fn sra_r8(game_state: &mut GameState, r: Register) -> u8 {
    let result = general_sra(game_state, game_state.get_register8(r));
    game_state.set_register8(r, result);
    2
}

pub fn sra_hladdr(game_state: &mut GameState) -> u8 {
    let addr = game_state.get_register16(Register::HL);
    let result = general_sra(game_state, game_state.read(addr));
    game_state.write(result, addr);
    4
}

fn general_srl(game_state: &mut GameState, val: u8) -> u8 {
    let old_lsb = val & 1;
    let result = val >> 1;
    let new_flags = Flags {
        Z: result == 0,
        N: false,
        H: false,
        C: old_lsb == 1,
    };
    game_state.set_flags(&new_flags);
    result
}

pub fn srl_r8(game_state: &mut GameState, r: Register) -> u8 {
    let result = general_srl(game_state, game_state.get_register8(r));
    game_state.set_register8(r, result);
    2
}

pub fn srl_hladdr(game_state: &mut GameState) -> u8 {
    let addr = game_state.get_register16(Register::HL);
    let result = general_srl(game_state, game_state.read(addr));
    game_state.write(result, addr);
    4
}

fn swap_general(game_state: &mut GameState, val: u8) -> u8 {
    let lower_mask = val << 4;
    let upper_mask = val >> 4;
    let result = lower_mask | upper_mask;
    let new_flags = Flags {
        Z: result == 0,
        N: false,
        H: false,
        C: false,
    };
    game_state.set_flags(&new_flags);
    result
}

pub fn swap_r8(game_state: &mut GameState, r: Register) -> u8 {
    let result = swap_general(game_state, game_state.get_register8(r));
    game_state.set_register8(r, result);
    2
}

pub fn swap_hladdr(game_state: &mut GameState) -> u8 {
    let addr = game_state.get_register16(Register::HL);
    let result = swap_general(game_state, game_state.read(addr));
    game_state.write(result, addr);
    4
}

pub fn call_n16(game_state: &mut GameState) -> u8 {
    let next_addr = game_state.get_register16(Register::PC) + 3;
    let lsb = (next_addr & 0x00FF) as u8;
    let msb = (next_addr >> 8) as u8;
    dec_sp(game_state);
    game_state.write(msb, game_state.get_register16(Register::SP));
    dec_sp(game_state);
    game_state.write(lsb, game_state.get_register16(Register::SP));
    jp_n16(game_state);
    6
}

pub fn call_cc(game_state: &mut GameState, cc: CC) -> u8 {
    let flags = game_state.get_flags();
    if (matches!(cc, CC::Z) && flags.Z)
        || (matches!(cc, CC::NZ) && !flags.Z)
        || (matches!(cc, CC::C) && flags.C)
        || (matches!(cc, CC::NC) && !flags.C)
    {
        call_n16(game_state);
        return 6;
    }
    3
}

pub fn jp_hl(game_state: &mut GameState) -> u8 {
    game_state.set_register16(Register::PC, game_state.get_register16(Register::HL));
    game_state.set_pc_moved(true);
    1
}

pub fn jp_n16(game_state: &mut GameState) -> u8 {
    let lsb = game_state.read(game_state.get_register16(Register::PC) + 1);
    let msb = game_state.read(game_state.get_register16(Register::PC) + 2);
    let jump_addr = ((msb as u16) << 8) | (lsb as u16);
    game_state.set_register16(Register::PC, jump_addr);
    game_state.set_pc_moved(true);
    4
}

pub fn jp_cc(game_state: &mut GameState, cc: CC) -> u8 {
    let flags = game_state.get_flags();
    if (matches!(cc, CC::Z, ) && flags.Z)
        || (matches!(cc, CC::NZ, ) && !flags.Z)
        || (matches!(cc, CC::C) && flags.C)
        || (matches!(cc, CC::NC) && !flags.C)
    {
        jp_n16(game_state);
        return 4;
    }
    3
}

pub fn jr_e8(game_state: &mut GameState) -> u8 {
    let offset = game_state.read(game_state.get_register16(Register::PC) + 1) as i8;
    let jump_addr = (game_state.get_register16(Register::PC) as i16 + 2) + offset as i16;
    game_state.set_register16(Register::PC, jump_addr as u16);
    game_state.set_pc_moved(true);
    3
}

pub fn jr_cc(game_state: &mut GameState, cc: CC) -> u8 {
    let flags = game_state.get_flags();
    if (matches!(cc, CC::Z) && flags.Z)
        || (matches!(cc, CC::NZ) && !flags.Z)
        || (matches!(cc, CC::C) && flags.C)
        || (matches!(cc, CC::NC) && !flags.C)
    {
        print!(" JUMPING RELATIVELY ");
	println!("Z_FLAG: {}", flags.Z);
        jr_e8(game_state);
        return 3;
    }
    2
}

pub fn ret(game_state: &mut GameState) -> u8 {
    pop_r16(game_state, Register::PC);
    game_state.set_pc_moved(true);
    4
}

pub fn reti(game_state: &mut GameState) -> u8 {
    game_state.set_interrupts(true);
    pop_r16(game_state, Register::PC);
    game_state.set_pc_moved(true);
    4
}

pub fn ret_cc(game_state: &mut GameState, cc: CC) -> u8 {
    let flags = game_state.get_flags();
    if (matches!(cc, CC::Z) && flags.Z)
        || (matches!(cc, CC::NZ) && !flags.Z)
        || (matches!(cc, CC::C) && flags.C)
        || (matches!(cc, CC::NC) && !flags.C)
    {
        ret(game_state);
        return 5;
    }
    2
}

pub fn rst_vec(game_state: &mut GameState, vec: u8) -> u8 {
    let next_addr = game_state.get_register16(Register::PC) + 1;
    let n_lsb = (next_addr & 0x00FF) as u8;
    let n_msb = (next_addr >> 8) as u8;
    dec_sp(game_state);
    game_state.write(n_msb, game_state.get_register16(Register::SP));
    dec_sp(game_state);
    game_state.write(n_lsb, game_state.get_register16(Register::SP));
    let lsb = vec;
    let msb = 0u8;
    let jump_addr = ((msb as u16) << 8) | (lsb as u16);
    game_state.set_register16(Register::PC, jump_addr);
    game_state.set_pc_moved(true);
    4
}

// Carry Flag
pub fn ccf(game_state: &mut GameState) -> u8 {
    let old_flags = game_state.get_flags();
    let new_flags = Flags {
        Z: old_flags.Z,
        N: false,
        H: false,
        C: !old_flags.C,
    };
    game_state.set_flags(&new_flags);
    1
}

pub fn scf(game_state: &mut GameState) -> u8 {
    let old_flags = game_state.get_flags();
    let new_flags = Flags {
        Z: old_flags.Z,
        N: false,
        H: false,
        C: true,
    };
    game_state.set_flags(&new_flags);
    1
}

pub fn add_hl_sp(game_state: &mut GameState) -> u8 {
    add_hl_r16(game_state, Register::SP);
    2
}

// returns 16-bit sum value, bit 3 overflow, and bit 7 overflow
fn add16_special(a: u16, b: u16) -> (u16, bool, bool) {
    let sum = (a as u32) + (b as u32);

    let half_carry = ((a & 0xF) + (b & 0xF)) > 0xF;
    let carry_out = sum > 0xFF;

    (sum as u16, half_carry, carry_out)
}

pub fn add_sp_e8(game_state: &mut GameState) -> u8 {
    let e = game_state.read(game_state.get_register16(Register::PC) + 1) as i8;
    let (result, half_carry, carry_out) =
        add16_special(game_state.get_register16(Register::SP), e as i16 as u16);
    game_state.set_register16(Register::SP, result);

    let new_flags = Flags {
        Z: false,
        N: false,
        H: half_carry,
        C: carry_out,
    };

    game_state.set_flags(&new_flags);
    4
}

pub fn dec_sp(game_state: &mut GameState) -> u8 {
    game_state.set_register16(Register::SP, game_state.get_register16(Register::SP) - 1);
    2
}

pub fn inc_sp(game_state: &mut GameState) -> u8 {
    game_state.set_register16(Register::SP, game_state.get_register16(Register::SP) + 1);
    2
}

pub fn ld_hl_spe8(game_state: &mut GameState) -> u8 {
    let e = game_state.read(game_state.get_register16(Register::PC) + 1) as i8;
    let (result, half_carry, carry_out) =
        add16_special(game_state.get_register16(Register::SP), e as i16 as u16);
    game_state.set_register16(Register::SP, result);
    game_state.set_register16(Register::HL, result);

    let new_flags = Flags {
        Z: false,
        N: false,
        H: half_carry,
        C: carry_out,
    };

    game_state.set_flags(&new_flags);
    3
}

pub fn ld_n16addr_sp(game_state: &mut GameState) -> u8 {
    let lsb = game_state.read(game_state.get_register16(Register::PC) + 1);
    let msb = game_state.read(game_state.get_register16(Register::PC) + 2);
    let addr = ((msb as u16) << 8) | (lsb as u16);
    let sp_val = game_state.get_register16(Register::SP);
    let val1 = (sp_val & 0xFF) as u8;
    let val2 = (sp_val >> 8) as u8;
    game_state.write(val1, addr);
    game_state.write(val2, addr + 1);
    5
}

pub fn ld_sp_n16addr(game_state: &mut GameState) -> u8 {
    let lsb = game_state.read(game_state.get_register16(Register::PC) + 1);
    let msb = game_state.read(game_state.get_register16(Register::PC) + 2);
    let val = ((msb as u16) << 8) | (lsb as u16);
    game_state.set_register16(Register::SP, val);
    3
}

pub fn ld_sp_hl(game_state: &mut GameState) -> u8 {
    game_state.set_register16(Register::SP, game_state.get_register16(Register::HL));
    2
}

pub fn pop_r16(game_state: &mut GameState, r: Register) -> u8 {
    let lsb = game_state.read(game_state.get_register16(Register::SP));
    inc_sp(game_state);
    let msb = game_state.read(game_state.get_register16(Register::SP));
    inc_sp(game_state);
    let val = ((msb as u16) << 8) | (lsb as u16);
    game_state.set_register16(r, val);
    3
}

pub fn push_r16(game_state: &mut GameState, r: Register) -> u8 {
    let reg_val = game_state.get_register16(r);
    let lsb = (reg_val & 0x00FF) as u8;
    let msb = (reg_val >> 8) as u8;
    dec_sp(game_state);
    game_state.write(msb, game_state.get_register16(Register::SP));
    dec_sp(game_state);
    game_state.write(lsb, game_state.get_register16(Register::SP));
    4
}

// Interrupts
pub fn di(game_state: &mut GameState) -> u8 {
    game_state.set_interrupts(false);
    1
}

pub fn ei(game_state: &mut GameState) -> u8 {
    game_state.set_interrupts(true);
    1
}

// Misc
pub fn daa(game_state: &mut GameState) -> u8 {
    let flags = game_state.get_flags();

    let mut adjustment: i16 = 0;
    let a_val = game_state.get_register8(Register::A);
    let mut new_flags = Flags {
        Z: flags.Z,
        N: flags.N,
        H: false,
        C: flags.C,
    };

    if flags.N {
        if flags.H {
            adjustment += 0x06
        };
        if flags.C {
            adjustment += 0x60
        };
    } else {
        if flags.H || (a_val & 0x0F) > 0x09 {
            adjustment += 0x06;
        }
        if flags.C || a_val > 0x99 {
            adjustment += 0x60;
            new_flags.C = true
        }
    }

    let result = ((a_val as i16) - (adjustment)) as u16 as u8;
    game_state.set_register8(Register::A, result);

    if result == 0 {
        new_flags.Z = true;
    }
    game_state.set_flags(&new_flags);
    1
}

pub fn stop(game_state: &mut GameState) -> u8 {
    0
}

// TODO HALT Implementation
pub fn halt(game_state: &mut GameState) -> u8 {
    0
}

pub fn interrupt_handler(game_state: &mut GameState) {
    let i_flag = game_state.read(0xFF0F);
    if (i_flag << 3) != 0 {
        game_state.set_interrupts(false);
        let jump_addr;
        if i_flag & INT_VBLANK != 0 {
            game_state.write(i_flag & !INT_VBLANK, 0xFF0F);
            jump_addr = 0x0040;
        } else if i_flag & INT_LCD != 0 {
            game_state.write(i_flag & !INT_LCD, 0xFF0F);
            jump_addr = 0x0048;
        } else if i_flag & INT_TIMER != 0 {
            game_state.write(i_flag & !INT_TIMER, 0xFF0F);
            jump_addr = 0x0050;
        } else if i_flag & INT_SERIAL != 0 {
            game_state.write(i_flag & !INT_SERIAL, 0xFF0F);
            jump_addr = 0x0058;
        } else {
            game_state.write(i_flag & !INT_JOYPAD, 0xFF0F);
            jump_addr = 0x0060;
        }

        let curr_addr = game_state.get_register16(Register::PC);
        let lsb = (curr_addr & 0x00FF) as u8;
        let msb = (curr_addr >> 8) as u8;
        dec_sp(game_state);
        game_state.write(msb, game_state.get_register16(Register::SP));
        dec_sp(game_state);
        game_state.write(lsb, game_state.get_register16(Register::SP));
        game_state.set_register16(Register::PC, jump_addr);
        game_state.set_pc_moved(true);
        game_state.update_clock(5);
    }
}
