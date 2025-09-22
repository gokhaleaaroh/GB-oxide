use crate::state::{GameState, Register};
use crate::instructions::*;

type InstructionWrapper = fn(&mut GameState);

pub struct Decoder {
    non_prefix_opcodes: [InstructionWrapper; 256],
    cb_prefix_opcodes:  [InstructionWrapper; 256]
}

impl Decoder {
    fn initialize() -> Self {
	Self {
	    non_prefix_opcodes: [
		|_: &mut GameState| {}, // 0x00
		|game_state: &mut GameState| ld_r16_n16(game_state, Register::BC), // 0x01
		|game_state: &mut GameState| ld_r16addr_a(game_state, Register::BC), // 0x02
		|game_state: &mut GameState| inc_r16(game_state, Register::BC), // 0x03    
		|game_state: &mut GameState| inc_r8(game_state, Register::B), // 0x04
		|game_state: &mut GameState| dec_r8(game_state, Register::B), // 0x05
		|game_state: &mut GameState| ld_r8_n8(game_state, Register::B), // 0x06
		|game_state: &mut GameState| rlca(game_state), // 0x07
		|game_state: &mut GameState| ld_n16addr_sp(game_state), // 0x08
		|game_state: &mut GameState| add_hl_r16(game_state, Register::BC), // 0x09
		|game_state: &mut GameState| ld_a_r16addr(game_state, Register::BC), // 0x0A
		|game_state: &mut GameState| dec_r16(game_state, Register::BC), // 0x0B
		|game_state: &mut GameState| inc_r8(game_state, Register::C), // 0x0C
		|game_state: &mut GameState| dec_r8(game_state, Register::C), // 0x0D
		|game_state: &mut GameState| ld_r8_n8(game_state, Register::C), // 0x0E
		|game_state: &mut GameState| rrc_a(game_state), // 0x0F

		|game_state: &mut GameState| stop(game_state), // 0x10
		|game_state: &mut GameState| ld_r16_n16(game_state, Register::DE), // 0x11
		|game_state: &mut GameState| ld_r16addr_a(game_state, Register::DE), // 0x12    
		|game_state: &mut GameState| inc_r16(game_state, Register::DE), // 0x13
		|game_state: &mut GameState| inc_r8(game_state, Register::D), // 0x14
		|game_state: &mut GameState| dec_r8(game_state, Register::D), // 0x15
		|game_state: &mut GameState| ld_r8_n8(game_state, Register::D), // 0x16
		|game_state: &mut GameState| rl_a(game_state), // 0x17
		|game_state: &mut GameState| jr_n16(game_state), // 0x18
		|game_state: &mut GameState| add_hl_r16(game_state, Register::DE), // 0x19
		|game_state: &mut GameState| ld_a_r16addr(game_state, Register::DE), // 0x1A
		|game_state: &mut GameState| dec_r16(game_state, Register::DE), // 0x1B
		|game_state: &mut GameState| inc_r8(game_state, Register::E), // 0x1C
		|game_state: &mut GameState| dec_r8(game_state, Register::E), // 0x1D
		|game_state: &mut GameState| ld_r8_n8(game_state, Register::E), // 0x1E
		|game_state: &mut GameState| rr_a(game_state), // 0x1F

		|game_state: &mut GameState| jr_cc(game_state, true, true, false), // 0x20
		|game_state: &mut GameState| ld_r16_n16(game_state, Register::HL), // 0x21
		|game_state: &mut GameState| ld_hliaddr_a(game_state), // 0x22    
		|game_state: &mut GameState| inc_r16(game_state, Register::HL), // 0x23
		|game_state: &mut GameState| inc_r8(game_state, Register::H), // 0x24
		|game_state: &mut GameState| dec_r8(game_state, Register::H), // 0x25
		|game_state: &mut GameState| ld_r8_n8(game_state, Register::H), // 0x26
		|game_state: &mut GameState| daa(game_state), // 0x27
		|game_state: &mut GameState| jr_cc(game_state, true, false, false), // 0x28
		|game_state: &mut GameState| add_hl_r16(game_state, Register::HL), // 0x29
		|game_state: &mut GameState| ld_a_hli(game_state), // 0x2A
		|game_state: &mut GameState| dec_r16(game_state, Register::HL), // 0x2B
		|game_state: &mut GameState| inc_r8(game_state, Register::L), // 0x2C
		|game_state: &mut GameState| dec_r8(game_state, Register::L), // 0x2D
		|game_state: &mut GameState| ld_r8_n8(game_state, Register::L), // 0x2E
		|game_state: &mut GameState| cpl(game_state), // 0x2F

		|game_state: &mut GameState| jr_cc(game_state, false, true, true), // 0x30
		|game_state: &mut GameState| ld_sp_n16addr(game_state), // 0x31
		|game_state: &mut GameState| ld_hldaddr_a(game_state), // 0x32    
		|game_state: &mut GameState| inc_sp(game_state), // 0x33
		|game_state: &mut GameState| inc_hladdr(game_state), // 0x34
		|game_state: &mut GameState| dec_hladdr(game_state), // 0x35
		|game_state: &mut GameState| ld_hladdr_n8(game_state), // 0x36
		|game_state: &mut GameState| scf(game_state), // 0x37
		|game_state: &mut GameState| jr_cc(game_state, false, false, true), // 0x38
		|game_state: &mut GameState| add_hl_sp(game_state), // 0x39
		|game_state: &mut GameState| ld_a_hld(game_state), // 0x3A
		|game_state: &mut GameState| dec_sp(game_state), // 0x3B
		|game_state: &mut GameState| inc_r8(game_state, Register::A), // 0x3C
		|game_state: &mut GameState| dec_r8(game_state, Register::A), // 0x3D
		|game_state: &mut GameState| ld_r8_n8(game_state, Register::A), // 0x3E
		|game_state: &mut GameState| ccf(game_state), // 0x3F

		|game_state: &mut GameState| ld_r8_r8(game_state, Register::B, Register::B), // 0x40
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::B, Register::C), // 0x41
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::B, Register::D), // 0x42
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::B, Register::E), // 0x43
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::B, Register::H), // 0x44
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::B, Register::L), // 0x45
		|game_state: &mut GameState| ld_r8_hladdr(game_state, Register::B), // 0x46
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::B, Register::A), // 0x47
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::C, Register::B), // 0x48
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::C, Register::C), // 0x49
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::C, Register::D), // 0x4A
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::C, Register::E), // 0x4B
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::C, Register::H), // 0x4C
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::C, Register::L), // 0x4D
		|game_state: &mut GameState| ld_r8_hladdr(game_state, Register::C), // 0x4E
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::C, Register::A), // 0x4F

		|game_state: &mut GameState| ld_r8_r8(game_state, Register::D, Register::B), // 0x50
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::D, Register::C), // 0x51
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::D, Register::D), // 0x52
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::D, Register::E), // 0x53
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::D, Register::H), // 0x54
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::D, Register::L), // 0x55
		|game_state: &mut GameState| ld_r8_hladdr(game_state, Register::D), // 0x56
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::D, Register::A), // 0x57
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::E, Register::B), // 0x58
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::E, Register::C), // 0x59
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::E, Register::D), // 0x5A
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::E, Register::E), // 0x5B
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::E, Register::H), // 0x5C
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::E, Register::L), // 0x5D
		|game_state: &mut GameState| ld_r8_hladdr(game_state, Register::E), // 0x5E
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::E, Register::A), // 0x5F

		|game_state: &mut GameState| ld_r8_r8(game_state, Register::H, Register::B), // 0x60
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::H, Register::C), // 0x61
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::H, Register::D), // 0x62
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::H, Register::E), // 0x63
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::H, Register::H), // 0x64
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::H, Register::L), // 0x65
		|game_state: &mut GameState| ld_r8_hladdr(game_state, Register::H), // 0x66
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::H, Register::A), // 0x67
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::L, Register::B), // 0x68
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::L, Register::C), // 0x69
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::L, Register::D), // 0x6A
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::L, Register::E), // 0x6B
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::L, Register::H), // 0x6C
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::L, Register::L), // 0x6D
		|game_state: &mut GameState| ld_r8_hladdr(game_state, Register::L), // 0x6E
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::L, Register::A), // 0x6F

		|game_state: &mut GameState| ld_hladdr_r8(game_state, Register::B), // 0x70
		|game_state: &mut GameState| ld_hladdr_r8(game_state, Register::C), // 0x71
		|game_state: &mut GameState| ld_hladdr_r8(game_state, Register::D), // 0x72
		|game_state: &mut GameState| ld_hladdr_r8(game_state, Register::E), // 0x73
		|game_state: &mut GameState| ld_hladdr_r8(game_state, Register::H), // 0x74
		|game_state: &mut GameState| ld_hladdr_r8(game_state, Register::L), // 0x75
		|game_state: &mut GameState| halt(game_state), // 0x76
		|game_state: &mut GameState| ld_hladdr_r8(game_state, Register::A), // 0x75
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::A, Register::B), // 0x78
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::A, Register::C), // 0x79
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::A, Register::D), // 0x7A
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::A, Register::E), // 0x7B
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::A, Register::H), // 0x7C
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::A, Register::L), // 0x7D
		|game_state: &mut GameState| ld_r8_hladdr(game_state, Register::A), // 0x7E
		|game_state: &mut GameState| ld_r8_r8(game_state, Register::A, Register::A), // 0x7F
		
		|game_state: &mut GameState| add_a_r8(game_state, Register::B), // 0x80
		|game_state: &mut GameState| add_a_r8(game_state, Register::C), // 0x81
		|game_state: &mut GameState| add_a_r8(game_state, Register::D), // 0x82
		|game_state: &mut GameState| add_a_r8(game_state, Register::E), // 0x83
		|game_state: &mut GameState| add_a_r8(game_state, Register::H), // 0x84
		|game_state: &mut GameState| add_a_r8(game_state, Register::L), // 0x85
		|game_state: &mut GameState| add_a_hladdr(game_state), // 0x86
		|game_state: &mut GameState| add_a_r8(game_state, Register::A), // 0x87
		|game_state: &mut GameState| adc_a_r8(game_state, Register::B), // 0x88
		|game_state: &mut GameState| adc_a_r8(game_state, Register::C), // 0x89
		|game_state: &mut GameState| adc_a_r8(game_state, Register::D), // 0x8A
		|game_state: &mut GameState| adc_a_r8(game_state, Register::E), // 0x8B
		|game_state: &mut GameState| adc_a_r8(game_state, Register::H), // 0x8C
		|game_state: &mut GameState| adc_a_r8(game_state, Register::L), // 0x8D
		|game_state: &mut GameState| adc_a_hladdr(game_state), // 0x8E
		|game_state: &mut GameState| adc_a_r8(game_state, Register::A), // 0x8F

		|game_state: &mut GameState| sub_a_r8(game_state, Register::B), // 0x90
		|game_state: &mut GameState| sub_a_r8(game_state, Register::C), // 0x91
		|game_state: &mut GameState| sub_a_r8(game_state, Register::D), // 0x92
		|game_state: &mut GameState| sub_a_r8(game_state, Register::E), // 0x93
		|game_state: &mut GameState| sub_a_r8(game_state, Register::H), // 0x94
		|game_state: &mut GameState| sub_a_r8(game_state, Register::L), // 0x95
		|game_state: &mut GameState| sub_a_hladdr(game_state), // 0x96
		|game_state: &mut GameState| sub_a_r8(game_state, Register::A), // 0x97
		|game_state: &mut GameState| sbc_a_r8(game_state, Register::B), // 0x98
		|game_state: &mut GameState| sbc_a_r8(game_state, Register::C), // 0x99
		|game_state: &mut GameState| sbc_a_r8(game_state, Register::D), // 0x9A
		|game_state: &mut GameState| sbc_a_r8(game_state, Register::E), // 0x9B
		|game_state: &mut GameState| sbc_a_r8(game_state, Register::H), // 0x9C
		|game_state: &mut GameState| sbc_a_r8(game_state, Register::L), // 0x9D
		|game_state: &mut GameState| sbc_a_hladdr(game_state), // 0x9E
		|game_state: &mut GameState| sbc_a_r8(game_state, Register::A), // 0x9F

		|game_state: &mut GameState| and_a_r8(game_state, Register::B), // 0xA0
		|game_state: &mut GameState| and_a_r8(game_state, Register::C), // 0xA1
		|game_state: &mut GameState| and_a_r8(game_state, Register::D), // 0xA2
		|game_state: &mut GameState| and_a_r8(game_state, Register::E), // 0xA3
		|game_state: &mut GameState| and_a_r8(game_state, Register::H), // 0xA4
		|game_state: &mut GameState| and_a_r8(game_state, Register::L), // 0xA5
		|game_state: &mut GameState| and_a_hladdr(game_state), // 0xA6
		|game_state: &mut GameState| and_a_r8(game_state, Register::A), // 0xA7
		|game_state: &mut GameState| xor_a_r8(game_state, Register::B), // 0xA8
		|game_state: &mut GameState| xor_a_r8(game_state, Register::C), // 0xA9
		|game_state: &mut GameState| xor_a_r8(game_state, Register::D), // 0xAA
		|game_state: &mut GameState| xor_a_r8(game_state, Register::E), // 0xAB
		|game_state: &mut GameState| xor_a_r8(game_state, Register::H), // 0xAC
		|game_state: &mut GameState| xor_a_r8(game_state, Register::L), // 0xAD
		|game_state: &mut GameState| xor_a_hladdr(game_state), // 0xAE
		|game_state: &mut GameState| xor_a_r8(game_state, Register::A), // 0xAF

		|game_state: &mut GameState| or_a_r8(game_state, Register::B), // 0xB0
		|game_state: &mut GameState| or_a_r8(game_state, Register::C), // 0xB1
		|game_state: &mut GameState| or_a_r8(game_state, Register::D), // 0xB2
		|game_state: &mut GameState| or_a_r8(game_state, Register::E), // 0xB3
		|game_state: &mut GameState| or_a_r8(game_state, Register::H), // 0xB4
		|game_state: &mut GameState| or_a_r8(game_state, Register::L), // 0xB5
		|game_state: &mut GameState| or_a_hladdr(game_state), // 0xB6
		|game_state: &mut GameState| or_a_r8(game_state, Register::A), // 0xB7
		|game_state: &mut GameState| cp_a_r8(game_state, Register::B), // 0xB8
		|game_state: &mut GameState| cp_a_r8(game_state, Register::C), // 0xB9
		|game_state: &mut GameState| cp_a_r8(game_state, Register::D), // 0xBA
		|game_state: &mut GameState| cp_a_r8(game_state, Register::E), // 0xBB
		|game_state: &mut GameState| cp_a_r8(game_state, Register::H), // 0xBC
		|game_state: &mut GameState| cp_a_r8(game_state, Register::L), // 0xBD
		|game_state: &mut GameState| cp_a_hladdr(game_state), // 0xBE
		|game_state: &mut GameState| cp_a_r8(game_state, Register::A), // 0xBF

		|game_state: &mut GameState| ret_cc(game_state, true, true, false), // 0xC0
		|game_state: &mut GameState| pop_r16(game_state, Register::BC), // 0xC1
		|game_state: &mut GameState| jp_cc(game_state, true, true, false), // 0xC2
		|game_state: &mut GameState| jp_n16(game_state), // 0xC3
		|game_state: &mut GameState| call_cc(game_state, true, true, false), // 0xC4
		|game_state: &mut GameState| push_r16(game_state, Register::BC), // 0xC5
		|game_state: &mut GameState| add_a_n8(game_state), // 0xC6
		|game_state: &mut GameState| rst_vec(game_state, 0x00), // 0xC7
		|game_state: &mut GameState| ret_cc(game_state, true, false, false), // 0xC8
		|game_state: &mut GameState| ret(game_state), // 0xC9
		|game_state: &mut GameState| jp_cc(game_state, true, false, false), // 0xCA
		|_: &mut GameState| {}, // 0xCB PREFIX!
		|game_state: &mut GameState| call_cc(game_state, true, false, false), // 0xCC
		|game_state: &mut GameState| call_n16(game_state), // 0xCD
		|game_state: &mut GameState| adc_a_n8(game_state), // 0xCE
		|game_state: &mut GameState| rst_vec(game_state, 0x08), // 0xCF

		|game_state: &mut GameState| ret_cc(game_state, false, true, true), // 0xD0
		|game_state: &mut GameState| pop_r16(game_state, Register::DE), // 0xD1
		|game_state: &mut GameState| jp_cc(game_state, false, true, true), // 0xD2
		|_: &mut GameState| {}, // 0xD3 Blank
		|game_state: &mut GameState| call_cc(game_state, false, true, true), // 0xD4
		|game_state: &mut GameState| push_r16(game_state, Register::DE), // 0xD5
		|game_state: &mut GameState| sub_a_n8(game_state), // 0xD6
		|game_state: &mut GameState| rst_vec(game_state, 0x10), // 0xD7
		|game_state: &mut GameState| ret_cc(game_state, false, false, true), // 0xD8
		|game_state: &mut GameState| reti(game_state), // 0xD9
		|game_state: &mut GameState| jp_cc(game_state, false, false, true), // 0xDA
		|_: &mut GameState| {}, // 0xDB Blank
		|game_state: &mut GameState| call_cc(game_state, false, false, true), // 0xDC
		|_: &mut GameState| {}, // 0xDD Blank
		|game_state: &mut GameState| sbc_a_n8(game_state), // 0xDE
		|game_state: &mut GameState| rst_vec(game_state, 0x18), // 0xDF

		|game_state: &mut GameState| ldh_n16addr_a(game_state), // 0xE0
		|game_state: &mut GameState| pop_r16(game_state, Register::HL), // 0xE1
		|game_state: &mut GameState| ldh_caddr_a(game_state), // 0xE2
		|_: &mut GameState| {}, // 0xE3 Blank
		|_: &mut GameState| {}, // 0xE4 Blank
		|game_state: &mut GameState| push_r16(game_state, Register::HL), // 0xE5
		|game_state: &mut GameState| and_a_n8(game_state), // 0xE6
		|game_state: &mut GameState| rst_vec(game_state, 0x20), // 0xE7
		|game_state: &mut GameState| add_sp_e8(game_state), // 0xE8
		|game_state: &mut GameState| jp_hl(game_state), // 0xE9
		|game_state: &mut GameState| ld_n16addr_a(game_state), // 0xEA
		|_: &mut GameState| {}, // 0xEB Blank
		|_: &mut GameState| {}, // 0xEC Blank
		|_: &mut GameState| {}, // 0xED Blank
		|game_state: &mut GameState| xor_a_n8(game_state), // 0xEE
		|game_state: &mut GameState| rst_vec(game_state, 0x28), // 0xEF

		|game_state: &mut GameState| ldh_a_n16addr(game_state), // 0xF0
		|game_state: &mut GameState| pop_r16(game_state, Register::AF), // 0xF1
		|game_state: &mut GameState| ldh_a_caddr(game_state), // 0xF2
		|game_state: &mut GameState| di(game_state), // 0xF3 Blank
		|_: &mut GameState| {}, // 0xF4 Blank
		|game_state: &mut GameState| push_r16(game_state, Register::AF), // 0xF5
		|game_state: &mut GameState| or_a_n8(game_state), // 0xF6
		|game_state: &mut GameState| rst_vec(game_state, 0x30), // 0xF7
		|game_state: &mut GameState| ld_hl_spe8(game_state), // 0xF8
		|game_state: &mut GameState| ld_sp_hl(game_state), // 0xF9
		|game_state: &mut GameState| ld_a_n16addr(game_state), // 0xFA
		|game_state: &mut GameState| ei(game_state), // 0xFB Blank
		|_: &mut GameState| {}, // 0xFC Blank
		|_: &mut GameState| {}, // 0xFD Blank
		|game_state: &mut GameState| cp_a_n8(game_state), // 0xFE
		|game_state: &mut GameState| rst_vec(game_state, 0x38), // 0xFF
	    ],

	    cb_prefix_opcodes: [
		|game_state: &mut GameState| rlc_r8(game_state, Register::B), // 0x00
		|game_state: &mut GameState| rlc_r8(game_state, Register::C), // 0x01
		|game_state: &mut GameState| rlc_r8(game_state, Register::D), // 0x02
		|game_state: &mut GameState| rlc_r8(game_state, Register::E), // 0x03
		|game_state: &mut GameState| rlc_r8(game_state, Register::H), // 0x04
		|game_state: &mut GameState| rlc_r8(game_state, Register::L), // 0x05
		|game_state: &mut GameState| rlc_hladdr(game_state), // 0x06
		|game_state: &mut GameState| rrc_r8(game_state, Register::A), // 0x07
		|game_state: &mut GameState| rrc_r8(game_state, Register::B), // 0x08
		|game_state: &mut GameState| rrc_r8(game_state, Register::C), // 0x09
		|game_state: &mut GameState| rrc_r8(game_state, Register::D), // 0x0A
		|game_state: &mut GameState| rrc_r8(game_state, Register::E), // 0x0B
		|game_state: &mut GameState| rrc_r8(game_state, Register::H), // 0x0C
		|game_state: &mut GameState| rrc_r8(game_state, Register::L), // 0x0D
		|game_state: &mut GameState| rrc_hladdr(game_state), // 0x0E
		|game_state: &mut GameState| rrc_r8(game_state, Register::A), // 0x0F

		|game_state: &mut GameState| rl_r8(game_state, Register::B), // 0x10
		|game_state: &mut GameState| rl_r8(game_state, Register::C), // 0x11
		|game_state: &mut GameState| rl_r8(game_state, Register::D), // 0x12
		|game_state: &mut GameState| rl_r8(game_state, Register::E), // 0x13
		|game_state: &mut GameState| rl_r8(game_state, Register::H), // 0x14
		|game_state: &mut GameState| rl_r8(game_state, Register::L), // 0x15
		|game_state: &mut GameState| rl_hladdr(game_state), // 0x16
		|game_state: &mut GameState| rr_r8(game_state, Register::A), // 0x17
		|game_state: &mut GameState| rr_r8(game_state, Register::B), // 0x18
		|game_state: &mut GameState| rr_r8(game_state, Register::C), // 0x19
		|game_state: &mut GameState| rr_r8(game_state, Register::D), // 0x1A
		|game_state: &mut GameState| rr_r8(game_state, Register::E), // 0x1B
		|game_state: &mut GameState| rr_r8(game_state, Register::H), // 0x1C
		|game_state: &mut GameState| rr_r8(game_state, Register::L), // 0x1D
		|game_state: &mut GameState| rr_hladdr(game_state), // 0x1E
		|game_state: &mut GameState| rr_r8(game_state, Register::A), // 0x1F

		|game_state: &mut GameState| sla_r8(game_state, Register::B), // 0x20
		|game_state: &mut GameState| sla_r8(game_state, Register::C), // 0x21
		|game_state: &mut GameState| sla_r8(game_state, Register::D), // 0x22
		|game_state: &mut GameState| sla_r8(game_state, Register::E), // 0x23
		|game_state: &mut GameState| sla_r8(game_state, Register::H), // 0x24
		|game_state: &mut GameState| sla_r8(game_state, Register::L), // 0x25
		|game_state: &mut GameState| sla_hladdr(game_state), // 0x26
		|game_state: &mut GameState| sra_r8(game_state, Register::A), // 0x27
		|game_state: &mut GameState| sra_r8(game_state, Register::B), // 0x28
		|game_state: &mut GameState| sra_r8(game_state, Register::C), // 0x29
		|game_state: &mut GameState| sra_r8(game_state, Register::D), // 0x2A
		|game_state: &mut GameState| sra_r8(game_state, Register::E), // 0x2B
		|game_state: &mut GameState| sra_r8(game_state, Register::H), // 0x2C
		|game_state: &mut GameState| sra_r8(game_state, Register::L), // 0x2D
		|game_state: &mut GameState| sra_hladdr(game_state), // 0x2E
		|game_state: &mut GameState| sra_r8(game_state, Register::A), // 0x2F

		|game_state: &mut GameState| swap_r8(game_state, Register::B), // 0x30
		|game_state: &mut GameState| swap_r8(game_state, Register::C), // 0x31
		|game_state: &mut GameState| swap_r8(game_state, Register::D), // 0x32
		|game_state: &mut GameState| swap_r8(game_state, Register::E), // 0x33
		|game_state: &mut GameState| swap_r8(game_state, Register::H), // 0x34
		|game_state: &mut GameState| swap_r8(game_state, Register::L), // 0x35
		|game_state: &mut GameState| swap_hladdr(game_state), // 0x36
		|game_state: &mut GameState| swap_r8(game_state, Register::A), // 0x37
		|game_state: &mut GameState| srl_r8(game_state, Register::B), // 0x38
		|game_state: &mut GameState| srl_r8(game_state, Register::C), // 0x39
		|game_state: &mut GameState| srl_r8(game_state, Register::D), // 0x3A
		|game_state: &mut GameState| srl_r8(game_state, Register::E), // 0x3B
		|game_state: &mut GameState| srl_r8(game_state, Register::H), // 0x3C
		|game_state: &mut GameState| srl_r8(game_state, Register::L), // 0x3D
		|game_state: &mut GameState| srl_hladdr(game_state), // 0x3E
		|game_state: &mut GameState| srl_r8(game_state, Register::A), // 0x3F

		|game_state: &mut GameState| bit_u3_r8(game_state, 0, Register::B), // 0x40
		|game_state: &mut GameState| bit_u3_r8(game_state, 0, Register::C), // 0x41
		|game_state: &mut GameState| bit_u3_r8(game_state, 0, Register::D), // 0x42
		|game_state: &mut GameState| bit_u3_r8(game_state, 0, Register::E), // 0x43
		|game_state: &mut GameState| bit_u3_r8(game_state, 0, Register::H), // 0x44
		|game_state: &mut GameState| bit_u3_r8(game_state, 0, Register::L), // 0x45
		|game_state: &mut GameState| bit_u3_hladdr(game_state, 0), // 0x46
		|game_state: &mut GameState| bit_u3_r8(game_state, 0, Register::A), // 0x47
		|game_state: &mut GameState| bit_u3_r8(game_state, 1, Register::B), // 0x48
		|game_state: &mut GameState| bit_u3_r8(game_state, 1, Register::C), // 0x49
		|game_state: &mut GameState| bit_u3_r8(game_state, 1, Register::D), // 0x4A
		|game_state: &mut GameState| bit_u3_r8(game_state, 1, Register::E), // 0x4B
		|game_state: &mut GameState| bit_u3_r8(game_state, 1, Register::H), // 0x4C
		|game_state: &mut GameState| bit_u3_r8(game_state, 1, Register::L), // 0x4D
		|game_state: &mut GameState| bit_u3_hladdr(game_state, 1), // 0x4E
		|game_state: &mut GameState| bit_u3_r8(game_state, 1, Register::A), // 0x4F

		|game_state: &mut GameState| bit_u3_r8(game_state, 2, Register::B), // 0x50
		|game_state: &mut GameState| bit_u3_r8(game_state, 2, Register::C), // 0x51
		|game_state: &mut GameState| bit_u3_r8(game_state, 2, Register::D), // 0x52
		|game_state: &mut GameState| bit_u3_r8(game_state, 2, Register::E), // 0x53
		|game_state: &mut GameState| bit_u3_r8(game_state, 2, Register::H), // 0x54
		|game_state: &mut GameState| bit_u3_r8(game_state, 2, Register::L), // 0x55
		|game_state: &mut GameState| bit_u3_hladdr(game_state, 2), // 0x56
		|game_state: &mut GameState| bit_u3_r8(game_state, 2, Register::A), // 0x57
		|game_state: &mut GameState| bit_u3_r8(game_state, 3, Register::B), // 0x58
		|game_state: &mut GameState| bit_u3_r8(game_state, 3, Register::C), // 0x59
		|game_state: &mut GameState| bit_u3_r8(game_state, 3, Register::D), // 0x5A
		|game_state: &mut GameState| bit_u3_r8(game_state, 3, Register::E), // 0x5B
		|game_state: &mut GameState| bit_u3_r8(game_state, 3, Register::H), // 0x5C
		|game_state: &mut GameState| bit_u3_r8(game_state, 3, Register::L), // 0x5D
		|game_state: &mut GameState| bit_u3_hladdr(game_state, 3), // 0x5E
		|game_state: &mut GameState| bit_u3_r8(game_state, 3, Register::A), // 0x5F

		|game_state: &mut GameState| bit_u3_r8(game_state, 4, Register::B), // 0x60
		|game_state: &mut GameState| bit_u3_r8(game_state, 4, Register::C), // 0x61
		|game_state: &mut GameState| bit_u3_r8(game_state, 4, Register::D), // 0x62
		|game_state: &mut GameState| bit_u3_r8(game_state, 4, Register::E), // 0x63
		|game_state: &mut GameState| bit_u3_r8(game_state, 4, Register::H), // 0x64
		|game_state: &mut GameState| bit_u3_r8(game_state, 4, Register::L), // 0x65
		|game_state: &mut GameState| bit_u3_hladdr(game_state, 4), // 0x66
		|game_state: &mut GameState| bit_u3_r8(game_state, 4, Register::A), // 0x67
		|game_state: &mut GameState| bit_u3_r8(game_state, 5, Register::B), // 0x68
		|game_state: &mut GameState| bit_u3_r8(game_state, 5, Register::C), // 0x69
		|game_state: &mut GameState| bit_u3_r8(game_state, 5, Register::D), // 0x6A
		|game_state: &mut GameState| bit_u3_r8(game_state, 5, Register::E), // 0x6B
		|game_state: &mut GameState| bit_u3_r8(game_state, 5, Register::H), // 0x6C
		|game_state: &mut GameState| bit_u3_r8(game_state, 5, Register::L), // 0x6D
		|game_state: &mut GameState| bit_u3_hladdr(game_state, 5), // 0x6E
		|game_state: &mut GameState| bit_u3_r8(game_state, 5, Register::A), // 0x6F

		|game_state: &mut GameState| bit_u3_r8(game_state, 6, Register::B), // 0x70
		|game_state: &mut GameState| bit_u3_r8(game_state, 6, Register::C), // 0x71
		|game_state: &mut GameState| bit_u3_r8(game_state, 6, Register::D), // 0x72
		|game_state: &mut GameState| bit_u3_r8(game_state, 6, Register::E), // 0x73
		|game_state: &mut GameState| bit_u3_r8(game_state, 6, Register::H), // 0x74
		|game_state: &mut GameState| bit_u3_r8(game_state, 6, Register::L), // 0x75
		|game_state: &mut GameState| bit_u3_hladdr(game_state, 6), // 0x76
		|game_state: &mut GameState| bit_u3_r8(game_state, 6, Register::A), // 0x77
		|game_state: &mut GameState| bit_u3_r8(game_state, 7, Register::B), // 0x78
		|game_state: &mut GameState| bit_u3_r8(game_state, 7, Register::C), // 0x79
		|game_state: &mut GameState| bit_u3_r8(game_state, 7, Register::D), // 0x7A
		|game_state: &mut GameState| bit_u3_r8(game_state, 7, Register::E), // 0x7B
		|game_state: &mut GameState| bit_u3_r8(game_state, 7, Register::H), // 0x7C
		|game_state: &mut GameState| bit_u3_r8(game_state, 7, Register::L), // 0x7D
		|game_state: &mut GameState| bit_u3_hladdr(game_state, 7), // 0x7E
		|game_state: &mut GameState| bit_u3_r8(game_state, 7, Register::A), // 0x7F

		|game_state: &mut GameState| res_u3_r8(game_state, 0, Register::B), // 0x80
		|game_state: &mut GameState| res_u3_r8(game_state, 0, Register::C), // 0x81
		|game_state: &mut GameState| res_u3_r8(game_state, 0, Register::D), // 0x82
		|game_state: &mut GameState| res_u3_r8(game_state, 0, Register::E), // 0x83
		|game_state: &mut GameState| res_u3_r8(game_state, 0, Register::H), // 0x84
		|game_state: &mut GameState| res_u3_r8(game_state, 0, Register::L), // 0x85
		|game_state: &mut GameState| res_u3_hladdr(game_state, 0), // 0x86
		|game_state: &mut GameState| res_u3_r8(game_state, 0, Register::A), // 0x87
		|game_state: &mut GameState| res_u3_r8(game_state, 1, Register::B), // 0x88
		|game_state: &mut GameState| res_u3_r8(game_state, 1, Register::C), // 0x89
		|game_state: &mut GameState| res_u3_r8(game_state, 1, Register::D), // 0x8A
		|game_state: &mut GameState| res_u3_r8(game_state, 1, Register::E), // 0x8B
		|game_state: &mut GameState| res_u3_r8(game_state, 1, Register::H), // 0x8C
		|game_state: &mut GameState| res_u3_r8(game_state, 1, Register::L), // 0x8D
		|game_state: &mut GameState| res_u3_hladdr(game_state, 1), // 0x8E
		|game_state: &mut GameState| res_u3_r8(game_state, 1, Register::A), // 0x8F

		|game_state: &mut GameState| res_u3_r8(game_state, 2, Register::B), // 0x90
		|game_state: &mut GameState| res_u3_r8(game_state, 2, Register::C), // 0x91
		|game_state: &mut GameState| res_u3_r8(game_state, 2, Register::D), // 0x92
		|game_state: &mut GameState| res_u3_r8(game_state, 2, Register::E), // 0x93
		|game_state: &mut GameState| res_u3_r8(game_state, 2, Register::H), // 0x94
		|game_state: &mut GameState| res_u3_r8(game_state, 2, Register::L), // 0x95
		|game_state: &mut GameState| res_u3_hladdr(game_state, 2), // 0x96
		|game_state: &mut GameState| res_u3_r8(game_state, 2, Register::A), // 0x97
		|game_state: &mut GameState| res_u3_r8(game_state, 3, Register::B), // 0x98
		|game_state: &mut GameState| res_u3_r8(game_state, 3, Register::C), // 0x99
		|game_state: &mut GameState| res_u3_r8(game_state, 3, Register::D), // 0x9A
		|game_state: &mut GameState| res_u3_r8(game_state, 3, Register::E), // 0x9B
		|game_state: &mut GameState| res_u3_r8(game_state, 3, Register::H), // 0x9C
		|game_state: &mut GameState| res_u3_r8(game_state, 3, Register::L), // 0x9D
		|game_state: &mut GameState| res_u3_hladdr(game_state, 3), // 0x9E
		|game_state: &mut GameState| res_u3_r8(game_state, 3, Register::A), // 0x9F

		|game_state: &mut GameState| res_u3_r8(game_state, 4, Register::B), // 0xA0
		|game_state: &mut GameState| res_u3_r8(game_state, 4, Register::C), // 0xA1
		|game_state: &mut GameState| res_u3_r8(game_state, 4, Register::D), // 0xA2
		|game_state: &mut GameState| res_u3_r8(game_state, 4, Register::E), // 0xA3
		|game_state: &mut GameState| res_u3_r8(game_state, 4, Register::H), // 0xA4
		|game_state: &mut GameState| res_u3_r8(game_state, 4, Register::L), // 0xA5
		|game_state: &mut GameState| res_u3_hladdr(game_state, 4), // 0xA6
		|game_state: &mut GameState| res_u3_r8(game_state, 4, Register::A), // 0xA7
		|game_state: &mut GameState| res_u3_r8(game_state, 5, Register::B), // 0xA8
		|game_state: &mut GameState| res_u3_r8(game_state, 5, Register::C), // 0xA9
		|game_state: &mut GameState| res_u3_r8(game_state, 5, Register::D), // 0xAA
		|game_state: &mut GameState| res_u3_r8(game_state, 5, Register::E), // 0xAB
		|game_state: &mut GameState| res_u3_r8(game_state, 5, Register::H), // 0xAC
		|game_state: &mut GameState| res_u3_r8(game_state, 5, Register::L), // 0xAD
		|game_state: &mut GameState| res_u3_hladdr(game_state, 5), // 0xAE
		|game_state: &mut GameState| res_u3_r8(game_state, 5, Register::A), // 0xAF

		|game_state: &mut GameState| res_u3_r8(game_state, 6, Register::B), // 0xB0
		|game_state: &mut GameState| res_u3_r8(game_state, 6, Register::C), // 0xB1
		|game_state: &mut GameState| res_u3_r8(game_state, 6, Register::D), // 0xB2
		|game_state: &mut GameState| res_u3_r8(game_state, 6, Register::E), // 0xB3
		|game_state: &mut GameState| res_u3_r8(game_state, 6, Register::H), // 0xB4
		|game_state: &mut GameState| res_u3_r8(game_state, 6, Register::L), // 0xB5
		|game_state: &mut GameState| res_u3_hladdr(game_state, 6), // 0xB6
		|game_state: &mut GameState| res_u3_r8(game_state, 6, Register::A), // 0xB7
		|game_state: &mut GameState| res_u3_r8(game_state, 7, Register::B), // 0xB8
		|game_state: &mut GameState| res_u3_r8(game_state, 7, Register::C), // 0xB9
		|game_state: &mut GameState| res_u3_r8(game_state, 7, Register::D), // 0xBA
		|game_state: &mut GameState| res_u3_r8(game_state, 7, Register::E), // 0xBB
		|game_state: &mut GameState| res_u3_r8(game_state, 7, Register::H), // 0xBC
		|game_state: &mut GameState| res_u3_r8(game_state, 7, Register::L), // 0xBD
		|game_state: &mut GameState| res_u3_hladdr(game_state, 7), // 0xBE
		|game_state: &mut GameState| res_u3_r8(game_state, 7, Register::A), // 0xBF

		// TODO Below
		|game_state: &mut GameState| set_u3_r8(game_state, 0, Register::B), // 0xC0
		|game_state: &mut GameState| set_u3_r8(game_state, 0, Register::C), // 0xC1
		|game_state: &mut GameState| set_u3_r8(game_state, 0, Register::D), // 0xC2
		|game_state: &mut GameState| set_u3_r8(game_state, 0, Register::E), // 0xC3
		|game_state: &mut GameState| set_u3_r8(game_state, 0, Register::H), // 0xC4
		|game_state: &mut GameState| set_u3_r8(game_state, 0, Register::L), // 0xC5
		|game_state: &mut GameState| set_u3_hladdr(game_state, 0), // 0xC6
		|game_state: &mut GameState| set_u3_r8(game_state, 0, Register::A), // 0xC7
		|game_state: &mut GameState| set_u3_r8(game_state, 1, Register::B), // 0xC8
		|game_state: &mut GameState| set_u3_r8(game_state, 1, Register::C), // 0xC9
		|game_state: &mut GameState| set_u3_r8(game_state, 1, Register::D), // 0xCA
		|game_state: &mut GameState| set_u3_r8(game_state, 1, Register::E), // 0xCB
		|game_state: &mut GameState| set_u3_r8(game_state, 1, Register::H), // 0xCC
		|game_state: &mut GameState| set_u3_r8(game_state, 1, Register::L), // 0xCD
		|game_state: &mut GameState| set_u3_hladdr(game_state, 1), // 0xCE
		|game_state: &mut GameState| set_u3_r8(game_state, 1, Register::A), // 0xCF

		|game_state: &mut GameState| set_u3_r8(game_state, 2, Register::B), // 0xD0
		|game_state: &mut GameState| set_u3_r8(game_state, 2, Register::C), // 0xD1
		|game_state: &mut GameState| set_u3_r8(game_state, 2, Register::D), // 0xD2
		|game_state: &mut GameState| set_u3_r8(game_state, 2, Register::E), // 0xD3
		|game_state: &mut GameState| set_u3_r8(game_state, 2, Register::H), // 0xD4
		|game_state: &mut GameState| set_u3_r8(game_state, 2, Register::L), // 0xD5
		|game_state: &mut GameState| set_u3_hladdr(game_state, 2), // 0xD6
		|game_state: &mut GameState| set_u3_r8(game_state, 2, Register::A), // 0xD7
		|game_state: &mut GameState| set_u3_r8(game_state, 3, Register::B), // 0xD8
		|game_state: &mut GameState| set_u3_r8(game_state, 3, Register::C), // 0xD9
		|game_state: &mut GameState| set_u3_r8(game_state, 3, Register::D), // 0xDA
		|game_state: &mut GameState| set_u3_r8(game_state, 3, Register::E), // 0xDB
		|game_state: &mut GameState| set_u3_r8(game_state, 3, Register::H), // 0xDC
		|game_state: &mut GameState| set_u3_r8(game_state, 3, Register::L), // 0xDD
		|game_state: &mut GameState| set_u3_hladdr(game_state, 3), // 0xDE
		|game_state: &mut GameState| set_u3_r8(game_state, 3, Register::A), // 0xDF

		|game_state: &mut GameState| set_u3_r8(game_state, 4, Register::B), // 0xE0
		|game_state: &mut GameState| set_u3_r8(game_state, 4, Register::C), // 0xE1
		|game_state: &mut GameState| set_u3_r8(game_state, 4, Register::D), // 0xE2
		|game_state: &mut GameState| set_u3_r8(game_state, 4, Register::E), // 0xE3
		|game_state: &mut GameState| set_u3_r8(game_state, 4, Register::H), // 0xE4
		|game_state: &mut GameState| set_u3_r8(game_state, 4, Register::L), // 0xE5
		|game_state: &mut GameState| set_u3_hladdr(game_state, 4), // 0xE6
		|game_state: &mut GameState| set_u3_r8(game_state, 4, Register::A), // 0xE7
		|game_state: &mut GameState| set_u3_r8(game_state, 5, Register::B), // 0xE8
		|game_state: &mut GameState| set_u3_r8(game_state, 5, Register::C), // 0xE9
		|game_state: &mut GameState| set_u3_r8(game_state, 5, Register::D), // 0xEA
		|game_state: &mut GameState| set_u3_r8(game_state, 5, Register::E), // 0xEB
		|game_state: &mut GameState| set_u3_r8(game_state, 5, Register::H), // 0xEC
		|game_state: &mut GameState| set_u3_r8(game_state, 5, Register::L), // 0xED
		|game_state: &mut GameState| set_u3_hladdr(game_state, 5), // 0xEE
		|game_state: &mut GameState| set_u3_r8(game_state, 5, Register::A), // 0xEF

		|game_state: &mut GameState| set_u3_r8(game_state, 6, Register::B), // 0xF0
		|game_state: &mut GameState| set_u3_r8(game_state, 6, Register::C), // 0xF1
		|game_state: &mut GameState| set_u3_r8(game_state, 6, Register::D), // 0xF2
		|game_state: &mut GameState| set_u3_r8(game_state, 6, Register::E), // 0xF3
		|game_state: &mut GameState| set_u3_r8(game_state, 6, Register::H), // 0xF4
		|game_state: &mut GameState| set_u3_r8(game_state, 6, Register::L), // 0xF5
		|game_state: &mut GameState| set_u3_hladdr(game_state, 6), // 0xF6
		|game_state: &mut GameState| set_u3_r8(game_state, 6, Register::A), // 0xF7
		|game_state: &mut GameState| set_u3_r8(game_state, 7, Register::B), // 0xF8
		|game_state: &mut GameState| set_u3_r8(game_state, 7, Register::C), // 0xF9
		|game_state: &mut GameState| set_u3_r8(game_state, 7, Register::D), // 0xFA
		|game_state: &mut GameState| set_u3_r8(game_state, 7, Register::E), // 0xFB
		|game_state: &mut GameState| set_u3_r8(game_state, 7, Register::H), // 0xFC
		|game_state: &mut GameState| set_u3_r8(game_state, 7, Register::L), // 0xFD
		|game_state: &mut GameState| set_u3_hladdr(game_state, 7), // 0xFE
		|game_state: &mut GameState| set_u3_r8(game_state, 7, Register::A), // 0xFF
	    ]
	}
    }

}
