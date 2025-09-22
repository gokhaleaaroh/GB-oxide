use crate::state::{GameState, Register};
use crate::instructions::*;

type InstructionWrapper = Box<dyn Fn(&mut GameState)>;

struct Decoder {
    non_prefix_opcodes: [InstructionWrapper; 192],
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

		Box::new(move |game_state: &mut GameState| jr_cc(game_state, true, true, false, false)), // 0x20
		Box::new(move |game_state: &mut GameState| ld_r16_n16(game_state, Register::HL)), // 0x21
		Box::new(move |game_state: &mut GameState| ld_hliaddr_a(game_state)), // 0x22    
		Box::new(move |game_state: &mut GameState| inc_r16(game_state, Register::HL)), // 0x23
		Box::new(move |game_state: &mut GameState| inc_r8(game_state, Register::H)), // 0x24
		Box::new(move |game_state: &mut GameState| dec_r8(game_state, Register::H)), // 0x25
		Box::new(move |game_state: &mut GameState| ld_r8_n8(game_state, Register::H)), // 0x26
		Box::new(move |game_state: &mut GameState| daa(game_state)), // 0x27
		Box::new(move |game_state: &mut GameState| jr_cc(game_state, true, false, false, false)), // 0x28
		Box::new(move |game_state: &mut GameState| add_hl_r16(game_state, Register::HL)), // 0x29
		Box::new(move |game_state: &mut GameState| ld_a_hli(game_state)), // 0x2A
		Box::new(move |game_state: &mut GameState| dec_r16(game_state, Register::HL)), // 0x2B
		Box::new(move |game_state: &mut GameState| inc_r8(game_state, Register::L)), // 0x2C
		Box::new(move |game_state: &mut GameState| dec_r8(game_state, Register::L)), // 0x2D
		Box::new(move |game_state: &mut GameState| ld_r8_n8(game_state, Register::L)), // 0x2E
		Box::new(move |game_state: &mut GameState| cpl(game_state)), // 0x2F

		Box::new(move |game_state: &mut GameState| jr_cc(game_state, false, true, false, true)), // 0x30
		Box::new(move |game_state: &mut GameState| ld_sp_n16addr(game_state)), // 0x31
		Box::new(move |game_state: &mut GameState| ld_hldaddr_a(game_state)), // 0x32    
		Box::new(move |game_state: &mut GameState| inc_sp(game_state)), // 0x33
		Box::new(move |game_state: &mut GameState| inc_hladdr(game_state)), // 0x34
		Box::new(move |game_state: &mut GameState| dec_hladdr(game_state)), // 0x35
		Box::new(move |game_state: &mut GameState| ld_hladdr_n8(game_state)), // 0x36
		Box::new(move |game_state: &mut GameState| scf(game_state)), // 0x37
		Box::new(move |game_state: &mut GameState| jr_cc(game_state, false, false, false, true)), // 0x38
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

		Box::new(move |game_state: &mut GameState| or_a_r8(game_state, Register::B)), // 0xA0
		Box::new(move |game_state: &mut GameState| or_a_r8(game_state, Register::C)), // 0xA1
		Box::new(move |game_state: &mut GameState| or_a_r8(game_state, Register::D)), // 0xA2
		Box::new(move |game_state: &mut GameState| or_a_r8(game_state, Register::E)), // 0xA3
		Box::new(move |game_state: &mut GameState| or_a_r8(game_state, Register::H)), // 0xA4
		Box::new(move |game_state: &mut GameState| or_a_r8(game_state, Register::L)), // 0xA5
		Box::new(move |game_state: &mut GameState| or_a_hladdr(game_state)), // 0xA6
		Box::new(move |game_state: &mut GameState| or_a_r8(game_state, Register::A)), // 0xA7
		Box::new(move |game_state: &mut GameState| cp_a_r8(game_state, Register::B)), // 0xA8
		Box::new(move |game_state: &mut GameState| cp_a_r8(game_state, Register::C)), // 0xA9
		Box::new(move |game_state: &mut GameState| cp_a_r8(game_state, Register::D)), // 0xAA
		Box::new(move |game_state: &mut GameState| cp_a_r8(game_state, Register::E)), // 0xAB
		Box::new(move |game_state: &mut GameState| cp_a_r8(game_state, Register::H)), // 0xAC
		Box::new(move |game_state: &mut GameState| cp_a_r8(game_state, Register::L)), // 0xAD
		Box::new(move |game_state: &mut GameState| cp_a_hladdr(game_state)), // 0xAE
		Box::new(move |game_state: &mut GameState| cp_a_r8(game_state, Register::A)), // 0xAF

		// TODO Below
	    ]
	}
    }

}
