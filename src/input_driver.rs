
extern crate sdl2;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub struct Input {
    event_pump: sdl2::EventPump,
}

impl Input {

    pub fn new(context: &sdl2::Sdl) -> Result<Self, String> {
        let event_pump = context.event_pump()?;
        Ok(
        Input{
            event_pump: event_pump,
        })
    }

    pub fn upload_keys(&mut self, keys : &mut [bool ; 16]) -> bool {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return true,
                Event::KeyDown { keycode: Some(key), ..} => {
                    if let Some(k) = key2btn(key) {
                        keys[k] = true;
                    }
                },
                Event::KeyUp{keycode: Some(key), ..} => {
                    if let Some(k) = key2btn(key) {
                        keys[k] = false;
                    }
                }
                _ => {}
            }
        }
        false
    }
}

fn key2btn(key: Keycode) -> Option<usize> {
    match key {
        Keycode::Num1 =>    Some(0x1),
        Keycode::Num2 =>    Some(0x2),
        Keycode::Num3 =>    Some(0x3),
        Keycode::Num4 =>    Some(0xC),
        Keycode::Q =>       Some(0x4),
        Keycode::W =>       Some(0x5),
        Keycode::E =>       Some(0x6),
        Keycode::R =>       Some(0xD),
        Keycode::A =>       Some(0x7),
        Keycode::S =>       Some(0x8),
        Keycode::D =>       Some(0x9),
        Keycode::F =>       Some(0xE),
        Keycode::Z =>       Some(0xA),
        Keycode::X =>       Some(0x0),
        Keycode::C =>       Some(0xB),
        Keycode::V =>       Some(0xF),
        _ =>                None,
    }
}