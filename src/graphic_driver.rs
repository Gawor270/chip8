use sdl2::rect::Rect;

extern crate sdl2;
pub struct Graphics {
    canvas: sdl2::render::Canvas<sdl2::video::Window>,
}

impl Graphics {
    pub fn new(context: &sdl2::Sdl) -> Result<Self, String> {
        let video_subsystem = context.video()?;
        let window_height = (crate::CHIP8_WINDOW_HEIGHT as u32) * (crate::CHIP8_PIXEL_SIZE as u32);
        let window_width = (crate::CHIP8_WINDOW_WIDTH as u32) * (crate::CHIP8_PIXEL_SIZE as u32);
    
        let window = video_subsystem
            .window("Chip-8 Emulator", window_width, window_height)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;
        
        let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    
        canvas.set_draw_color(sdl2::pixels::Color::BLACK);
        canvas.clear();
        canvas.present();
    
        Ok(Graphics{
            canvas : canvas,
        })
    }

    pub fn draw(&mut self, gfx : &[u8 ; crate::CHIP8_WINDOW_WIDTH * crate::CHIP8_WINDOW_HEIGHT]) {
        self.canvas.clear();
        for i in 0..crate::CHIP8_WINDOW_HEIGHT {
            for j in 0..crate::CHIP8_WINDOW_WIDTH {
                match gfx[i* crate::CHIP8_WINDOW_WIDTH + j] {
                    0 => self.canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0x64, 0)),
                    _ => self.canvas.set_draw_color(sdl2::pixels::Color::RGB(0, 0x0a, 0)),
                };
                self.canvas.fill_rect(Rect::new((j*crate::CHIP8_PIXEL_SIZE) as i32, (i*crate::CHIP8_PIXEL_SIZE) as i32, crate::CHIP8_PIXEL_SIZE as u32, crate::CHIP8_PIXEL_SIZE as u32)).unwrap();
            }
        }
        self.canvas.present();
    }

}

