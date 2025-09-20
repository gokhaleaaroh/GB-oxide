use std::error::Error;
mod state;
mod instructions;


fn main() -> Result<(), Box<dyn Error>> {

    let cart = state::Cartridge::load_rom("roms/tetris.gb")?;

    println!();

    Ok(())
}
