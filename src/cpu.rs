use crate::instructions::*;
use crate::state::{GameState, Register, CC};
use std::collections::HashSet;

type InstructionWrapper = fn(&mut GameState) -> u8;

pub struct CPU {
    non_prefix_opcodes: [InstructionWrapper; 256],
    cb_prefix_opcodes: [InstructionWrapper; 256],
    two_byte_ins: HashSet<u8>,
    three_byte_ins: HashSet<u8>,
}

impl CPU {
    pub fn initialize() -> Self {
        let mut iibi: HashSet<u8> = HashSet::new();
        iibi.insert(0x06);
        iibi.insert(0x0E);
        iibi.insert(0x16);
        iibi.insert(0x18);
        iibi.insert(0x1E);
        iibi.insert(0x20);
        iibi.insert(0x26);
        iibi.insert(0x28);
        iibi.insert(0x2E);
        iibi.insert(0x30);
        iibi.insert(0x36);
        iibi.insert(0x38);
        iibi.insert(0x3E);
        iibi.insert(0xC6);
        iibi.insert(0xCE);
        iibi.insert(0xD6);
        iibi.insert(0xDE);
        iibi.insert(0xE0);
        iibi.insert(0xE6);
        iibi.insert(0xE8);
        iibi.insert(0xEE);
        iibi.insert(0xF0);
        iibi.insert(0xF6);
        iibi.insert(0xF8);
        iibi.insert(0xFE);
        iibi.insert(0xFE);

        let mut iiibi: HashSet<u8> = HashSet::new();
        iiibi.insert(0x01);
        iiibi.insert(0x08);
        iiibi.insert(0x11);
        iiibi.insert(0x21);
        iiibi.insert(0x31);
        iiibi.insert(0xC2);
        iiibi.insert(0xC3);
        iiibi.insert(0xC4);
        iiibi.insert(0xCA);
        iiibi.insert(0xCC);
        iiibi.insert(0xCD);
        iiibi.insert(0xD2);
        iiibi.insert(0xD4);
        iiibi.insert(0xDA);
        iiibi.insert(0xDC);
        iiibi.insert(0xEA);
        iiibi.insert(0xFA);

        Self {
            non_prefix_opcodes: [
                |_: &mut GameState| 1,                                     // 0x00
                |s: &mut GameState| ld_r16_n16(s, Register::BC),           // 0x01
                |s: &mut GameState| ld_r16addr_a(s, Register::BC),         // 0x02
                |s: &mut GameState| inc_r16(s, Register::BC),              // 0x03
                |s: &mut GameState| inc_r8(s, Register::B),                // 0x04
                |s: &mut GameState| dec_r8(s, Register::B),                // 0x05
                |s: &mut GameState| ld_r8_n8(s, Register::B),              // 0x06
                |s: &mut GameState| rlca(s),                               // 0x07
                |s: &mut GameState| ld_n16addr_sp(s),                      // 0x08
                |s: &mut GameState| add_hl_r16(s, Register::BC),           // 0x09
                |s: &mut GameState| ld_a_r16addr(s, Register::BC),         // 0x0A
                |s: &mut GameState| dec_r16(s, Register::BC),              // 0x0B
                |s: &mut GameState| inc_r8(s, Register::C),                // 0x0C
                |s: &mut GameState| dec_r8(s, Register::C),                // 0x0D
                |s: &mut GameState| ld_r8_n8(s, Register::C),              // 0x0E
                |s: &mut GameState| rrca(s),                               // 0x0F
                |s: &mut GameState| stop(s),                               // 0x10
                |s: &mut GameState| ld_r16_n16(s, Register::DE),           // 0x11
                |s: &mut GameState| ld_r16addr_a(s, Register::DE),         // 0x12
                |s: &mut GameState| inc_r16(s, Register::DE),              // 0x13
                |s: &mut GameState| inc_r8(s, Register::D),                // 0x14
                |s: &mut GameState| dec_r8(s, Register::D),                // 0x15
                |s: &mut GameState| ld_r8_n8(s, Register::D),              // 0x16
                |s: &mut GameState| rla(s),                                // 0x17
                |s: &mut GameState| jr_e8(s),                             // 0x18
                |s: &mut GameState| add_hl_r16(s, Register::DE),           // 0x19
                |s: &mut GameState| ld_a_r16addr(s, Register::DE),         // 0x1A
                |s: &mut GameState| dec_r16(s, Register::DE),              // 0x1B
                |s: &mut GameState| inc_r8(s, Register::E),                // 0x1C
                |s: &mut GameState| dec_r8(s, Register::E),                // 0x1D
                |s: &mut GameState| ld_r8_n8(s, Register::E),              // 0x1E
                |s: &mut GameState| rra(s),                                // 0x1F
                |s: &mut GameState| jr_cc(s, CC::NZ),           // 0x20
                |s: &mut GameState| ld_r16_n16(s, Register::HL),           // 0x21
                |s: &mut GameState| ld_hliaddr_a(s),                       // 0x22
                |s: &mut GameState| inc_r16(s, Register::HL),              // 0x23
                |s: &mut GameState| inc_r8(s, Register::H),                // 0x24
                |s: &mut GameState| dec_r8(s, Register::H),                // 0x25
                |s: &mut GameState| ld_r8_n8(s, Register::H),              // 0x26
                |s: &mut GameState| daa(s),                                // 0x27
                |s: &mut GameState| jr_cc(s, CC::Z),          // 0x28
                |s: &mut GameState| add_hl_r16(s, Register::HL),           // 0x29
                |s: &mut GameState| ld_a_hli(s),                           // 0x2A
                |s: &mut GameState| dec_r16(s, Register::HL),              // 0x2B
                |s: &mut GameState| inc_r8(s, Register::L),                // 0x2C
                |s: &mut GameState| dec_r8(s, Register::L),                // 0x2D
                |s: &mut GameState| ld_r8_n8(s, Register::L),              // 0x2E
                |s: &mut GameState| cpl(s),                                // 0x2F
                |s: &mut GameState| jr_cc(s, CC::NC),           // 0x30
                |s: &mut GameState| ld_sp_n16addr(s),                      // 0x31
                |s: &mut GameState| ld_hldaddr_a(s),                       // 0x32
                |s: &mut GameState| inc_sp(s),                             // 0x33
                |s: &mut GameState| inc_hladdr(s),                         // 0x34
                |s: &mut GameState| dec_hladdr(s),                         // 0x35
                |s: &mut GameState| ld_hladdr_n8(s),                       // 0x36
                |s: &mut GameState| scf(s),                                // 0x37
                |s: &mut GameState| jr_cc(s, CC::C),          // 0x38
                |s: &mut GameState| add_hl_sp(s),                          // 0x39
                |s: &mut GameState| ld_a_hld(s),                           // 0x3A
                |s: &mut GameState| dec_sp(s),                             // 0x3B
                |s: &mut GameState| inc_r8(s, Register::A),                // 0x3C
                |s: &mut GameState| dec_r8(s, Register::A),                // 0x3D
                |s: &mut GameState| ld_r8_n8(s, Register::A),              // 0x3E
                |s: &mut GameState| ccf(s),                                // 0x3F
                |s: &mut GameState| ld_r8_r8(s, Register::B, Register::B), // 0x40
                |s: &mut GameState| ld_r8_r8(s, Register::B, Register::C), // 0x41
                |s: &mut GameState| ld_r8_r8(s, Register::B, Register::D), // 0x42
                |s: &mut GameState| ld_r8_r8(s, Register::B, Register::E), // 0x43
                |s: &mut GameState| ld_r8_r8(s, Register::B, Register::H), // 0x44
                |s: &mut GameState| ld_r8_r8(s, Register::B, Register::L), // 0x45
                |s: &mut GameState| ld_r8_hladdr(s, Register::B),          // 0x46
                |s: &mut GameState| ld_r8_r8(s, Register::B, Register::A), // 0x47
                |s: &mut GameState| ld_r8_r8(s, Register::C, Register::B), // 0x48
                |s: &mut GameState| ld_r8_r8(s, Register::C, Register::C), // 0x49
                |s: &mut GameState| ld_r8_r8(s, Register::C, Register::D), // 0x4A
                |s: &mut GameState| ld_r8_r8(s, Register::C, Register::E), // 0x4B
                |s: &mut GameState| ld_r8_r8(s, Register::C, Register::H), // 0x4C
                |s: &mut GameState| ld_r8_r8(s, Register::C, Register::L), // 0x4D
                |s: &mut GameState| ld_r8_hladdr(s, Register::C),          // 0x4E
                |s: &mut GameState| ld_r8_r8(s, Register::C, Register::A), // 0x4F
                |s: &mut GameState| ld_r8_r8(s, Register::D, Register::B), // 0x50
                |s: &mut GameState| ld_r8_r8(s, Register::D, Register::C), // 0x51
                |s: &mut GameState| ld_r8_r8(s, Register::D, Register::D), // 0x52
                |s: &mut GameState| ld_r8_r8(s, Register::D, Register::E), // 0x53
                |s: &mut GameState| ld_r8_r8(s, Register::D, Register::H), // 0x54
                |s: &mut GameState| ld_r8_r8(s, Register::D, Register::L), // 0x55
                |s: &mut GameState| ld_r8_hladdr(s, Register::D),          // 0x56
                |s: &mut GameState| ld_r8_r8(s, Register::D, Register::A), // 0x57
                |s: &mut GameState| ld_r8_r8(s, Register::E, Register::B), // 0x58
                |s: &mut GameState| ld_r8_r8(s, Register::E, Register::C), // 0x59
                |s: &mut GameState| ld_r8_r8(s, Register::E, Register::D), // 0x5A
                |s: &mut GameState| ld_r8_r8(s, Register::E, Register::E), // 0x5B
                |s: &mut GameState| ld_r8_r8(s, Register::E, Register::H), // 0x5C
                |s: &mut GameState| ld_r8_r8(s, Register::E, Register::L), // 0x5D
                |s: &mut GameState| ld_r8_hladdr(s, Register::E),          // 0x5E
                |s: &mut GameState| ld_r8_r8(s, Register::E, Register::A), // 0x5F
                |s: &mut GameState| ld_r8_r8(s, Register::H, Register::B), // 0x60
                |s: &mut GameState| ld_r8_r8(s, Register::H, Register::C), // 0x61
                |s: &mut GameState| ld_r8_r8(s, Register::H, Register::D), // 0x62
                |s: &mut GameState| ld_r8_r8(s, Register::H, Register::E), // 0x63
                |s: &mut GameState| ld_r8_r8(s, Register::H, Register::H), // 0x64
                |s: &mut GameState| ld_r8_r8(s, Register::H, Register::L), // 0x65
                |s: &mut GameState| ld_r8_hladdr(s, Register::H),          // 0x66
                |s: &mut GameState| ld_r8_r8(s, Register::H, Register::A), // 0x67
                |s: &mut GameState| ld_r8_r8(s, Register::L, Register::B), // 0x68
                |s: &mut GameState| ld_r8_r8(s, Register::L, Register::C), // 0x69
                |s: &mut GameState| ld_r8_r8(s, Register::L, Register::D), // 0x6A
                |s: &mut GameState| ld_r8_r8(s, Register::L, Register::E), // 0x6B
                |s: &mut GameState| ld_r8_r8(s, Register::L, Register::H), // 0x6C
                |s: &mut GameState| ld_r8_r8(s, Register::L, Register::L), // 0x6D
                |s: &mut GameState| ld_r8_hladdr(s, Register::L),          // 0x6E
                |s: &mut GameState| ld_r8_r8(s, Register::L, Register::A), // 0x6F
                |s: &mut GameState| ld_hladdr_r8(s, Register::B),          // 0x70
                |s: &mut GameState| ld_hladdr_r8(s, Register::C),          // 0x71
                |s: &mut GameState| ld_hladdr_r8(s, Register::D),          // 0x72
                |s: &mut GameState| ld_hladdr_r8(s, Register::E),          // 0x73
                |s: &mut GameState| ld_hladdr_r8(s, Register::H),          // 0x74
                |s: &mut GameState| ld_hladdr_r8(s, Register::L),          // 0x75
                |s: &mut GameState| halt(s),                               // 0x76
                |s: &mut GameState| ld_hladdr_r8(s, Register::A),          // 0x75
                |s: &mut GameState| ld_r8_r8(s, Register::A, Register::B), // 0x78
                |s: &mut GameState| ld_r8_r8(s, Register::A, Register::C), // 0x79
                |s: &mut GameState| ld_r8_r8(s, Register::A, Register::D), // 0x7A
                |s: &mut GameState| ld_r8_r8(s, Register::A, Register::E), // 0x7B
                |s: &mut GameState| ld_r8_r8(s, Register::A, Register::H), // 0x7C
                |s: &mut GameState| ld_r8_r8(s, Register::A, Register::L), // 0x7D
                |s: &mut GameState| ld_r8_hladdr(s, Register::A),          // 0x7E
                |s: &mut GameState| ld_r8_r8(s, Register::A, Register::A), // 0x7F
                |s: &mut GameState| add_a_r8(s, Register::B),              // 0x80
                |s: &mut GameState| add_a_r8(s, Register::C),              // 0x81
                |s: &mut GameState| add_a_r8(s, Register::D),              // 0x82
                |s: &mut GameState| add_a_r8(s, Register::E),              // 0x83
                |s: &mut GameState| add_a_r8(s, Register::H),              // 0x84
                |s: &mut GameState| add_a_r8(s, Register::L),              // 0x85
                |s: &mut GameState| add_a_hladdr(s),                       // 0x86
                |s: &mut GameState| add_a_r8(s, Register::A),              // 0x87
                |s: &mut GameState| adc_a_r8(s, Register::B),              // 0x88
                |s: &mut GameState| adc_a_r8(s, Register::C),              // 0x89
                |s: &mut GameState| adc_a_r8(s, Register::D),              // 0x8A
                |s: &mut GameState| adc_a_r8(s, Register::E),              // 0x8B
                |s: &mut GameState| adc_a_r8(s, Register::H),              // 0x8C
                |s: &mut GameState| adc_a_r8(s, Register::L),              // 0x8D
                |s: &mut GameState| adc_a_hladdr(s),                       // 0x8E
                |s: &mut GameState| adc_a_r8(s, Register::A),              // 0x8F
                |s: &mut GameState| sub_a_r8(s, Register::B),              // 0x90
                |s: &mut GameState| sub_a_r8(s, Register::C),              // 0x91
                |s: &mut GameState| sub_a_r8(s, Register::D),              // 0x92
                |s: &mut GameState| sub_a_r8(s, Register::E),              // 0x93
                |s: &mut GameState| sub_a_r8(s, Register::H),              // 0x94
                |s: &mut GameState| sub_a_r8(s, Register::L),              // 0x95
                |s: &mut GameState| sub_a_hladdr(s),                       // 0x96
                |s: &mut GameState| sub_a_r8(s, Register::A),              // 0x97
                |s: &mut GameState| sbc_a_r8(s, Register::B),              // 0x98
                |s: &mut GameState| sbc_a_r8(s, Register::C),              // 0x99
                |s: &mut GameState| sbc_a_r8(s, Register::D),              // 0x9A
                |s: &mut GameState| sbc_a_r8(s, Register::E),              // 0x9B
                |s: &mut GameState| sbc_a_r8(s, Register::H),              // 0x9C
                |s: &mut GameState| sbc_a_r8(s, Register::L),              // 0x9D
                |s: &mut GameState| sbc_a_hladdr(s),                       // 0x9E
                |s: &mut GameState| sbc_a_r8(s, Register::A),              // 0x9F
                |s: &mut GameState| and_a_r8(s, Register::B),              // 0xA0
                |s: &mut GameState| and_a_r8(s, Register::C),              // 0xA1
                |s: &mut GameState| and_a_r8(s, Register::D),              // 0xA2
                |s: &mut GameState| and_a_r8(s, Register::E),              // 0xA3
                |s: &mut GameState| and_a_r8(s, Register::H),              // 0xA4
                |s: &mut GameState| and_a_r8(s, Register::L),              // 0xA5
                |s: &mut GameState| and_a_hladdr(s),                       // 0xA6
                |s: &mut GameState| and_a_r8(s, Register::A),              // 0xA7
                |s: &mut GameState| xor_a_r8(s, Register::B),              // 0xA8
                |s: &mut GameState| xor_a_r8(s, Register::C),              // 0xA9
                |s: &mut GameState| xor_a_r8(s, Register::D),              // 0xAA
                |s: &mut GameState| xor_a_r8(s, Register::E),              // 0xAB
                |s: &mut GameState| xor_a_r8(s, Register::H),              // 0xAC
                |s: &mut GameState| xor_a_r8(s, Register::L),              // 0xAD
                |s: &mut GameState| xor_a_hladdr(s),                       // 0xAE
                |s: &mut GameState| xor_a_r8(s, Register::A),              // 0xAF
                |s: &mut GameState| or_a_r8(s, Register::B),               // 0xB0
                |s: &mut GameState| or_a_r8(s, Register::C),               // 0xB1
                |s: &mut GameState| or_a_r8(s, Register::D),               // 0xB2
                |s: &mut GameState| or_a_r8(s, Register::E),               // 0xB3
                |s: &mut GameState| or_a_r8(s, Register::H),               // 0xB4
                |s: &mut GameState| or_a_r8(s, Register::L),               // 0xB5
                |s: &mut GameState| or_a_hladdr(s),                        // 0xB6
                |s: &mut GameState| or_a_r8(s, Register::A),               // 0xB7
                |s: &mut GameState| cp_a_r8(s, Register::B),               // 0xB8
                |s: &mut GameState| cp_a_r8(s, Register::C),               // 0xB9
                |s: &mut GameState| cp_a_r8(s, Register::D),               // 0xBA
                |s: &mut GameState| cp_a_r8(s, Register::E),               // 0xBB
                |s: &mut GameState| cp_a_r8(s, Register::H),               // 0xBC
                |s: &mut GameState| cp_a_r8(s, Register::L),               // 0xBD
                |s: &mut GameState| cp_a_hladdr(s),                        // 0xBE
                |s: &mut GameState| cp_a_r8(s, Register::A),               // 0xBF
                |s: &mut GameState| ret_cc(s, CC::NZ),          // 0xC0
                |s: &mut GameState| pop_r16(s, Register::BC),              // 0xC1
                |s: &mut GameState| jp_cc(s, CC::NZ),           // 0xC2
                |s: &mut GameState| jp_n16(s),                             // 0xC3
                |s: &mut GameState| call_cc(s, CC::NZ),         // 0xC4
                |s: &mut GameState| push_r16(s, Register::BC),             // 0xC5
                |s: &mut GameState| add_a_n8(s),                           // 0xC6
                |s: &mut GameState| rst_vec(s, 0x00),                      // 0xC7
                |s: &mut GameState| ret_cc(s, CC::Z),         // 0xC8
                |s: &mut GameState| ret(s),                                // 0xC9
                |s: &mut GameState| jp_cc(s, CC::Z),          // 0xCA
                |_: &mut GameState| 1,                                     // 0xCB PREFIX!
                |s: &mut GameState| call_cc(s, CC::Z),        // 0xCC
                |s: &mut GameState| call_n16(s),                           // 0xCD
                |s: &mut GameState| adc_a_n8(s),                           // 0xCE
                |s: &mut GameState| rst_vec(s, 0x08),                      // 0xCF
                |s: &mut GameState| ret_cc(s, CC::NC),          // 0xD0
                |s: &mut GameState| pop_r16(s, Register::DE),              // 0xD1
                |s: &mut GameState| jp_cc(s, CC::NC),           // 0xD2
                |_: &mut GameState| 1,                                     // 0xD3 Blank
                |s: &mut GameState| call_cc(s, CC::NC),         // 0xD4
                |s: &mut GameState| push_r16(s, Register::DE),             // 0xD5
                |s: &mut GameState| sub_a_n8(s),                           // 0xD6
                |s: &mut GameState| rst_vec(s, 0x10),                      // 0xD7
                |s: &mut GameState| ret_cc(s, CC::C),         // 0xD8
                |s: &mut GameState| reti(s),                               // 0xD9
                |s: &mut GameState| jp_cc(s, CC::C),          // 0xDA
                |_: &mut GameState| 1,                                     // 0xDB Blank
                |s: &mut GameState| call_cc(s, CC::C),        // 0xDC
                |_: &mut GameState| 1,                                     // 0xDD Blank
                |s: &mut GameState| sbc_a_n8(s),                           // 0xDE
                |s: &mut GameState| rst_vec(s, 0x18),                      // 0xDF
                |s: &mut GameState| ldh_n8addr_a(s),                       // 0xE0
                |s: &mut GameState| pop_r16(s, Register::HL),              // 0xE1
                |s: &mut GameState| ldh_caddr_a(s),                        // 0xE2
                |_: &mut GameState| 1,                                     // 0xE3 Blank
                |_: &mut GameState| 1,                                     // 0xE4 Blank
                |s: &mut GameState| push_r16(s, Register::HL),             // 0xE5
                |s: &mut GameState| and_a_n8(s),                           // 0xE6
                |s: &mut GameState| rst_vec(s, 0x20),                      // 0xE7
                |s: &mut GameState| add_sp_e8(s),                          // 0xE8
                |s: &mut GameState| jp_hl(s),                              // 0xE9
                |s: &mut GameState| ld_n16addr_a(s),                       // 0xEA
                |_: &mut GameState| 1,                                     // 0xEB Blank
                |_: &mut GameState| 1,                                     // 0xEC Blank
                |_: &mut GameState| 1,                                     // 0xED Blank
                |s: &mut GameState| xor_a_n8(s),                           // 0xEE
                |s: &mut GameState| rst_vec(s, 0x28),                      // 0xEF
                |s: &mut GameState| ldh_a_n8addr(s),                       // 0xF0
                |s: &mut GameState| pop_r16(s, Register::AF),              // 0xF1
                |s: &mut GameState| ldh_a_caddr(s),                        // 0xF2
                |s: &mut GameState| di(s),                                 // 0xF3 Blank
                |_: &mut GameState| 1,                                     // 0xF4 Blank
                |s: &mut GameState| push_r16(s, Register::AF),             // 0xF5
                |s: &mut GameState| or_a_n8(s),                            // 0xF6
                |s: &mut GameState| rst_vec(s, 0x30),                      // 0xF7
                |s: &mut GameState| ld_hl_spe8(s),                         // 0xF8
                |s: &mut GameState| ld_sp_hl(s),                           // 0xF9
                |s: &mut GameState| ld_a_n16addr(s),                       // 0xFA
                |s: &mut GameState| ei(s),                                 // 0xFB Blank
                |_: &mut GameState| 1,                                     // 0xFC Blank
                |_: &mut GameState| 1,                                     // 0xFD Blank
                |s: &mut GameState| cp_a_n8(s),                            // 0xFE
                |s: &mut GameState| rst_vec(s, 0x38),                      // 0xFF
            ],

            cb_prefix_opcodes: [
                |s: &mut GameState| rlc_r8(s, Register::B),       // 0x00
                |s: &mut GameState| rlc_r8(s, Register::C),       // 0x01
                |s: &mut GameState| rlc_r8(s, Register::D),       // 0x02
                |s: &mut GameState| rlc_r8(s, Register::E),       // 0x03
                |s: &mut GameState| rlc_r8(s, Register::H),       // 0x04
                |s: &mut GameState| rlc_r8(s, Register::L),       // 0x05
                |s: &mut GameState| rlc_hladdr(s),                // 0x06
                |s: &mut GameState| rlc_r8(s, Register::A),       // 0x07
                |s: &mut GameState| rrc_r8(s, Register::B),       // 0x08
                |s: &mut GameState| rrc_r8(s, Register::C),       // 0x09
                |s: &mut GameState| rrc_r8(s, Register::D),       // 0x0A
                |s: &mut GameState| rrc_r8(s, Register::E),       // 0x0B
                |s: &mut GameState| rrc_r8(s, Register::H),       // 0x0C
                |s: &mut GameState| rrc_r8(s, Register::L),       // 0x0D
                |s: &mut GameState| rrc_hladdr(s),                // 0x0E
                |s: &mut GameState| rrc_r8(s, Register::A),       // 0x0F
                |s: &mut GameState| rl_r8(s, Register::B),        // 0x10
                |s: &mut GameState| rl_r8(s, Register::C),        // 0x11
                |s: &mut GameState| rl_r8(s, Register::D),        // 0x12
                |s: &mut GameState| rl_r8(s, Register::E),        // 0x13
                |s: &mut GameState| rl_r8(s, Register::H),        // 0x14
                |s: &mut GameState| rl_r8(s, Register::L),        // 0x15
                |s: &mut GameState| rl_hladdr(s),                 // 0x16
                |s: &mut GameState| rl_r8(s, Register::A),        // 0x17
                |s: &mut GameState| rr_r8(s, Register::B),        // 0x18
                |s: &mut GameState| rr_r8(s, Register::C),        // 0x19
                |s: &mut GameState| rr_r8(s, Register::D),        // 0x1A
                |s: &mut GameState| rr_r8(s, Register::E),        // 0x1B
                |s: &mut GameState| rr_r8(s, Register::H),        // 0x1C
                |s: &mut GameState| rr_r8(s, Register::L),        // 0x1D
                |s: &mut GameState| rr_hladdr(s),                 // 0x1E
                |s: &mut GameState| rr_r8(s, Register::A),        // 0x1F
                |s: &mut GameState| sla_r8(s, Register::B),       // 0x20
                |s: &mut GameState| sla_r8(s, Register::C),       // 0x21
                |s: &mut GameState| sla_r8(s, Register::D),       // 0x22
                |s: &mut GameState| sla_r8(s, Register::E),       // 0x23
                |s: &mut GameState| sla_r8(s, Register::H),       // 0x24
                |s: &mut GameState| sla_r8(s, Register::L),       // 0x25
                |s: &mut GameState| sla_hladdr(s),                // 0x26
                |s: &mut GameState| sla_r8(s, Register::A),       // 0x27
                |s: &mut GameState| sra_r8(s, Register::B),       // 0x28
                |s: &mut GameState| sra_r8(s, Register::C),       // 0x29
                |s: &mut GameState| sra_r8(s, Register::D),       // 0x2A
                |s: &mut GameState| sra_r8(s, Register::E),       // 0x2B
                |s: &mut GameState| sra_r8(s, Register::H),       // 0x2C
                |s: &mut GameState| sra_r8(s, Register::L),       // 0x2D
                |s: &mut GameState| sra_hladdr(s),                // 0x2E
                |s: &mut GameState| sra_r8(s, Register::A),       // 0x2F
                |s: &mut GameState| swap_r8(s, Register::B),      // 0x30
                |s: &mut GameState| swap_r8(s, Register::C),      // 0x31
                |s: &mut GameState| swap_r8(s, Register::D),      // 0x32
                |s: &mut GameState| swap_r8(s, Register::E),      // 0x33
                |s: &mut GameState| swap_r8(s, Register::H),      // 0x34
                |s: &mut GameState| swap_r8(s, Register::L),      // 0x35
                |s: &mut GameState| swap_hladdr(s),               // 0x36
                |s: &mut GameState| swap_r8(s, Register::A),      // 0x37
                |s: &mut GameState| srl_r8(s, Register::B),       // 0x38
                |s: &mut GameState| srl_r8(s, Register::C),       // 0x39
                |s: &mut GameState| srl_r8(s, Register::D),       // 0x3A
                |s: &mut GameState| srl_r8(s, Register::E),       // 0x3B
                |s: &mut GameState| srl_r8(s, Register::H),       // 0x3C
                |s: &mut GameState| srl_r8(s, Register::L),       // 0x3D
                |s: &mut GameState| srl_hladdr(s),                // 0x3E
                |s: &mut GameState| srl_r8(s, Register::A),       // 0x3F
                |s: &mut GameState| bit_u3_r8(s, 0, Register::B), // 0x40
                |s: &mut GameState| bit_u3_r8(s, 0, Register::C), // 0x41
                |s: &mut GameState| bit_u3_r8(s, 0, Register::D), // 0x42
                |s: &mut GameState| bit_u3_r8(s, 0, Register::E), // 0x43
                |s: &mut GameState| bit_u3_r8(s, 0, Register::H), // 0x44
                |s: &mut GameState| bit_u3_r8(s, 0, Register::L), // 0x45
                |s: &mut GameState| bit_u3_hladdr(s, 0),          // 0x46
                |s: &mut GameState| bit_u3_r8(s, 0, Register::A), // 0x47
                |s: &mut GameState| bit_u3_r8(s, 1, Register::B), // 0x48
                |s: &mut GameState| bit_u3_r8(s, 1, Register::C), // 0x49
                |s: &mut GameState| bit_u3_r8(s, 1, Register::D), // 0x4A
                |s: &mut GameState| bit_u3_r8(s, 1, Register::E), // 0x4B
                |s: &mut GameState| bit_u3_r8(s, 1, Register::H), // 0x4C
                |s: &mut GameState| bit_u3_r8(s, 1, Register::L), // 0x4D
                |s: &mut GameState| bit_u3_hladdr(s, 1),          // 0x4E
                |s: &mut GameState| bit_u3_r8(s, 1, Register::A), // 0x4F
                |s: &mut GameState| bit_u3_r8(s, 2, Register::B), // 0x50
                |s: &mut GameState| bit_u3_r8(s, 2, Register::C), // 0x51
                |s: &mut GameState| bit_u3_r8(s, 2, Register::D), // 0x52
                |s: &mut GameState| bit_u3_r8(s, 2, Register::E), // 0x53
                |s: &mut GameState| bit_u3_r8(s, 2, Register::H), // 0x54
                |s: &mut GameState| bit_u3_r8(s, 2, Register::L), // 0x55
                |s: &mut GameState| bit_u3_hladdr(s, 2),          // 0x56
                |s: &mut GameState| bit_u3_r8(s, 2, Register::A), // 0x57
                |s: &mut GameState| bit_u3_r8(s, 3, Register::B), // 0x58
                |s: &mut GameState| bit_u3_r8(s, 3, Register::C), // 0x59
                |s: &mut GameState| bit_u3_r8(s, 3, Register::D), // 0x5A
                |s: &mut GameState| bit_u3_r8(s, 3, Register::E), // 0x5B
                |s: &mut GameState| bit_u3_r8(s, 3, Register::H), // 0x5C
                |s: &mut GameState| bit_u3_r8(s, 3, Register::L), // 0x5D
                |s: &mut GameState| bit_u3_hladdr(s, 3),          // 0x5E
                |s: &mut GameState| bit_u3_r8(s, 3, Register::A), // 0x5F
                |s: &mut GameState| bit_u3_r8(s, 4, Register::B), // 0x60
                |s: &mut GameState| bit_u3_r8(s, 4, Register::C), // 0x61
                |s: &mut GameState| bit_u3_r8(s, 4, Register::D), // 0x62
                |s: &mut GameState| bit_u3_r8(s, 4, Register::E), // 0x63
                |s: &mut GameState| bit_u3_r8(s, 4, Register::H), // 0x64
                |s: &mut GameState| bit_u3_r8(s, 4, Register::L), // 0x65
                |s: &mut GameState| bit_u3_hladdr(s, 4),          // 0x66
                |s: &mut GameState| bit_u3_r8(s, 4, Register::A), // 0x67
                |s: &mut GameState| bit_u3_r8(s, 5, Register::B), // 0x68
                |s: &mut GameState| bit_u3_r8(s, 5, Register::C), // 0x69
                |s: &mut GameState| bit_u3_r8(s, 5, Register::D), // 0x6A
                |s: &mut GameState| bit_u3_r8(s, 5, Register::E), // 0x6B
                |s: &mut GameState| bit_u3_r8(s, 5, Register::H), // 0x6C
                |s: &mut GameState| bit_u3_r8(s, 5, Register::L), // 0x6D
                |s: &mut GameState| bit_u3_hladdr(s, 5),          // 0x6E
                |s: &mut GameState| bit_u3_r8(s, 5, Register::A), // 0x6F
                |s: &mut GameState| bit_u3_r8(s, 6, Register::B), // 0x70
                |s: &mut GameState| bit_u3_r8(s, 6, Register::C), // 0x71
                |s: &mut GameState| bit_u3_r8(s, 6, Register::D), // 0x72
                |s: &mut GameState| bit_u3_r8(s, 6, Register::E), // 0x73
                |s: &mut GameState| bit_u3_r8(s, 6, Register::H), // 0x74
                |s: &mut GameState| bit_u3_r8(s, 6, Register::L), // 0x75
                |s: &mut GameState| bit_u3_hladdr(s, 6),          // 0x76
                |s: &mut GameState| bit_u3_r8(s, 6, Register::A), // 0x77
                |s: &mut GameState| bit_u3_r8(s, 7, Register::B), // 0x78
                |s: &mut GameState| bit_u3_r8(s, 7, Register::C), // 0x79
                |s: &mut GameState| bit_u3_r8(s, 7, Register::D), // 0x7A
                |s: &mut GameState| bit_u3_r8(s, 7, Register::E), // 0x7B
                |s: &mut GameState| bit_u3_r8(s, 7, Register::H), // 0x7C
                |s: &mut GameState| bit_u3_r8(s, 7, Register::L), // 0x7D
                |s: &mut GameState| bit_u3_hladdr(s, 7),          // 0x7E
                |s: &mut GameState| bit_u3_r8(s, 7, Register::A), // 0x7F
                |s: &mut GameState| res_u3_r8(s, 0, Register::B), // 0x80
                |s: &mut GameState| res_u3_r8(s, 0, Register::C), // 0x81
                |s: &mut GameState| res_u3_r8(s, 0, Register::D), // 0x82
                |s: &mut GameState| res_u3_r8(s, 0, Register::E), // 0x83
                |s: &mut GameState| res_u3_r8(s, 0, Register::H), // 0x84
                |s: &mut GameState| res_u3_r8(s, 0, Register::L), // 0x85
                |s: &mut GameState| res_u3_hladdr(s, 0),          // 0x86
                |s: &mut GameState| res_u3_r8(s, 0, Register::A), // 0x87
                |s: &mut GameState| res_u3_r8(s, 1, Register::B), // 0x88
                |s: &mut GameState| res_u3_r8(s, 1, Register::C), // 0x89
                |s: &mut GameState| res_u3_r8(s, 1, Register::D), // 0x8A
                |s: &mut GameState| res_u3_r8(s, 1, Register::E), // 0x8B
                |s: &mut GameState| res_u3_r8(s, 1, Register::H), // 0x8C
                |s: &mut GameState| res_u3_r8(s, 1, Register::L), // 0x8D
                |s: &mut GameState| res_u3_hladdr(s, 1),          // 0x8E
                |s: &mut GameState| res_u3_r8(s, 1, Register::A), // 0x8F
                |s: &mut GameState| res_u3_r8(s, 2, Register::B), // 0x90
                |s: &mut GameState| res_u3_r8(s, 2, Register::C), // 0x91
                |s: &mut GameState| res_u3_r8(s, 2, Register::D), // 0x92
                |s: &mut GameState| res_u3_r8(s, 2, Register::E), // 0x93
                |s: &mut GameState| res_u3_r8(s, 2, Register::H), // 0x94
                |s: &mut GameState| res_u3_r8(s, 2, Register::L), // 0x95
                |s: &mut GameState| res_u3_hladdr(s, 2),          // 0x96
                |s: &mut GameState| res_u3_r8(s, 2, Register::A), // 0x97
                |s: &mut GameState| res_u3_r8(s, 3, Register::B), // 0x98
                |s: &mut GameState| res_u3_r8(s, 3, Register::C), // 0x99
                |s: &mut GameState| res_u3_r8(s, 3, Register::D), // 0x9A
                |s: &mut GameState| res_u3_r8(s, 3, Register::E), // 0x9B
                |s: &mut GameState| res_u3_r8(s, 3, Register::H), // 0x9C
                |s: &mut GameState| res_u3_r8(s, 3, Register::L), // 0x9D
                |s: &mut GameState| res_u3_hladdr(s, 3),          // 0x9E
                |s: &mut GameState| res_u3_r8(s, 3, Register::A), // 0x9F
                |s: &mut GameState| res_u3_r8(s, 4, Register::B), // 0xA0
                |s: &mut GameState| res_u3_r8(s, 4, Register::C), // 0xA1
                |s: &mut GameState| res_u3_r8(s, 4, Register::D), // 0xA2
                |s: &mut GameState| res_u3_r8(s, 4, Register::E), // 0xA3
                |s: &mut GameState| res_u3_r8(s, 4, Register::H), // 0xA4
                |s: &mut GameState| res_u3_r8(s, 4, Register::L), // 0xA5
                |s: &mut GameState| res_u3_hladdr(s, 4),          // 0xA6
                |s: &mut GameState| res_u3_r8(s, 4, Register::A), // 0xA7
                |s: &mut GameState| res_u3_r8(s, 5, Register::B), // 0xA8
                |s: &mut GameState| res_u3_r8(s, 5, Register::C), // 0xA9
                |s: &mut GameState| res_u3_r8(s, 5, Register::D), // 0xAA
                |s: &mut GameState| res_u3_r8(s, 5, Register::E), // 0xAB
                |s: &mut GameState| res_u3_r8(s, 5, Register::H), // 0xAC
                |s: &mut GameState| res_u3_r8(s, 5, Register::L), // 0xAD
                |s: &mut GameState| res_u3_hladdr(s, 5),          // 0xAE
                |s: &mut GameState| res_u3_r8(s, 5, Register::A), // 0xAF
                |s: &mut GameState| res_u3_r8(s, 6, Register::B), // 0xB0
                |s: &mut GameState| res_u3_r8(s, 6, Register::C), // 0xB1
                |s: &mut GameState| res_u3_r8(s, 6, Register::D), // 0xB2
                |s: &mut GameState| res_u3_r8(s, 6, Register::E), // 0xB3
                |s: &mut GameState| res_u3_r8(s, 6, Register::H), // 0xB4
                |s: &mut GameState| res_u3_r8(s, 6, Register::L), // 0xB5
                |s: &mut GameState| res_u3_hladdr(s, 6),          // 0xB6
                |s: &mut GameState| res_u3_r8(s, 6, Register::A), // 0xB7
                |s: &mut GameState| res_u3_r8(s, 7, Register::B), // 0xB8
                |s: &mut GameState| res_u3_r8(s, 7, Register::C), // 0xB9
                |s: &mut GameState| res_u3_r8(s, 7, Register::D), // 0xBA
                |s: &mut GameState| res_u3_r8(s, 7, Register::E), // 0xBB
                |s: &mut GameState| res_u3_r8(s, 7, Register::H), // 0xBC
                |s: &mut GameState| res_u3_r8(s, 7, Register::L), // 0xBD
                |s: &mut GameState| res_u3_hladdr(s, 7),          // 0xBE
                |s: &mut GameState| res_u3_r8(s, 7, Register::A), // 0xBF
                |s: &mut GameState| set_u3_r8(s, 0, Register::B), // 0xC0
                |s: &mut GameState| set_u3_r8(s, 0, Register::C), // 0xC1
                |s: &mut GameState| set_u3_r8(s, 0, Register::D), // 0xC2
                |s: &mut GameState| set_u3_r8(s, 0, Register::E), // 0xC3
                |s: &mut GameState| set_u3_r8(s, 0, Register::H), // 0xC4
                |s: &mut GameState| set_u3_r8(s, 0, Register::L), // 0xC5
                |s: &mut GameState| set_u3_hladdr(s, 0),          // 0xC6
                |s: &mut GameState| set_u3_r8(s, 0, Register::A), // 0xC7
                |s: &mut GameState| set_u3_r8(s, 1, Register::B), // 0xC8
                |s: &mut GameState| set_u3_r8(s, 1, Register::C), // 0xC9
                |s: &mut GameState| set_u3_r8(s, 1, Register::D), // 0xCA
                |s: &mut GameState| set_u3_r8(s, 1, Register::E), // 0xCB
                |s: &mut GameState| set_u3_r8(s, 1, Register::H), // 0xCC
                |s: &mut GameState| set_u3_r8(s, 1, Register::L), // 0xCD
                |s: &mut GameState| set_u3_hladdr(s, 1),          // 0xCE
                |s: &mut GameState| set_u3_r8(s, 1, Register::A), // 0xCF
                |s: &mut GameState| set_u3_r8(s, 2, Register::B), // 0xD0
                |s: &mut GameState| set_u3_r8(s, 2, Register::C), // 0xD1
                |s: &mut GameState| set_u3_r8(s, 2, Register::D), // 0xD2
                |s: &mut GameState| set_u3_r8(s, 2, Register::E), // 0xD3
                |s: &mut GameState| set_u3_r8(s, 2, Register::H), // 0xD4
                |s: &mut GameState| set_u3_r8(s, 2, Register::L), // 0xD5
                |s: &mut GameState| set_u3_hladdr(s, 2),          // 0xD6
                |s: &mut GameState| set_u3_r8(s, 2, Register::A), // 0xD7
                |s: &mut GameState| set_u3_r8(s, 3, Register::B), // 0xD8
                |s: &mut GameState| set_u3_r8(s, 3, Register::C), // 0xD9
                |s: &mut GameState| set_u3_r8(s, 3, Register::D), // 0xDA
                |s: &mut GameState| set_u3_r8(s, 3, Register::E), // 0xDB
                |s: &mut GameState| set_u3_r8(s, 3, Register::H), // 0xDC
                |s: &mut GameState| set_u3_r8(s, 3, Register::L), // 0xDD
                |s: &mut GameState| set_u3_hladdr(s, 3),          // 0xDE
                |s: &mut GameState| set_u3_r8(s, 3, Register::A), // 0xDF
                |s: &mut GameState| set_u3_r8(s, 4, Register::B), // 0xE0
                |s: &mut GameState| set_u3_r8(s, 4, Register::C), // 0xE1
                |s: &mut GameState| set_u3_r8(s, 4, Register::D), // 0xE2
                |s: &mut GameState| set_u3_r8(s, 4, Register::E), // 0xE3
                |s: &mut GameState| set_u3_r8(s, 4, Register::H), // 0xE4
                |s: &mut GameState| set_u3_r8(s, 4, Register::L), // 0xE5
                |s: &mut GameState| set_u3_hladdr(s, 4),          // 0xE6
                |s: &mut GameState| set_u3_r8(s, 4, Register::A), // 0xE7
                |s: &mut GameState| set_u3_r8(s, 5, Register::B), // 0xE8
                |s: &mut GameState| set_u3_r8(s, 5, Register::C), // 0xE9
                |s: &mut GameState| set_u3_r8(s, 5, Register::D), // 0xEA
                |s: &mut GameState| set_u3_r8(s, 5, Register::E), // 0xEB
                |s: &mut GameState| set_u3_r8(s, 5, Register::H), // 0xEC
                |s: &mut GameState| set_u3_r8(s, 5, Register::L), // 0xED
                |s: &mut GameState| set_u3_hladdr(s, 5),          // 0xEE
                |s: &mut GameState| set_u3_r8(s, 5, Register::A), // 0xEF
                |s: &mut GameState| set_u3_r8(s, 6, Register::B), // 0xF0
                |s: &mut GameState| set_u3_r8(s, 6, Register::C), // 0xF1
                |s: &mut GameState| set_u3_r8(s, 6, Register::D), // 0xF2
                |s: &mut GameState| set_u3_r8(s, 6, Register::E), // 0xF3
                |s: &mut GameState| set_u3_r8(s, 6, Register::H), // 0xF4
                |s: &mut GameState| set_u3_r8(s, 6, Register::L), // 0xF5
                |s: &mut GameState| set_u3_hladdr(s, 6),          // 0xF6
                |s: &mut GameState| set_u3_r8(s, 6, Register::A), // 0xF7
                |s: &mut GameState| set_u3_r8(s, 7, Register::B), // 0xF8
                |s: &mut GameState| set_u3_r8(s, 7, Register::C), // 0xF9
                |s: &mut GameState| set_u3_r8(s, 7, Register::D), // 0xFA
                |s: &mut GameState| set_u3_r8(s, 7, Register::E), // 0xFB
                |s: &mut GameState| set_u3_r8(s, 7, Register::H), // 0xFC
                |s: &mut GameState| set_u3_r8(s, 7, Register::L), // 0xFD
                |s: &mut GameState| set_u3_hladdr(s, 7),          // 0xFE
                |s: &mut GameState| set_u3_r8(s, 7, Register::A), // 0xFF
            ],

            two_byte_ins: iibi,

            three_byte_ins: iiibi,
        }
    }

    pub fn step(&self, game_state: &mut GameState) -> u8 {
        let curr_pc = game_state.get_register16(Register::PC);
        let next_instruction = game_state.read(curr_pc);
        print!("PC: 0x{:04X}, OP: 0x{:02X}", curr_pc, next_instruction);
        let cycles;

        let mut advance_amount = 1;
        if next_instruction != 0xCB {
            if self.two_byte_ins.contains(&next_instruction) {
                advance_amount = 2;
                print!(", Byte: 0x{:02X} ", game_state.read(curr_pc + 1));
            } else if self.three_byte_ins.contains(&next_instruction) {
                advance_amount = 3;
                print!(
                    ", Bytes: 0x{:02X}, 0x{:02X} ",
                    game_state.read(curr_pc + 1),
                    game_state.read(curr_pc + 2)
                );
            }
            cycles = (self.non_prefix_opcodes[next_instruction as usize])(game_state);
        } else {
            let actual_ins = game_state.read(curr_pc + 1);
            cycles = (self.cb_prefix_opcodes[actual_ins as usize])(game_state);
            advance_amount = 2;
        }
        println!();

        if game_state.pc_moved() {
            advance_amount = 0;
            game_state.set_pc_moved(false);
        }

        game_state.update_clock(cycles);

        // it is possible for curr_pc and Register::PC to disagree at this point
        game_state.set_register16(
            Register::PC,
            game_state.get_register16(Register::PC) + advance_amount,
        );
        cycles
    }
}
