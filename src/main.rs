use std::error::Error;
mod cpu;
mod instructions;
mod state;

fn main() -> Result<(), Box<dyn Error>> {
    // let cart = state::Cartridge::load_rom("roms/tetris.gb")?;
    let lsb: u8 = 0x80;
    let msb: u8 = 0x3F;
    let val = ((msb as u16) << 8) | (lsb as u16);

    println!("{:02X}", val);

    Ok(())
}
