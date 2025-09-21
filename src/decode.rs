use crate::state::{GameState, Register};
use crate::instructions;

type InstructionWrapper = fn(&mut GameState);

