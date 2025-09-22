use crate::state::{GameState, Register};
use crate::instructions::*;

type InstructionWrapper = Box<dyn Fn(&mut GameState)>;

struct Decoder {
    non_prefix_opcodes: [InstructionWrapper; 256],
    cb_prefix_opcodes:  [InstructionWrapper; 256]
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
		Box::new(move |game_state: &mut GameState| rlca(game_state)), // 0x07
		Box::new(move |game_state: &mut GameState| ld_n16addr_sp(game_state)), // 0x08
		Box::new(move |game_state: &mut GameState| add_hl_r16(game_state, Register::BC)), // 0x09
		Box::new(move |game_state: &mut GameState| ld_a_r16addr(game_state, Register::BC)), // 0x0A
		Box::new(move |game_state: &mut GameState| dec_r16(game_state, Register::BC)), // 0x0B
		Box::new(move |game_state: &mut GameState| inc_r8(game_state, Register::C)), // 0x0C
		Box::new(move |game_state: &mut GameState| dec_r8(game_state, Register::C)), // 0x0D
		Box::new(move |game_state: &mut GameState| ld_r8_n8(game_state, Register::C)), // 0x0E
		Box::new(move |game_state: &mut GameState| rrc_a(game_state)), // 0x0F

		Box::new(move |game_state: &mut GameState| stop(game_state)), // 0x10
		Box::new(move |game_state: &mut GameState| ld_r16_n16(game_state, Register::DE)), // 0x11
		Box::new(move |game_state: &mut GameState| ld_r16addr_a(game_state, Register::DE)), // 0x12    
		Box::new(move |game_state: &mut GameState| inc_r16(game_state, Register::DE)), // 0x13
		Box::new(move |game_state: &mut GameState| inc_r8(game_state, Register::D)), // 0x14
		Box::new(move |game_state: &mut GameState| dec_r8(game_state, Register::D)), // 0x15
		Box::new(move |game_state: &mut GameState| ld_r8_n8(game_state, Register::D)), // 0x16
		Box::new(move |game_state: &mut GameState| rl_a(game_state)), // 0x17
		Box::new(move |game_state: &mut GameState| jr_n16(game_state)), // 0x18
		Box::new(move |game_state: &mut GameState| add_hl_r16(game_state, Register::DE)), // 0x19
		Box::new(move |game_state: &mut GameState| ld_a_r16addr(game_state, Register::DE)), // 0x1A
		Box::new(move |game_state: &mut GameState| dec_r16(game_state, Register::DE)), // 0x1B
		Box::new(move |game_state: &mut GameState| inc_r8(game_state, Register::E)), // 0x1C
		Box::new(move |game_state: &mut GameState| dec_r8(game_state, Register::E)), // 0x1D
		Box::new(move |game_state: &mut GameState| ld_r8_n8(game_state, Register::E)), // 0x1E
		Box::new(move |game_state: &mut GameState| rr_a(game_state)), // 0x1F

		Box::new(move |game_state: &mut GameState| jr_cc(game_state, true, true, false)), // 0x20
		Box::new(move |game_state: &mut GameState| ld_r16_n16(game_state, Register::HL)), // 0x21
		Box::new(move |game_state: &mut GameState| ld_hliaddr_a(game_state)), // 0x22    
		Box::new(move |game_state: &mut GameState| inc_r16(game_state, Register::HL)), // 0x23
		Box::new(move |game_state: &mut GameState| inc_r8(game_state, Register::H)), // 0x24
		Box::new(move |game_state: &mut GameState| dec_r8(game_state, Register::H)), // 0x25
		Box::new(move |game_state: &mut GameState| ld_r8_n8(game_state, Register::H)), // 0x26
		Box::new(move |game_state: &mut GameState| daa(game_state)), // 0x27
		Box::new(move |game_state: &mut GameState| jr_cc(game_state, true, false, false)), // 0x28
		Box::new(move |game_state: &mut GameState| add_hl_r16(game_state, Register::HL)), // 0x29
		Box::new(move |game_state: &mut GameState| ld_a_hli(game_state)), // 0x2A
		Box::new(move |game_state: &mut GameState| dec_r16(game_state, Register::HL)), // 0x2B
		Box::new(move |game_state: &mut GameState| inc_r8(game_state, Register::L)), // 0x2C
		Box::new(move |game_state: &mut GameState| dec_r8(game_state, Register::L)), // 0x2D
		Box::new(move |game_state: &mut GameState| ld_r8_n8(game_state, Register::L)), // 0x2E
		Box::new(move |game_state: &mut GameState| cpl(game_state)), // 0x2F

		Box::new(move |game_state: &mut GameState| jr_cc(game_state, false, true, true)), // 0x30
		Box::new(move |game_state: &mut GameState| ld_sp_n16addr(game_state)), // 0x31
		Box::new(move |game_state: &mut GameState| ld_hldaddr_a(game_state)), // 0x32    
		Box::new(move |game_state: &mut GameState| inc_sp(game_state)), // 0x33
		Box::new(move |game_state: &mut GameState| inc_hladdr(game_state)), // 0x34
		Box::new(move |game_state: &mut GameState| dec_hladdr(game_state)), // 0x35
		Box::new(move |game_state: &mut GameState| ld_hladdr_n8(game_state)), // 0x36
		Box::new(move |game_state: &mut GameState| scf(game_state)), // 0x37
		Box::new(move |game_state: &mut GameState| jr_cc(game_state, false, false, true)), // 0x38
		Box::new(move |game_state: &mut GameState| add_hl_sp(game_state)), // 0x39
		Box::new(move |game_state: &mut GameState| ld_a_hld(game_state)), // 0x3A
		Box::new(move |game_state: &mut GameState| dec_sp(game_state)), // 0x3B
		Box::new(move |game_state: &mut GameState| inc_r8(game_state, Register::A)), // 0x3C
		Box::new(move |game_state: &mut GameState| dec_r8(game_state, Register::A)), // 0x3D
		Box::new(move |game_state: &mut GameState| ld_r8_n8(game_state, Register::A)), // 0x3E
		Box::new(move |game_state: &mut GameState| ccf(game_state)), // 0x3F

		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::B, Register::B)), // 0x40
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::B, Register::C)), // 0x41
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::B, Register::D)), // 0x42
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::B, Register::E)), // 0x43
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::B, Register::H)), // 0x44
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::B, Register::L)), // 0x45
		Box::new(move |game_state: &mut GameState| ld_r8_hladdr(game_state, Register::B)), // 0x46
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::B, Register::A)), // 0x47
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::C, Register::B)), // 0x48
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::C, Register::C)), // 0x49
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::C, Register::D)), // 0x4A
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::C, Register::E)), // 0x4B
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::C, Register::H)), // 0x4C
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::C, Register::L)), // 0x4D
		Box::new(move |game_state: &mut GameState| ld_r8_hladdr(game_state, Register::C)), // 0x4E
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::C, Register::A)), // 0x4F

		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::D, Register::B)), // 0x50
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::D, Register::C)), // 0x51
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::D, Register::D)), // 0x52
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::D, Register::E)), // 0x53
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::D, Register::H)), // 0x54
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::D, Register::L)), // 0x55
		Box::new(move |game_state: &mut GameState| ld_r8_hladdr(game_state, Register::D)), // 0x56
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::D, Register::A)), // 0x57
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::E, Register::B)), // 0x58
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::E, Register::C)), // 0x59
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::E, Register::D)), // 0x5A
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::E, Register::E)), // 0x5B
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::E, Register::H)), // 0x5C
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::E, Register::L)), // 0x5D
		Box::new(move |game_state: &mut GameState| ld_r8_hladdr(game_state, Register::E)), // 0x5E
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::E, Register::A)), // 0x5F

		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::H, Register::B)), // 0x60
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::H, Register::C)), // 0x61
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::H, Register::D)), // 0x62
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::H, Register::E)), // 0x63
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::H, Register::H)), // 0x64
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::H, Register::L)), // 0x65
		Box::new(move |game_state: &mut GameState| ld_r8_hladdr(game_state, Register::H)), // 0x66
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::H, Register::A)), // 0x67
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::L, Register::B)), // 0x68
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::L, Register::C)), // 0x69
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::L, Register::D)), // 0x6A
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::L, Register::E)), // 0x6B
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::L, Register::H)), // 0x6C
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::L, Register::L)), // 0x6D
		Box::new(move |game_state: &mut GameState| ld_r8_hladdr(game_state, Register::L)), // 0x6E
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::L, Register::A)), // 0x6F

		Box::new(move |game_state: &mut GameState| ld_hladdr_r8(game_state, Register::B)), // 0x70
		Box::new(move |game_state: &mut GameState| ld_hladdr_r8(game_state, Register::C)), // 0x71
		Box::new(move |game_state: &mut GameState| ld_hladdr_r8(game_state, Register::D)), // 0x72
		Box::new(move |game_state: &mut GameState| ld_hladdr_r8(game_state, Register::E)), // 0x73
		Box::new(move |game_state: &mut GameState| ld_hladdr_r8(game_state, Register::H)), // 0x74
		Box::new(move |game_state: &mut GameState| ld_hladdr_r8(game_state, Register::L)), // 0x75
		Box::new(move |game_state: &mut GameState| halt(game_state)), // 0x76
		Box::new(move |game_state: &mut GameState| ld_hladdr_r8(game_state, Register::A)), // 0x75
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::A, Register::B)), // 0x78
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::A, Register::C)), // 0x79
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::A, Register::D)), // 0x7A
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::A, Register::E)), // 0x7B
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::A, Register::H)), // 0x7C
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::A, Register::L)), // 0x7D
		Box::new(move |game_state: &mut GameState| ld_r8_hladdr(game_state, Register::A)), // 0x7E
		Box::new(move |game_state: &mut GameState| ld_r8_r8(game_state, Register::A, Register::A)), // 0x7F
		
		Box::new(move |game_state: &mut GameState| add_a_r8(game_state, Register::B)), // 0x80
		Box::new(move |game_state: &mut GameState| add_a_r8(game_state, Register::C)), // 0x81
		Box::new(move |game_state: &mut GameState| add_a_r8(game_state, Register::D)), // 0x82
		Box::new(move |game_state: &mut GameState| add_a_r8(game_state, Register::E)), // 0x83
		Box::new(move |game_state: &mut GameState| add_a_r8(game_state, Register::H)), // 0x84
		Box::new(move |game_state: &mut GameState| add_a_r8(game_state, Register::L)), // 0x85
		Box::new(move |game_state: &mut GameState| add_a_hladdr(game_state)), // 0x86
		Box::new(move |game_state: &mut GameState| add_a_r8(game_state, Register::A)), // 0x87
		Box::new(move |game_state: &mut GameState| adc_a_r8(game_state, Register::B)), // 0x88
		Box::new(move |game_state: &mut GameState| adc_a_r8(game_state, Register::C)), // 0x89
		Box::new(move |game_state: &mut GameState| adc_a_r8(game_state, Register::D)), // 0x8A
		Box::new(move |game_state: &mut GameState| adc_a_r8(game_state, Register::E)), // 0x8B
		Box::new(move |game_state: &mut GameState| adc_a_r8(game_state, Register::H)), // 0x8C
		Box::new(move |game_state: &mut GameState| adc_a_r8(game_state, Register::L)), // 0x8D
		Box::new(move |game_state: &mut GameState| adc_a_hladdr(game_state)), // 0x8E
		Box::new(move |game_state: &mut GameState| adc_a_r8(game_state, Register::A)), // 0x8F

		Box::new(move |game_state: &mut GameState| sub_a_r8(game_state, Register::B)), // 0x90
		Box::new(move |game_state: &mut GameState| sub_a_r8(game_state, Register::C)), // 0x91
		Box::new(move |game_state: &mut GameState| sub_a_r8(game_state, Register::D)), // 0x92
		Box::new(move |game_state: &mut GameState| sub_a_r8(game_state, Register::E)), // 0x93
		Box::new(move |game_state: &mut GameState| sub_a_r8(game_state, Register::H)), // 0x94
		Box::new(move |game_state: &mut GameState| sub_a_r8(game_state, Register::L)), // 0x95
		Box::new(move |game_state: &mut GameState| sub_a_hladdr(game_state)), // 0x96
		Box::new(move |game_state: &mut GameState| sub_a_r8(game_state, Register::A)), // 0x97
		Box::new(move |game_state: &mut GameState| sbc_a_r8(game_state, Register::B)), // 0x98
		Box::new(move |game_state: &mut GameState| sbc_a_r8(game_state, Register::C)), // 0x99
		Box::new(move |game_state: &mut GameState| sbc_a_r8(game_state, Register::D)), // 0x9A
		Box::new(move |game_state: &mut GameState| sbc_a_r8(game_state, Register::E)), // 0x9B
		Box::new(move |game_state: &mut GameState| sbc_a_r8(game_state, Register::H)), // 0x9C
		Box::new(move |game_state: &mut GameState| sbc_a_r8(game_state, Register::L)), // 0x9D
		Box::new(move |game_state: &mut GameState| sbc_a_hladdr(game_state)), // 0x9E
		Box::new(move |game_state: &mut GameState| sbc_a_r8(game_state, Register::A)), // 0x9F

		Box::new(move |game_state: &mut GameState| and_a_r8(game_state, Register::B)), // 0xA0
		Box::new(move |game_state: &mut GameState| and_a_r8(game_state, Register::C)), // 0xA1
		Box::new(move |game_state: &mut GameState| and_a_r8(game_state, Register::D)), // 0xA2
		Box::new(move |game_state: &mut GameState| and_a_r8(game_state, Register::E)), // 0xA3
		Box::new(move |game_state: &mut GameState| and_a_r8(game_state, Register::H)), // 0xA4
		Box::new(move |game_state: &mut GameState| and_a_r8(game_state, Register::L)), // 0xA5
		Box::new(move |game_state: &mut GameState| and_a_hladdr(game_state)), // 0xA6
		Box::new(move |game_state: &mut GameState| and_a_r8(game_state, Register::A)), // 0xA7
		Box::new(move |game_state: &mut GameState| xor_a_r8(game_state, Register::B)), // 0xA8
		Box::new(move |game_state: &mut GameState| xor_a_r8(game_state, Register::C)), // 0xA9
		Box::new(move |game_state: &mut GameState| xor_a_r8(game_state, Register::D)), // 0xAA
		Box::new(move |game_state: &mut GameState| xor_a_r8(game_state, Register::E)), // 0xAB
		Box::new(move |game_state: &mut GameState| xor_a_r8(game_state, Register::H)), // 0xAC
		Box::new(move |game_state: &mut GameState| xor_a_r8(game_state, Register::L)), // 0xAD
		Box::new(move |game_state: &mut GameState| xor_a_hladdr(game_state)), // 0xAE
		Box::new(move |game_state: &mut GameState| xor_a_r8(game_state, Register::A)), // 0xAF

		Box::new(move |game_state: &mut GameState| or_a_r8(game_state, Register::B)), // 0xB0
		Box::new(move |game_state: &mut GameState| or_a_r8(game_state, Register::C)), // 0xB1
		Box::new(move |game_state: &mut GameState| or_a_r8(game_state, Register::D)), // 0xB2
		Box::new(move |game_state: &mut GameState| or_a_r8(game_state, Register::E)), // 0xB3
		Box::new(move |game_state: &mut GameState| or_a_r8(game_state, Register::H)), // 0xB4
		Box::new(move |game_state: &mut GameState| or_a_r8(game_state, Register::L)), // 0xB5
		Box::new(move |game_state: &mut GameState| or_a_hladdr(game_state)), // 0xB6
		Box::new(move |game_state: &mut GameState| or_a_r8(game_state, Register::A)), // 0xB7
		Box::new(move |game_state: &mut GameState| cp_a_r8(game_state, Register::B)), // 0xB8
		Box::new(move |game_state: &mut GameState| cp_a_r8(game_state, Register::C)), // 0xB9
		Box::new(move |game_state: &mut GameState| cp_a_r8(game_state, Register::D)), // 0xBA
		Box::new(move |game_state: &mut GameState| cp_a_r8(game_state, Register::E)), // 0xBB
		Box::new(move |game_state: &mut GameState| cp_a_r8(game_state, Register::H)), // 0xBC
		Box::new(move |game_state: &mut GameState| cp_a_r8(game_state, Register::L)), // 0xBD
		Box::new(move |game_state: &mut GameState| cp_a_hladdr(game_state)), // 0xBE
		Box::new(move |game_state: &mut GameState| cp_a_r8(game_state, Register::A)), // 0xBF

		Box::new(move |game_state: &mut GameState| ret_cc(game_state, true, true, false)), // 0xC0
		Box::new(move |game_state: &mut GameState| pop_r16(game_state, Register::BC)), // 0xC1
		Box::new(move |game_state: &mut GameState| jp_cc(game_state, true, true, false)), // 0xC2
		Box::new(move |game_state: &mut GameState| jp_n16(game_state)), // 0xC3
		Box::new(move |game_state: &mut GameState| call_cc(game_state, true, true, false)), // 0xC4
		Box::new(move |game_state: &mut GameState| push_r16(game_state, Register::BC)), // 0xC5
		Box::new(move |game_state: &mut GameState| add_a_n8(game_state)), // 0xC6
		Box::new(move |game_state: &mut GameState| rst_vec(game_state, 0x00)), // 0xC7
		Box::new(move |game_state: &mut GameState| ret_cc(game_state, true, false, false)), // 0xC8
		Box::new(move |game_state: &mut GameState| ret(game_state)), // 0xC9
		Box::new(move |game_state: &mut GameState| jp_cc(game_state, true, false, false)), // 0xCA
		Box::new(move |_: &mut GameState| {}), // 0xCB PREFIX!
		Box::new(move |game_state: &mut GameState| call_cc(game_state, true, false, false)), // 0xCC
		Box::new(move |game_state: &mut GameState| call_n16(game_state)), // 0xCD
		Box::new(move |game_state: &mut GameState| adc_a_n8(game_state)), // 0xCE
		Box::new(move |game_state: &mut GameState| rst_vec(game_state, 0x08)), // 0xCF

		Box::new(move |game_state: &mut GameState| ret_cc(game_state, false, true, true)), // 0xD0
		Box::new(move |game_state: &mut GameState| pop_r16(game_state, Register::DE)), // 0xD1
		Box::new(move |game_state: &mut GameState| jp_cc(game_state, false, true, true)), // 0xD2
		Box::new(move |_: &mut GameState| {}), // 0xD3 Blank
		Box::new(move |game_state: &mut GameState| call_cc(game_state, false, true, true)), // 0xD4
		Box::new(move |game_state: &mut GameState| push_r16(game_state, Register::DE)), // 0xD5
		Box::new(move |game_state: &mut GameState| sub_a_n8(game_state)), // 0xD6
		Box::new(move |game_state: &mut GameState| rst_vec(game_state, 0x10)), // 0xD7
		Box::new(move |game_state: &mut GameState| ret_cc(game_state, false, false, true)), // 0xD8
		Box::new(move |game_state: &mut GameState| reti(game_state)), // 0xD9
		Box::new(move |game_state: &mut GameState| jp_cc(game_state, false, false, true)), // 0xDA
		Box::new(move |_: &mut GameState| {}), // 0xDB Blank
		Box::new(move |game_state: &mut GameState| call_cc(game_state, false, false, true)), // 0xDC
		Box::new(move |_: &mut GameState| {}), // 0xDD Blank
		Box::new(move |game_state: &mut GameState| sbc_a_n8(game_state)), // 0xDE
		Box::new(move |game_state: &mut GameState| rst_vec(game_state, 0x18)), // 0xDF

		Box::new(move |game_state: &mut GameState| ldh_n16addr_a(game_state)), // 0xE0
		Box::new(move |game_state: &mut GameState| pop_r16(game_state, Register::HL)), // 0xE1
		Box::new(move |game_state: &mut GameState| ldh_caddr_a(game_state)), // 0xE2
		Box::new(move |_: &mut GameState| {}), // 0xE3 Blank
		Box::new(move |_: &mut GameState| {}), // 0xE4 Blank
		Box::new(move |game_state: &mut GameState| push_r16(game_state, Register::HL)), // 0xE5
		Box::new(move |game_state: &mut GameState| and_a_n8(game_state)), // 0xE6
		Box::new(move |game_state: &mut GameState| rst_vec(game_state, 0x20)), // 0xE7
		Box::new(move |game_state: &mut GameState| add_sp_e8(game_state)), // 0xE8
		Box::new(move |game_state: &mut GameState| jp_hl(game_state)), // 0xE9
		Box::new(move |game_state: &mut GameState| ld_n16addr_a(game_state)), // 0xEA
		Box::new(move |_: &mut GameState| {}), // 0xEB Blank
		Box::new(move |_: &mut GameState| {}), // 0xEC Blank
		Box::new(move |_: &mut GameState| {}), // 0xED Blank
		Box::new(move |game_state: &mut GameState| xor_a_n8(game_state)), // 0xEE
		Box::new(move |game_state: &mut GameState| rst_vec(game_state, 0x28)), // 0xEF

		Box::new(move |game_state: &mut GameState| ldh_a_n16addr(game_state)), // 0xF0
		Box::new(move |game_state: &mut GameState| pop_r16(game_state, Register::AF)), // 0xF1
		Box::new(move |game_state: &mut GameState| ldh_a_caddr(game_state)), // 0xF2
		Box::new(move |game_state: &mut GameState| di(game_state)), // 0xF3 Blank
		Box::new(move |_: &mut GameState| {}), // 0xF4 Blank
		Box::new(move |game_state: &mut GameState| push_r16(game_state, Register::AF)), // 0xF5
		Box::new(move |game_state: &mut GameState| or_a_n8(game_state)), // 0xF6
		Box::new(move |game_state: &mut GameState| rst_vec(game_state, 0x30)), // 0xF7
		Box::new(move |game_state: &mut GameState| ld_hl_spe8(game_state)), // 0xF8
		Box::new(move |game_state: &mut GameState| ld_sp_hl(game_state)), // 0xF9
		Box::new(move |game_state: &mut GameState| ld_a_n16addr(game_state)), // 0xFA
		Box::new(move |game_state: &mut GameState| ei(game_state)), // 0xFB Blank
		Box::new(move |_: &mut GameState| {}), // 0xFC Blank
		Box::new(move |_: &mut GameState| {}), // 0xFD Blank
		Box::new(move |game_state: &mut GameState| cp_a_n8(game_state)), // 0xFE
		Box::new(move |game_state: &mut GameState| rst_vec(game_state, 0x38)), // 0xFF
	    ],

	    cb_prefix_opcodes: [
		Box::new(move |game_state: &mut GameState| rlc_r8(game_state, Register::B)), // 0x00
		Box::new(move |game_state: &mut GameState| rlc_r8(game_state, Register::C)), // 0x01
		Box::new(move |game_state: &mut GameState| rlc_r8(game_state, Register::D)), // 0x02
		Box::new(move |game_state: &mut GameState| rlc_r8(game_state, Register::E)), // 0x03
		Box::new(move |game_state: &mut GameState| rlc_r8(game_state, Register::H)), // 0x04
		Box::new(move |game_state: &mut GameState| rlc_r8(game_state, Register::L)), // 0x05
		Box::new(move |game_state: &mut GameState| rlc_hladdr(game_state)), // 0x06
		Box::new(move |game_state: &mut GameState| rrc_r8(game_state, Register::A)), // 0x07
		Box::new(move |game_state: &mut GameState| rrc_r8(game_state, Register::B)), // 0x08
		Box::new(move |game_state: &mut GameState| rrc_r8(game_state, Register::C)), // 0x09
		Box::new(move |game_state: &mut GameState| rrc_r8(game_state, Register::D)), // 0x0A
		Box::new(move |game_state: &mut GameState| rrc_r8(game_state, Register::E)), // 0x0B
		Box::new(move |game_state: &mut GameState| rrc_r8(game_state, Register::H)), // 0x0C
		Box::new(move |game_state: &mut GameState| rrc_r8(game_state, Register::L)), // 0x0D
		Box::new(move |game_state: &mut GameState| rrc_hladdr(game_state)), // 0x0E
		Box::new(move |game_state: &mut GameState| rrc_r8(game_state, Register::A)), // 0x0F

		Box::new(move |game_state: &mut GameState| rl_r8(game_state, Register::B)), // 0x10
		Box::new(move |game_state: &mut GameState| rl_r8(game_state, Register::C)), // 0x11
		Box::new(move |game_state: &mut GameState| rl_r8(game_state, Register::D)), // 0x12
		Box::new(move |game_state: &mut GameState| rl_r8(game_state, Register::E)), // 0x13
		Box::new(move |game_state: &mut GameState| rl_r8(game_state, Register::H)), // 0x14
		Box::new(move |game_state: &mut GameState| rl_r8(game_state, Register::L)), // 0x15
		Box::new(move |game_state: &mut GameState| rl_hladdr(game_state)), // 0x16
		Box::new(move |game_state: &mut GameState| rr_r8(game_state, Register::A)), // 0x17
		Box::new(move |game_state: &mut GameState| rr_r8(game_state, Register::B)), // 0x18
		Box::new(move |game_state: &mut GameState| rr_r8(game_state, Register::C)), // 0x19
		Box::new(move |game_state: &mut GameState| rr_r8(game_state, Register::D)), // 0x1A
		Box::new(move |game_state: &mut GameState| rr_r8(game_state, Register::E)), // 0x1B
		Box::new(move |game_state: &mut GameState| rr_r8(game_state, Register::H)), // 0x1C
		Box::new(move |game_state: &mut GameState| rr_r8(game_state, Register::L)), // 0x1D
		Box::new(move |game_state: &mut GameState| rr_hladdr(game_state)), // 0x1E
		Box::new(move |game_state: &mut GameState| rr_r8(game_state, Register::A)), // 0x1F

		Box::new(move |game_state: &mut GameState| sla_r8(game_state, Register::B)), // 0x20
		Box::new(move |game_state: &mut GameState| sla_r8(game_state, Register::C)), // 0x21
		Box::new(move |game_state: &mut GameState| sla_r8(game_state, Register::D)), // 0x22
		Box::new(move |game_state: &mut GameState| sla_r8(game_state, Register::E)), // 0x23
		Box::new(move |game_state: &mut GameState| sla_r8(game_state, Register::H)), // 0x24
		Box::new(move |game_state: &mut GameState| sla_r8(game_state, Register::L)), // 0x25
		Box::new(move |game_state: &mut GameState| sla_hladdr(game_state)), // 0x26
		Box::new(move |game_state: &mut GameState| sra_r8(game_state, Register::A)), // 0x27
		Box::new(move |game_state: &mut GameState| sra_r8(game_state, Register::B)), // 0x28
		Box::new(move |game_state: &mut GameState| sra_r8(game_state, Register::C)), // 0x29
		Box::new(move |game_state: &mut GameState| sra_r8(game_state, Register::D)), // 0x2A
		Box::new(move |game_state: &mut GameState| sra_r8(game_state, Register::E)), // 0x2B
		Box::new(move |game_state: &mut GameState| sra_r8(game_state, Register::H)), // 0x2C
		Box::new(move |game_state: &mut GameState| sra_r8(game_state, Register::L)), // 0x2D
		Box::new(move |game_state: &mut GameState| sra_hladdr(game_state)), // 0x2E
		Box::new(move |game_state: &mut GameState| sra_r8(game_state, Register::A)), // 0x2F

		Box::new(move |game_state: &mut GameState| swap_r8(game_state, Register::B)), // 0x30
		Box::new(move |game_state: &mut GameState| swap_r8(game_state, Register::C)), // 0x31
		Box::new(move |game_state: &mut GameState| swap_r8(game_state, Register::D)), // 0x32
		Box::new(move |game_state: &mut GameState| swap_r8(game_state, Register::E)), // 0x33
		Box::new(move |game_state: &mut GameState| swap_r8(game_state, Register::H)), // 0x34
		Box::new(move |game_state: &mut GameState| swap_r8(game_state, Register::L)), // 0x35
		Box::new(move |game_state: &mut GameState| swap_hladdr(game_state)), // 0x36
		Box::new(move |game_state: &mut GameState| swap_r8(game_state, Register::A)), // 0x37
		Box::new(move |game_state: &mut GameState| srl_r8(game_state, Register::B)), // 0x38
		Box::new(move |game_state: &mut GameState| srl_r8(game_state, Register::C)), // 0x39
		Box::new(move |game_state: &mut GameState| srl_r8(game_state, Register::D)), // 0x3A
		Box::new(move |game_state: &mut GameState| srl_r8(game_state, Register::E)), // 0x3B
		Box::new(move |game_state: &mut GameState| srl_r8(game_state, Register::H)), // 0x3C
		Box::new(move |game_state: &mut GameState| srl_r8(game_state, Register::L)), // 0x3D
		Box::new(move |game_state: &mut GameState| srl_hladdr(game_state)), // 0x3E
		Box::new(move |game_state: &mut GameState| srl_r8(game_state, Register::A)), // 0x3F

		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 0, Register::B)), // 0x40
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 0, Register::C)), // 0x41
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 0, Register::D)), // 0x42
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 0, Register::E)), // 0x43
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 0, Register::H)), // 0x44
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 0, Register::L)), // 0x45
		Box::new(move |game_state: &mut GameState| bit_u3_hladdr(game_state, 0)), // 0x46
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 0, Register::A)), // 0x47
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 1, Register::B)), // 0x48
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 1, Register::C)), // 0x49
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 1, Register::D)), // 0x4A
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 1, Register::E)), // 0x4B
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 1, Register::H)), // 0x4C
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 1, Register::L)), // 0x4D
		Box::new(move |game_state: &mut GameState| bit_u3_hladdr(game_state, 1)), // 0x4E
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 1, Register::A)), // 0x4F

		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 2, Register::B)), // 0x50
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 2, Register::C)), // 0x51
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 2, Register::D)), // 0x52
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 2, Register::E)), // 0x53
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 2, Register::H)), // 0x54
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 2, Register::L)), // 0x55
		Box::new(move |game_state: &mut GameState| bit_u3_hladdr(game_state, 2)), // 0x56
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 2, Register::A)), // 0x57
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 3, Register::B)), // 0x58
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 3, Register::C)), // 0x59
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 3, Register::D)), // 0x5A
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 3, Register::E)), // 0x5B
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 3, Register::H)), // 0x5C
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 3, Register::L)), // 0x5D
		Box::new(move |game_state: &mut GameState| bit_u3_hladdr(game_state, 3)), // 0x5E
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 3, Register::A)), // 0x5F

		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 4, Register::B)), // 0x60
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 4, Register::C)), // 0x61
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 4, Register::D)), // 0x62
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 4, Register::E)), // 0x63
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 4, Register::H)), // 0x64
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 4, Register::L)), // 0x65
		Box::new(move |game_state: &mut GameState| bit_u3_hladdr(game_state, 4)), // 0x66
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 4, Register::A)), // 0x67
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 5, Register::B)), // 0x68
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 5, Register::C)), // 0x69
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 5, Register::D)), // 0x6A
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 5, Register::E)), // 0x6B
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 5, Register::H)), // 0x6C
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 5, Register::L)), // 0x6D
		Box::new(move |game_state: &mut GameState| bit_u3_hladdr(game_state, 5)), // 0x6E
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 5, Register::A)), // 0x6F

		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 6, Register::B)), // 0x70
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 6, Register::C)), // 0x71
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 6, Register::D)), // 0x72
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 6, Register::E)), // 0x73
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 6, Register::H)), // 0x74
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 6, Register::L)), // 0x75
		Box::new(move |game_state: &mut GameState| bit_u3_hladdr(game_state, 6)), // 0x76
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 6, Register::A)), // 0x77
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 7, Register::B)), // 0x78
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 7, Register::C)), // 0x79
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 7, Register::D)), // 0x7A
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 7, Register::E)), // 0x7B
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 7, Register::H)), // 0x7C
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 7, Register::L)), // 0x7D
		Box::new(move |game_state: &mut GameState| bit_u3_hladdr(game_state, 7)), // 0x7E
		Box::new(move |game_state: &mut GameState| bit_u3_r8(game_state, 7, Register::A)), // 0x7F

		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 0, Register::B)), // 0x80
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 0, Register::C)), // 0x81
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 0, Register::D)), // 0x82
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 0, Register::E)), // 0x83
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 0, Register::H)), // 0x84
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 0, Register::L)), // 0x85
		Box::new(move |game_state: &mut GameState| res_u3_hladdr(game_state, 0)), // 0x86
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 0, Register::A)), // 0x87
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 1, Register::B)), // 0x88
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 1, Register::C)), // 0x89
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 1, Register::D)), // 0x8A
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 1, Register::E)), // 0x8B
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 1, Register::H)), // 0x8C
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 1, Register::L)), // 0x8D
		Box::new(move |game_state: &mut GameState| res_u3_hladdr(game_state, 1)), // 0x8E
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 1, Register::A)), // 0x8F

		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 2, Register::B)), // 0x90
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 2, Register::C)), // 0x91
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 2, Register::D)), // 0x92
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 2, Register::E)), // 0x93
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 2, Register::H)), // 0x94
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 2, Register::L)), // 0x95
		Box::new(move |game_state: &mut GameState| res_u3_hladdr(game_state, 2)), // 0x96
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 2, Register::A)), // 0x97
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 3, Register::B)), // 0x98
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 3, Register::C)), // 0x99
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 3, Register::D)), // 0x9A
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 3, Register::E)), // 0x9B
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 3, Register::H)), // 0x9C
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 3, Register::L)), // 0x9D
		Box::new(move |game_state: &mut GameState| res_u3_hladdr(game_state, 3)), // 0x9E
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 3, Register::A)), // 0x9F

		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 4, Register::B)), // 0xA0
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 4, Register::C)), // 0xA1
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 4, Register::D)), // 0xA2
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 4, Register::E)), // 0xA3
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 4, Register::H)), // 0xA4
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 4, Register::L)), // 0xA5
		Box::new(move |game_state: &mut GameState| res_u3_hladdr(game_state, 4)), // 0xA6
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 4, Register::A)), // 0xA7
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 5, Register::B)), // 0xA8
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 5, Register::C)), // 0xA9
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 5, Register::D)), // 0xAA
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 5, Register::E)), // 0xAB
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 5, Register::H)), // 0xAC
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 5, Register::L)), // 0xAD
		Box::new(move |game_state: &mut GameState| res_u3_hladdr(game_state, 5)), // 0xAE
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 5, Register::A)), // 0xAF

		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 6, Register::B)), // 0xB0
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 6, Register::C)), // 0xB1
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 6, Register::D)), // 0xB2
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 6, Register::E)), // 0xB3
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 6, Register::H)), // 0xB4
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 6, Register::L)), // 0xB5
		Box::new(move |game_state: &mut GameState| res_u3_hladdr(game_state, 6)), // 0xB6
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 6, Register::A)), // 0xB7
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 7, Register::B)), // 0xB8
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 7, Register::C)), // 0xB9
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 7, Register::D)), // 0xBA
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 7, Register::E)), // 0xBB
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 7, Register::H)), // 0xBC
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 7, Register::L)), // 0xBD
		Box::new(move |game_state: &mut GameState| res_u3_hladdr(game_state, 7)), // 0xBE
		Box::new(move |game_state: &mut GameState| res_u3_r8(game_state, 7, Register::A)), // 0xBF

		// TODO Below
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 0, Register::B)), // 0xC0
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 0, Register::C)), // 0xC1
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 0, Register::D)), // 0xC2
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 0, Register::E)), // 0xC3
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 0, Register::H)), // 0xC4
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 0, Register::L)), // 0xC5
		Box::new(move |game_state: &mut GameState| set_u3_hladdr(game_state, 0)), // 0xC6
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 0, Register::A)), // 0xC7
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 1, Register::B)), // 0xC8
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 1, Register::C)), // 0xC9
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 1, Register::D)), // 0xCA
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 1, Register::E)), // 0xCB
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 1, Register::H)), // 0xCC
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 1, Register::L)), // 0xCD
		Box::new(move |game_state: &mut GameState| set_u3_hladdr(game_state, 1)), // 0xCE
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 1, Register::A)), // 0xCF

		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 2, Register::B)), // 0xD0
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 2, Register::C)), // 0xD1
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 2, Register::D)), // 0xD2
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 2, Register::E)), // 0xD3
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 2, Register::H)), // 0xD4
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 2, Register::L)), // 0xD5
		Box::new(move |game_state: &mut GameState| set_u3_hladdr(game_state, 2)), // 0xD6
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 2, Register::A)), // 0xD7
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 3, Register::B)), // 0xD8
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 3, Register::C)), // 0xD9
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 3, Register::D)), // 0xDA
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 3, Register::E)), // 0xDB
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 3, Register::H)), // 0xDC
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 3, Register::L)), // 0xDD
		Box::new(move |game_state: &mut GameState| set_u3_hladdr(game_state, 3)), // 0xDE
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 3, Register::A)), // 0xDF

		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 4, Register::B)), // 0xE0
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 4, Register::C)), // 0xE1
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 4, Register::D)), // 0xE2
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 4, Register::E)), // 0xE3
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 4, Register::H)), // 0xE4
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 4, Register::L)), // 0xE5
		Box::new(move |game_state: &mut GameState| set_u3_hladdr(game_state, 4)), // 0xE6
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 4, Register::A)), // 0xE7
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 5, Register::B)), // 0xE8
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 5, Register::C)), // 0xE9
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 5, Register::D)), // 0xEA
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 5, Register::E)), // 0xEB
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 5, Register::H)), // 0xEC
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 5, Register::L)), // 0xED
		Box::new(move |game_state: &mut GameState| set_u3_hladdr(game_state, 5)), // 0xEE
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 5, Register::A)), // 0xEF

		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 6, Register::B)), // 0xF0
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 6, Register::C)), // 0xF1
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 6, Register::D)), // 0xF2
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 6, Register::E)), // 0xF3
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 6, Register::H)), // 0xF4
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 6, Register::L)), // 0xF5
		Box::new(move |game_state: &mut GameState| set_u3_hladdr(game_state, 6)), // 0xF6
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 6, Register::A)), // 0xF7
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 7, Register::B)), // 0xF8
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 7, Register::C)), // 0xF9
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 7, Register::D)), // 0xFA
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 7, Register::E)), // 0xFB
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 7, Register::H)), // 0xFC
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 7, Register::L)), // 0xFD
		Box::new(move |game_state: &mut GameState| set_u3_hladdr(game_state, 7)), // 0xFE
		Box::new(move |game_state: &mut GameState| set_u3_r8(game_state, 7, Register::A)), // 0xFF
	    ]
	}
    }

}
