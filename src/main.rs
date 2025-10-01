use std::error::Error;
mod constants;
mod cpu;
mod instructions;
mod logger;
mod ppu;
mod state;
use cpu::CPU;
use minifb::{Key, Window, WindowOptions};
use ppu::PPU;
use state::GameState;

fn main() -> Result<(), Box<dyn Error>> {
    // let cart = state::Cartridge::load_rom("roms/tetris.gb")?;
    let mut game_state =
        GameState::start_game("/home/aarohg/Projects/my-emulator/roms/tetris.gb")?;
    let cpu = CPU::initialize();
    let mut ppu = PPU::initialize();

    let mut window = Window::new("Test - ESC to exit", 160, 144, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    // for i in 0..5000 {
    // 	println!("OP 0x{:04X}: 0x{:02X}", 0x0100 + i, game_state.read(0x0100 + i as u16));
    // }

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let cycles = cpu.step(&mut game_state);
        let update = ppu.step(4 * cycles, &mut game_state);
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        if update {
            window
                .update_with_buffer(&ppu.current_fb, 160, 144)
                .unwrap();
        }
    }

    Ok(())
}
