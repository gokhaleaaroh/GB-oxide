use crate::state;














// Load instructions
pub fn ld_r8_r8(game_state: &mut state::GameState, r1: state::Register, r2: state::Register) {
    game_state.update_register8(r1, game_state.get_register8(r2));
}
