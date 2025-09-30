pub const non_prefix_strings: &[&str] = &[
    "NOP",                                     // 0x00
    "ld_r16_n16(s, Register::BC)",           // 0x01
    "ld_r16addr_a(s, Register::BC)",         // 0x02
    "inc_r16(s, Register::BC)",              // 0x03
    "inc_r8(s, Register::B)",                // 0x04
    "dec_r8(s, Register::B)",                // 0x05
    "ld_r8_n8(s, Register::B)",              // 0x06
    "rlca(s)",                               // 0x07
    "ld_n16addr_sp(s)",                      // 0x08
    "add_hl_r16(s, Register::BC)",           // 0x09
    "ld_a_r16addr(s, Register::BC)",         // 0x0A
    "dec_r16(s, Register::BC)",              // 0x0B
    "inc_r8(s, Register::C)",                // 0x0C
    "dec_r8(s, Register::C)",                // 0x0D
    "ld_r8_n8(s, Register::C)",              // 0x0E
    "rrca(s)",                               // 0x0F
    "stop(s)",                               // 0x10
    "ld_r16_n16(s, Register::DE)",           // 0x11
    "ld_r16addr_a(s, Register::DE)",         // 0x12
    "inc_r16(s, Register::DE)",              // 0x13
    "inc_r8(s, Register::D)",                // 0x14
    "dec_r8(s, Register::D)",                // 0x15
    "ld_r8_n8(s, Register::D)",              // 0x16
    "rla(s)",                                // 0x17
    "jr_e8(s)",                             // 0x18
    "add_hl_r16(s, Register::DE)",           // 0x19
    "ld_a_r16addr(s, Register::DE)",         // 0x1A
    "dec_r16(s, Register::DE)",              // 0x1B
    "inc_r8(s, Register::E)",                // 0x1C
    "dec_r8(s, Register::E)",                // 0x1D
    "ld_r8_n8(s, Register::E)",              // 0x1E
    "rra(s)",                                // 0x1F
    "jr_cc(s, CC::NZ)",           // 0x20
    "ld_r16_n16(s, Register::HL)",           // 0x21
    "ld_hliaddr_a(s)",                       // 0x22
    "inc_r16(s, Register::HL)",              // 0x23
    "inc_r8(s, Register::H)",                // 0x24
    "dec_r8(s, Register::H)",                // 0x25
    "ld_r8_n8(s, Register::H)",              // 0x26
    "daa(s)",                                // 0x27
    "jr_cc(s, CC::Z)",          // 0x28
    "add_hl_r16(s, Register::HL)",           // 0x29
    "ld_a_hli(s)",                           // 0x2A
    "dec_r16(s, Register::HL)",              // 0x2B
    "inc_r8(s, Register::L)",                // 0x2C
    "dec_r8(s, Register::L)",                // 0x2D
    "ld_r8_n8(s, Register::L)",              // 0x2E
    "cpl(s)",                                // 0x2F
    "jr_cc(s, CC::NC)",           // 0x30
    "ld_sp_n16addr(s)",                      // 0x31
    "ld_hldaddr_a(s)",                       // 0x32
    "inc_sp(s)",                             // 0x33
    "inc_hladdr(s)",                         // 0x34
    "dec_hladdr(s)",                         // 0x35
    "ld_hladdr_n8(s)",                       // 0x36
    "scf(s)",                                // 0x37
    "jr_cc(s, CC::C)",          // 0x38
    "add_hl_sp(s)",                          // 0x39
    "ld_a_hld(s)",                           // 0x3A
    "dec_sp(s)",                             // 0x3B
    "inc_r8(s, Register::A)",                // 0x3C
    "dec_r8(s, Register::A)",                // 0x3D
    "ld_r8_n8(s, Register::A)",              // 0x3E
    "ccf(s)",                                // 0x3F
    "ld_r8_r8(s, Register::B, Register::B)", // 0x40
    "ld_r8_r8(s, Register::B, Register::C)", // 0x41
    "ld_r8_r8(s, Register::B, Register::D)", // 0x42
    "ld_r8_r8(s, Register::B, Register::E)", // 0x43
    "ld_r8_r8(s, Register::B, Register::H)", // 0x44
    "ld_r8_r8(s, Register::B, Register::L)", // 0x45
    "ld_r8_hladdr(s, Register::B)",          // 0x46
    "ld_r8_r8(s, Register::B, Register::A)", // 0x47
    "ld_r8_r8(s, Register::C, Register::B)", // 0x48
    "ld_r8_r8(s, Register::C, Register::C)", // 0x49
    "ld_r8_r8(s, Register::C, Register::D)", // 0x4A
    "ld_r8_r8(s, Register::C, Register::E)", // 0x4B
    "ld_r8_r8(s, Register::C, Register::H)", // 0x4C
    "ld_r8_r8(s, Register::C, Register::L)", // 0x4D
    "ld_r8_hladdr(s, Register::C)",          // 0x4E
    "ld_r8_r8(s, Register::C, Register::A)", // 0x4F
    "ld_r8_r8(s, Register::D, Register::B)", // 0x50
    "ld_r8_r8(s, Register::D, Register::C)", // 0x51
    "ld_r8_r8(s, Register::D, Register::D)", // 0x52
    "ld_r8_r8(s, Register::D, Register::E)", // 0x53
    "ld_r8_r8(s, Register::D, Register::H)", // 0x54
    "ld_r8_r8(s, Register::D, Register::L)", // 0x55
    "ld_r8_hladdr(s, Register::D)",          // 0x56
    "ld_r8_r8(s, Register::D, Register::A)", // 0x57
    "ld_r8_r8(s, Register::E, Register::B)", // 0x58
    "ld_r8_r8(s, Register::E, Register::C)", // 0x59
    "ld_r8_r8(s, Register::E, Register::D)", // 0x5A
    "ld_r8_r8(s, Register::E, Register::E)", // 0x5B
    "ld_r8_r8(s, Register::E, Register::H)", // 0x5C
    "ld_r8_r8(s, Register::E, Register::L)", // 0x5D
    "ld_r8_hladdr(s, Register::E)",          // 0x5E
    "ld_r8_r8(s, Register::E, Register::A)", // 0x5F
    "ld_r8_r8(s, Register::H, Register::B)", // 0x60
    "ld_r8_r8(s, Register::H, Register::C)", // 0x61
    "ld_r8_r8(s, Register::H, Register::D)", // 0x62
    "ld_r8_r8(s, Register::H, Register::E)", // 0x63
    "ld_r8_r8(s, Register::H, Register::H)", // 0x64
    "ld_r8_r8(s, Register::H, Register::L)", // 0x65
    "ld_r8_hladdr(s, Register::H)",          // 0x66
    "ld_r8_r8(s, Register::H, Register::A)", // 0x67
    "ld_r8_r8(s, Register::L, Register::B)", // 0x68
    "ld_r8_r8(s, Register::L, Register::C)", // 0x69
    "ld_r8_r8(s, Register::L, Register::D)", // 0x6A
    "ld_r8_r8(s, Register::L, Register::E)", // 0x6B
    "ld_r8_r8(s, Register::L, Register::H)", // 0x6C
    "ld_r8_r8(s, Register::L, Register::L)", // 0x6D
    "ld_r8_hladdr(s, Register::L)",          // 0x6E
    "ld_r8_r8(s, Register::L, Register::A)", // 0x6F
    "ld_hladdr_r8(s, Register::B)",          // 0x70
    "ld_hladdr_r8(s, Register::C)",          // 0x71
    "ld_hladdr_r8(s, Register::D)",          // 0x72
    "ld_hladdr_r8(s, Register::E)",          // 0x73
    "ld_hladdr_r8(s, Register::H)",          // 0x74
    "ld_hladdr_r8(s, Register::L)",          // 0x75
    "halt(s)",                               // 0x76
    "ld_hladdr_r8(s, Register::A)",          // 0x75
    "ld_r8_r8(s, Register::A, Register::B)", // 0x78
    "ld_r8_r8(s, Register::A, Register::C)", // 0x79
    "ld_r8_r8(s, Register::A, Register::D)", // 0x7A
    "ld_r8_r8(s, Register::A, Register::E)", // 0x7B
    "ld_r8_r8(s, Register::A, Register::H)", // 0x7C
    "ld_r8_r8(s, Register::A, Register::L)", // 0x7D
    "ld_r8_hladdr(s, Register::A)",          // 0x7E
    "ld_r8_r8(s, Register::A, Register::A)", // 0x7F
    "add_a_r8(s, Register::B)",              // 0x80
    "add_a_r8(s, Register::C)",              // 0x81
    "add_a_r8(s, Register::D)",              // 0x82
    "add_a_r8(s, Register::E)",              // 0x83
    "add_a_r8(s, Register::H)",              // 0x84
    "add_a_r8(s, Register::L)",              // 0x85
    "add_a_hladdr(s)",                       // 0x86
    "add_a_r8(s, Register::A)",              // 0x87
    "adc_a_r8(s, Register::B)",              // 0x88
    "adc_a_r8(s, Register::C)",              // 0x89
    "adc_a_r8(s, Register::D)",              // 0x8A
    "adc_a_r8(s, Register::E)",              // 0x8B
    "adc_a_r8(s, Register::H)",              // 0x8C
    "adc_a_r8(s, Register::L)",              // 0x8D
    "adc_a_hladdr(s)",                       // 0x8E
    "adc_a_r8(s, Register::A)",              // 0x8F
    "sub_a_r8(s, Register::B)",              // 0x90
    "sub_a_r8(s, Register::C)",              // 0x91
    "sub_a_r8(s, Register::D)",              // 0x92
    "sub_a_r8(s, Register::E)",              // 0x93
    "sub_a_r8(s, Register::H)",              // 0x94
    "sub_a_r8(s, Register::L)",              // 0x95
    "sub_a_hladdr(s)",                       // 0x96
    "sub_a_r8(s, Register::A)",              // 0x97
    "sbc_a_r8(s, Register::B)",              // 0x98
    "sbc_a_r8(s, Register::C)",              // 0x99
    "sbc_a_r8(s, Register::D)",              // 0x9A
    "sbc_a_r8(s, Register::E)",              // 0x9B
    "sbc_a_r8(s, Register::H)",              // 0x9C
    "sbc_a_r8(s, Register::L)",              // 0x9D
    "sbc_a_hladdr(s)",                       // 0x9E
    "sbc_a_r8(s, Register::A)",              // 0x9F
    "and_a_r8(s, Register::B)",              // 0xA0
    "and_a_r8(s, Register::C)",              // 0xA1
    "and_a_r8(s, Register::D)",              // 0xA2
    "and_a_r8(s, Register::E)",              // 0xA3
    "and_a_r8(s, Register::H)",              // 0xA4
    "and_a_r8(s, Register::L)",              // 0xA5
    "and_a_hladdr(s)",                       // 0xA6
    "and_a_r8(s, Register::A)",              // 0xA7
    "xor_a_r8(s, Register::B)",              // 0xA8
    "xor_a_r8(s, Register::C)",              // 0xA9
    "xor_a_r8(s, Register::D)",              // 0xAA
    "xor_a_r8(s, Register::E)",              // 0xAB
    "xor_a_r8(s, Register::H)",              // 0xAC
    "xor_a_r8(s, Register::L)",              // 0xAD
    "xor_a_hladdr(s)",                       // 0xAE
    "xor_a_r8(s, Register::A)",              // 0xAF
    "or_a_r8(s, Register::B)",               // 0xB0
    "or_a_r8(s, Register::C)",               // 0xB1
    "or_a_r8(s, Register::D)",               // 0xB2
    "or_a_r8(s, Register::E)",               // 0xB3
    "or_a_r8(s, Register::H)",               // 0xB4
    "or_a_r8(s, Register::L)",               // 0xB5
    "or_a_hladdr(s)",                        // 0xB6
    "or_a_r8(s, Register::A)",               // 0xB7
    "cp_a_r8(s, Register::B)",               // 0xB8
    "cp_a_r8(s, Register::C)",               // 0xB9
    "cp_a_r8(s, Register::D)",               // 0xBA
    "cp_a_r8(s, Register::E)",               // 0xBB
    "cp_a_r8(s, Register::H)",               // 0xBC
    "cp_a_r8(s, Register::L)",               // 0xBD
    "cp_a_hladdr(s)",                        // 0xBE
    "cp_a_r8(s, Register::A)",               // 0xBF
    "ret_cc(s, CC::NZ)",          // 0xC0
    "pop_r16(s, Register::BC)",              // 0xC1
    "jp_cc(s, CC::NZ)",           // 0xC2
    "jp_n16(s)",                             // 0xC3
    "call_cc(s, CC::NZ)",         // 0xC4
    "push_r16(s, Register::BC)",             // 0xC5
    "add_a_n8(s)",                           // 0xC6
    "rst_vec(s, 0x00)",                      // 0xC7
    "ret_cc(s, CC::Z)",         // 0xC8
    "ret(s)",                                // 0xC9
    "jp_cc(s, CC::Z)",          // 0xCA
    "1",                                     // 0xCB PREFIX!
    "call_cc(s, CC::Z)",        // 0xCC
    "call_n16(s)",                           // 0xCD
    "adc_a_n8(s)",                           // 0xCE
    "rst_vec(s, 0x08)",                      // 0xCF
    "ret_cc(s, CC::NC)",          // 0xD0
    "pop_r16(s, Register::DE)",              // 0xD1
    "jp_cc(s, CC::NC)",           // 0xD2
    "1",                                     // 0xD3 Blank
    "call_cc(s, CC::NC)",         // 0xD4
    "push_r16(s, Register::DE)",             // 0xD5
    "sub_a_n8(s)",                           // 0xD6
    "rst_vec(s, 0x10)",                      // 0xD7
    "ret_cc(s, CC::C)",         // 0xD8
    "reti(s)",                               // 0xD9
    "jp_cc(s, CC::C)",          // 0xDA
    "1",                                     // 0xDB Blank
    "call_cc(s, CC::C)",        // 0xDC
    "1",                                     // 0xDD Blank
    "sbc_a_n8(s)",                           // 0xDE
    "rst_vec(s, 0x18)",                      // 0xDF
    "ldh_n8addr_a(s)",                       // 0xE0
    "pop_r16(s, Register::HL)",              // 0xE1
    "ldh_caddr_a(s)",                        // 0xE2
    "1",                                     // 0xE3 Blank
    "1",                                     // 0xE4 Blank
    "push_r16(s, Register::HL)",             // 0xE5
    "and_a_n8(s)",                           // 0xE6
    "rst_vec(s, 0x20)",                      // 0xE7
    "add_sp_e8(s)",                          // 0xE8
    "jp_hl(s)",                              // 0xE9
    "ld_n16addr_a(s)",                       // 0xEA
    "1",                                     // 0xEB Blank
    "1",                                     // 0xEC Blank
    "1",                                     // 0xED Blank
    "xor_a_n8(s)",                           // 0xEE
    "rst_vec(s, 0x28)",                      // 0xEF
    "ldh_a_n8addr(s)",                       // 0xF0
    "pop_r16(s, Register::AF)",              // 0xF1
    "ldh_a_caddr(s)",                        // 0xF2
    "di(s)",                                 // 0xF3 Blank
    "1",                                     // 0xF4 Blank
    "push_r16(s, Register::AF)",             // 0xF5
    "or_a_n8(s)",                            // 0xF6
    "rst_vec(s, 0x30)",                      // 0xF7
    "ld_hl_spe8(s)",                         // 0xF8
    "ld_sp_hl(s)",                           // 0xF9
    "ld_a_n16addr(s)",                       // 0xFA
    "ei(s)",                                 // 0xFB
    "1",                                     // 0xFC Blank
    "1",                                     // 0xFD Blank
    "cp_a_n8(s)",                            // 0xFE
    "rst_vec(s, 0x38)",                      // 0xFF
];

pub const cb_prefix_strings: &[&str] = &[
    "rlc_r8(s, Register::B)",       // 0x00
    "rlc_r8(s, Register::C)",       // 0x01
    "rlc_r8(s, Register::D)",       // 0x02
    "rlc_r8(s, Register::E)",       // 0x03
    "rlc_r8(s, Register::H)",       // 0x04
    "rlc_r8(s, Register::L)",       // 0x05
    "rlc_hladdr(s)",                // 0x06
    "rlc_r8(s, Register::A)",       // 0x07
    "rrc_r8(s, Register::B)",       // 0x08
    "rrc_r8(s, Register::C)",       // 0x09
    "rrc_r8(s, Register::D)",       // 0x0A
    "rrc_r8(s, Register::E)",       // 0x0B
    "rrc_r8(s, Register::H)",       // 0x0C
    "rrc_r8(s, Register::L)",       // 0x0D
    "rrc_hladdr(s)",                // 0x0E
    "rrc_r8(s, Register::A)",       // 0x0F
    "rl_r8(s, Register::B)",        // 0x10
    "rl_r8(s, Register::C)",        // 0x11
    "rl_r8(s, Register::D)",        // 0x12
    "rl_r8(s, Register::E)",        // 0x13
    "rl_r8(s, Register::H)",        // 0x14
    "rl_r8(s, Register::L)",        // 0x15
    "rl_hladdr(s)",                 // 0x16
    "rl_r8(s, Register::A)",        // 0x17
    "rr_r8(s, Register::B)",        // 0x18
    "rr_r8(s, Register::C)",        // 0x19
    "rr_r8(s, Register::D)",        // 0x1A
    "rr_r8(s, Register::E)",        // 0x1B
    "rr_r8(s, Register::H)",        // 0x1C
    "rr_r8(s, Register::L)",        // 0x1D
    "rr_hladdr(s)",                 // 0x1E
    "rr_r8(s, Register::A)",        // 0x1F
    "sla_r8(s, Register::B)",       // 0x20
    "sla_r8(s, Register::C)",       // 0x21
    "sla_r8(s, Register::D)",       // 0x22
    "sla_r8(s, Register::E)",       // 0x23
    "sla_r8(s, Register::H)",       // 0x24
    "sla_r8(s, Register::L)",       // 0x25
    "sla_hladdr(s)",                // 0x26
    "sla_r8(s, Register::A)",       // 0x27
    "sra_r8(s, Register::B)",       // 0x28
    "sra_r8(s, Register::C)",       // 0x29
    "sra_r8(s, Register::D)",       // 0x2A
    "sra_r8(s, Register::E)",       // 0x2B
    "sra_r8(s, Register::H)",       // 0x2C
    "sra_r8(s, Register::L)",       // 0x2D
    "sra_hladdr(s)",                // 0x2E
    "sra_r8(s, Register::A)",       // 0x2F
    "swap_r8(s, Register::B)",      // 0x30
    "swap_r8(s, Register::C)",      // 0x31
    "swap_r8(s, Register::D)",      // 0x32
    "swap_r8(s, Register::E)",      // 0x33
    "swap_r8(s, Register::H)",      // 0x34
    "swap_r8(s, Register::L)",      // 0x35
    "swap_hladdr(s)",               // 0x36
    "swap_r8(s, Register::A)",      // 0x37
    "srl_r8(s, Register::B)",       // 0x38
    "srl_r8(s, Register::C)",       // 0x39
    "srl_r8(s, Register::D)",       // 0x3A
    "srl_r8(s, Register::E)",       // 0x3B
    "srl_r8(s, Register::H)",       // 0x3C
    "srl_r8(s, Register::L)",       // 0x3D
    "srl_hladdr(s)",                // 0x3E
    "srl_r8(s, Register::A)",       // 0x3F
    "bit_u3_r8(s, 0, Register::B)", // 0x40
    "bit_u3_r8(s, 0, Register::C)", // 0x41
    "bit_u3_r8(s, 0, Register::D)", // 0x42
    "bit_u3_r8(s, 0, Register::E)", // 0x43
    "bit_u3_r8(s, 0, Register::H)", // 0x44
    "bit_u3_r8(s, 0, Register::L)", // 0x45
    "bit_u3_hladdr(s, 0)",          // 0x46
    "bit_u3_r8(s, 0, Register::A)", // 0x47
    "bit_u3_r8(s, 1, Register::B)", // 0x48
    "bit_u3_r8(s, 1, Register::C)", // 0x49
    "bit_u3_r8(s, 1, Register::D)", // 0x4A
    "bit_u3_r8(s, 1, Register::E)", // 0x4B
    "bit_u3_r8(s, 1, Register::H)", // 0x4C
    "bit_u3_r8(s, 1, Register::L)", // 0x4D
    "bit_u3_hladdr(s, 1)",          // 0x4E
    "bit_u3_r8(s, 1, Register::A)", // 0x4F
    "bit_u3_r8(s, 2, Register::B)", // 0x50
    "bit_u3_r8(s, 2, Register::C)", // 0x51
    "bit_u3_r8(s, 2, Register::D)", // 0x52
    "bit_u3_r8(s, 2, Register::E)", // 0x53
    "bit_u3_r8(s, 2, Register::H)", // 0x54
    "bit_u3_r8(s, 2, Register::L)", // 0x55
    "bit_u3_hladdr(s, 2)",          // 0x56
    "bit_u3_r8(s, 2, Register::A)", // 0x57
    "bit_u3_r8(s, 3, Register::B)", // 0x58
    "bit_u3_r8(s, 3, Register::C)", // 0x59
    "bit_u3_r8(s, 3, Register::D)", // 0x5A
    "bit_u3_r8(s, 3, Register::E)", // 0x5B
    "bit_u3_r8(s, 3, Register::H)", // 0x5C
    "bit_u3_r8(s, 3, Register::L)", // 0x5D
    "bit_u3_hladdr(s, 3)",          // 0x5E
    "bit_u3_r8(s, 3, Register::A)", // 0x5F
    "bit_u3_r8(s, 4, Register::B)", // 0x60
    "bit_u3_r8(s, 4, Register::C)", // 0x61
    "bit_u3_r8(s, 4, Register::D)", // 0x62
    "bit_u3_r8(s, 4, Register::E)", // 0x63
    "bit_u3_r8(s, 4, Register::H)", // 0x64
    "bit_u3_r8(s, 4, Register::L)", // 0x65
    "bit_u3_hladdr(s, 4)",          // 0x66
    "bit_u3_r8(s, 4, Register::A)", // 0x67
    "bit_u3_r8(s, 5, Register::B)", // 0x68
    "bit_u3_r8(s, 5, Register::C)", // 0x69
    "bit_u3_r8(s, 5, Register::D)", // 0x6A
    "bit_u3_r8(s, 5, Register::E)", // 0x6B
    "bit_u3_r8(s, 5, Register::H)", // 0x6C
    "bit_u3_r8(s, 5, Register::L)", // 0x6D
    "bit_u3_hladdr(s, 5)",          // 0x6E
    "bit_u3_r8(s, 5, Register::A)", // 0x6F
    "bit_u3_r8(s, 6, Register::B)", // 0x70
    "bit_u3_r8(s, 6, Register::C)", // 0x71
    "bit_u3_r8(s, 6, Register::D)", // 0x72
    "bit_u3_r8(s, 6, Register::E)", // 0x73
    "bit_u3_r8(s, 6, Register::H)", // 0x74
    "bit_u3_r8(s, 6, Register::L)", // 0x75
    "bit_u3_hladdr(s, 6)",          // 0x76
    "bit_u3_r8(s, 6, Register::A)", // 0x77
    "bit_u3_r8(s, 7, Register::B)", // 0x78
    "bit_u3_r8(s, 7, Register::C)", // 0x79
    "bit_u3_r8(s, 7, Register::D)", // 0x7A
    "bit_u3_r8(s, 7, Register::E)", // 0x7B
    "bit_u3_r8(s, 7, Register::H)", // 0x7C
    "bit_u3_r8(s, 7, Register::L)", // 0x7D
    "bit_u3_hladdr(s, 7)",          // 0x7E
    "bit_u3_r8(s, 7, Register::A)", // 0x7F
    "res_u3_r8(s, 0, Register::B)", // 0x80
    "res_u3_r8(s, 0, Register::C)", // 0x81
    "res_u3_r8(s, 0, Register::D)", // 0x82
    "res_u3_r8(s, 0, Register::E)", // 0x83
    "res_u3_r8(s, 0, Register::H)", // 0x84
    "res_u3_r8(s, 0, Register::L)", // 0x85
    "res_u3_hladdr(s, 0)",          // 0x86
    "res_u3_r8(s, 0, Register::A)", // 0x87
    "res_u3_r8(s, 1, Register::B)", // 0x88
    "res_u3_r8(s, 1, Register::C)", // 0x89
    "res_u3_r8(s, 1, Register::D)", // 0x8A
    "res_u3_r8(s, 1, Register::E)", // 0x8B
    "res_u3_r8(s, 1, Register::H)", // 0x8C
    "res_u3_r8(s, 1, Register::L)", // 0x8D
    "res_u3_hladdr(s, 1)",          // 0x8E
    "res_u3_r8(s, 1, Register::A)", // 0x8F
    "res_u3_r8(s, 2, Register::B)", // 0x90
    "res_u3_r8(s, 2, Register::C)", // 0x91
    "res_u3_r8(s, 2, Register::D)", // 0x92
    "res_u3_r8(s, 2, Register::E)", // 0x93
    "res_u3_r8(s, 2, Register::H)", // 0x94
    "res_u3_r8(s, 2, Register::L)", // 0x95
    "res_u3_hladdr(s, 2)",          // 0x96
    "res_u3_r8(s, 2, Register::A)", // 0x97
    "res_u3_r8(s, 3, Register::B)", // 0x98
    "res_u3_r8(s, 3, Register::C)", // 0x99
    "res_u3_r8(s, 3, Register::D)", // 0x9A
    "res_u3_r8(s, 3, Register::E)", // 0x9B
    "res_u3_r8(s, 3, Register::H)", // 0x9C
    "res_u3_r8(s, 3, Register::L)", // 0x9D
    "res_u3_hladdr(s, 3)",          // 0x9E
    "res_u3_r8(s, 3, Register::A)", // 0x9F
    "res_u3_r8(s, 4, Register::B)", // 0xA0
    "res_u3_r8(s, 4, Register::C)", // 0xA1
    "res_u3_r8(s, 4, Register::D)", // 0xA2
    "res_u3_r8(s, 4, Register::E)", // 0xA3
    "res_u3_r8(s, 4, Register::H)", // 0xA4
    "res_u3_r8(s, 4, Register::L)", // 0xA5
    "res_u3_hladdr(s, 4)",          // 0xA6
    "res_u3_r8(s, 4, Register::A)", // 0xA7
    "res_u3_r8(s, 5, Register::B)", // 0xA8
    "res_u3_r8(s, 5, Register::C)", // 0xA9
    "res_u3_r8(s, 5, Register::D)", // 0xAA
    "res_u3_r8(s, 5, Register::E)", // 0xAB
    "res_u3_r8(s, 5, Register::H)", // 0xAC
    "res_u3_r8(s, 5, Register::L)", // 0xAD
    "res_u3_hladdr(s, 5)",          // 0xAE
    "res_u3_r8(s, 5, Register::A)", // 0xAF
    "res_u3_r8(s, 6, Register::B)", // 0xB0
    "res_u3_r8(s, 6, Register::C)", // 0xB1
    "res_u3_r8(s, 6, Register::D)", // 0xB2
    "res_u3_r8(s, 6, Register::E)", // 0xB3
    "res_u3_r8(s, 6, Register::H)", // 0xB4
    "res_u3_r8(s, 6, Register::L)", // 0xB5
    "res_u3_hladdr(s, 6)",          // 0xB6
    "res_u3_r8(s, 6, Register::A)", // 0xB7
    "res_u3_r8(s, 7, Register::B)", // 0xB8
    "res_u3_r8(s, 7, Register::C)", // 0xB9
    "res_u3_r8(s, 7, Register::D)", // 0xBA
    "res_u3_r8(s, 7, Register::E)", // 0xBB
    "res_u3_r8(s, 7, Register::H)", // 0xBC
    "res_u3_r8(s, 7, Register::L)", // 0xBD
    "res_u3_hladdr(s, 7)",          // 0xBE
    "res_u3_r8(s, 7, Register::A)", // 0xBF
    "set_u3_r8(s, 0, Register::B)", // 0xC0
    "set_u3_r8(s, 0, Register::C)", // 0xC1
    "set_u3_r8(s, 0, Register::D)", // 0xC2
    "set_u3_r8(s, 0, Register::E)", // 0xC3
    "set_u3_r8(s, 0, Register::H)", // 0xC4
    "set_u3_r8(s, 0, Register::L)", // 0xC5
    "set_u3_hladdr(s, 0)",          // 0xC6
    "set_u3_r8(s, 0, Register::A)", // 0xC7
    "set_u3_r8(s, 1, Register::B)", // 0xC8
    "set_u3_r8(s, 1, Register::C)", // 0xC9
    "set_u3_r8(s, 1, Register::D)", // 0xCA
    "set_u3_r8(s, 1, Register::E)", // 0xCB
    "set_u3_r8(s, 1, Register::H)", // 0xCC
    "set_u3_r8(s, 1, Register::L)", // 0xCD
    "set_u3_hladdr(s, 1)",          // 0xCE
    "set_u3_r8(s, 1, Register::A)", // 0xCF
    "set_u3_r8(s, 2, Register::B)", // 0xD0
    "set_u3_r8(s, 2, Register::C)", // 0xD1
    "set_u3_r8(s, 2, Register::D)", // 0xD2
    "set_u3_r8(s, 2, Register::E)", // 0xD3
    "set_u3_r8(s, 2, Register::H)", // 0xD4
    "set_u3_r8(s, 2, Register::L)", // 0xD5
    "set_u3_hladdr(s, 2)",          // 0xD6
    "set_u3_r8(s, 2, Register::A)", // 0xD7
    "set_u3_r8(s, 3, Register::B)", // 0xD8
    "set_u3_r8(s, 3, Register::C)", // 0xD9
    "set_u3_r8(s, 3, Register::D)", // 0xDA
    "set_u3_r8(s, 3, Register::E)", // 0xDB
    "set_u3_r8(s, 3, Register::H)", // 0xDC
    "set_u3_r8(s, 3, Register::L)", // 0xDD
    "set_u3_hladdr(s, 3)",          // 0xDE
    "set_u3_r8(s, 3, Register::A)", // 0xDF
    "set_u3_r8(s, 4, Register::B)", // 0xE0
    "set_u3_r8(s, 4, Register::C)", // 0xE1
    "set_u3_r8(s, 4, Register::D)", // 0xE2
    "set_u3_r8(s, 4, Register::E)", // 0xE3
    "set_u3_r8(s, 4, Register::H)", // 0xE4
    "set_u3_r8(s, 4, Register::L)", // 0xE5
    "set_u3_hladdr(s, 4)",          // 0xE6
    "set_u3_r8(s, 4, Register::A)", // 0xE7
    "set_u3_r8(s, 5, Register::B)", // 0xE8
    "set_u3_r8(s, 5, Register::C)", // 0xE9
    "set_u3_r8(s, 5, Register::D)", // 0xEA
    "set_u3_r8(s, 5, Register::E)", // 0xEB
    "set_u3_r8(s, 5, Register::H)", // 0xEC
    "set_u3_r8(s, 5, Register::L)", // 0xED
    "set_u3_hladdr(s, 5)",          // 0xEE
    "set_u3_r8(s, 5, Register::A)", // 0xEF
    "set_u3_r8(s, 6, Register::B)", // 0xF0
    "set_u3_r8(s, 6, Register::C)", // 0xF1
    "set_u3_r8(s, 6, Register::D)", // 0xF2
    "set_u3_r8(s, 6, Register::E)", // 0xF3
    "set_u3_r8(s, 6, Register::H)", // 0xF4
    "set_u3_r8(s, 6, Register::L)", // 0xF5
    "set_u3_hladdr(s, 6)",          // 0xF6
    "set_u3_r8(s, 6, Register::A)", // 0xF7
    "set_u3_r8(s, 7, Register::B)", // 0xF8
    "set_u3_r8(s, 7, Register::C)", // 0xF9
    "set_u3_r8(s, 7, Register::D)", // 0xFA
    "set_u3_r8(s, 7, Register::E)", // 0xFB
    "set_u3_r8(s, 7, Register::H)", // 0xFC
    "set_u3_r8(s, 7, Register::L)", // 0xFD
    "set_u3_hladdr(s, 7)",          // 0xFE
    "set_u3_r8(s, 7, Register::A)", // 0xFF
];

