
mod chip8;
mod font;
mod graphic_driver;
mod input_driver;

use chip8::Chip8;
use std::env;
use std::thread::sleep;
use std::time::Duration;
use std::fs::File;
use std::io::Read;

pub const CHIP8_WINDOW_WIDTH:   usize = 64;
pub const CHIP8_WINDOW_HEIGHT:  usize = 32;
pub const CHIP8_PIXEL_SIZE:     usize = 20;
pub const CHIP8_RAM_SIZE:       usize = 4096;
pub const CHIP8_REGISTERS_SIZE: usize = 16;


fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        return Err("<Usage: ./chip8 rom_path>".to_string());
    }

    let mut rom = File::open(&args[1]).expect("Unable to open file");
    let mut buffer = Vec::new();

    rom.read_to_end(&mut buffer).unwrap();

    let mut chip8_emulator = Chip8::new();
    chip8_emulator.load(&buffer);

    let context = sdl2::init()?;

    let mut graphics = graphic_driver::Graphics::new(&context)?;
    let mut input = input_driver::Input::new(&context)?;

    'running: loop {
        chip8_emulator.tick()?;

        if chip8_emulator.draw_flag {
            graphics.draw(&chip8_emulator.gfx);
            chip8_emulator.draw_flag = false;
            sleep(Duration::new(0, 1_000_000_000u32/60));
        }

        if input.upload_keys(&mut chip8_emulator.key) {
            break 'running;
        }
        
    }

    Ok(())
}
