use std::error::Error;
mod cpu;
mod instructions;
mod state;
use state::GameState;
use cpu::CPU;

fn main() -> Result<(), Box<dyn Error>> {
    // let cart = state::Cartridge::load_rom("roms/tetris.gb")?;
    let mut new_game = GameState::start_game("/home/aarohg/Projects/my-emulator/roms/tetris.gb")?;
    let cpu = CPU::initialize(); 
    
    cpu.main_loop(&mut new_game);

    Ok(())
}
