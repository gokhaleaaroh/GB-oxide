use std::error::Error;
mod state;
mod instructions;


fn main() -> Result<(), Box<dyn Error>> {

    let cart = state::Cartridge::load_rom("roms/tetris.gb")?;

    for i in 0..10 {
	println!("{}th byte: {:02X}", i + 1, cart.rom[i]);
    }

    
    println!();

    Ok(())
}
