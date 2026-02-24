use std::error::Error;
mod constants;
mod cpu;
mod instructions;
mod logger;
mod ppu;
mod state;
use cpu::CPU;
use minifb::{Key, Scale, Window, WindowOptions};
use ppu::PPU;
use state::GameState;

fn main() -> Result<(), Box<dyn Error>> {
    // let cart = state::Cartridge::load_rom("roms/tetris.gb")?;
    let mut game_state =
        GameState::start_game("/home/aarohg/Projects/my-emulator/roms/pkmn-red.gb")?;
    let cpu = CPU::initialize();
    let mut ppu = PPU::initialize();

    let mut window = Window::new(
        "Test - ESC to exit",
        160,
        144,
        WindowOptions {
            scale: Scale::X2,
            ..Default::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    // for i in 0..5000 {
    // 	println!("OP 0x{:04X}: 0x{:02X}", 0x0100 + i, game_state.read(0x0100 + i as u16));
    // }

    let mut print_pc = false;
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let (mut a, mut b, mut start, mut select, mut down, mut up, mut left, mut right) =
            (false, false, false, false, false, false, false, false);

        // Arrow Keys
        if window.is_key_down(Key::Down) {
            down = true;
        }
        if window.is_key_down(Key::Up) {
            up = true;
        }
        if window.is_key_down(Key::Left) {
            left = true;
        }
        if window.is_key_down(Key::Right) {
            right = true;
        }

        if window.is_key_down(Key::R) {
            game_state.toggle_window = !game_state.toggle_window;
        }

        // Start, Select, B, A
        if window.is_key_down(Key::S) {
            select = true;
            // print_pc = !print_pc;
            // game_state.print_vals = true;
        }
        if window.is_key_down(Key::A) {
            start = true;
        }
        if window.is_key_down(Key::X) {
            b = true;
        }

        if window.is_key_down(Key::Z) {
            a = true;
        }

        game_state.update_joypad(a, b, start, select, up, down, left, right);

<<<<<<< HEAD
        let cycles = cpu.step(&mut game_state);
        let pos = game_state.get_register16(state::Register::PC);

        if print_pc {

            // println!(
            //     "LY: {}, LCDC: {}, IF: {}, IE: {}, IME: {}",
            //     game_state.get_ly(),
            //     game_state.get_lcdc(),
            //     game_state.get_i_flag(),
            //     game_state.get_i_enable(),
            //     game_state.get_interrupts()
            // );
        }
=======
        let cycles = cpu.step(&mut game_state) as u16;
>>>>>>> main
        let update = ppu.step(cycles, &mut game_state);
        // We unwrap here as we want this code to exit if it fails. Real applications may want to handle this in a different way
        if update {
            // game_state.print_oam();
            window
                .update_with_buffer(&ppu.current_fb, 160, 144)
                .unwrap();
        }
    }

    Ok(())
}
