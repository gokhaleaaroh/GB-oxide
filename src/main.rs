use std::error::Error;
mod cpu;
mod instructions;
mod state;
mod ppu;
use state::GameState;
use cpu::CPU;
use minifb::{Key, Window, WindowOptions};

fn main() -> Result<(), Box<dyn Error>> {
    // let cart = state::Cartridge::load_rom("roms/tetris.gb")?;
    // let mut new_game = GameState::start_game("/home/aarohg/Projects/my-emulator/roms/tetris.gb")?;
    // let cpu = CPU::initialize(); 
    // 
    // cpu.main_loop(&mut new_game);

    let mut buffer: Vec<u32> = vec![0; 144*160];

    let mut window = Window::new(
	"Test - ESC to exit",
	160,
	144,
	WindowOptions::default(),
    )
	.unwrap_or_else(|e| {
	    panic!("{}", e);
	});

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
	for i in buffer.iter_mut() {
	    *i = 0x32a852; // write something more funny here!
        }
	
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
	window
	    .update_with_buffer(&buffer, 160, 144)
	    .unwrap();
    }

    Ok(())
}
