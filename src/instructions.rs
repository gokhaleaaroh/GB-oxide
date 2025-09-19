use crate::state;

// Load instructions
pub fn LD_r8_r8(game_state: &mut state::GameState, r1: state::Register, r2: state::Register) {
    let val = state::read(game_state, r1 as u16);
    state::Registers::update_register8(&mut game_state.gb.registers, &r2, val);
}
