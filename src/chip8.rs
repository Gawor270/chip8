

use crate::font::DIGITS;
use crate::CHIP8_WINDOW_WIDTH;
use crate::CHIP8_WINDOW_HEIGHT;
use crate::CHIP8_RAM_SIZE;
use crate::CHIP8_REGISTERS_SIZE;

use rand::Rng;

pub struct Chip8 {
    memory: [u8; CHIP8_RAM_SIZE],
    v: [u8; CHIP8_REGISTERS_SIZE],
    i: usize,
    pc: usize,
    pub gfx: [u8; CHIP8_WINDOW_WIDTH * CHIP8_WINDOW_HEIGHT],
    delay_timer: u8,
    sound_timer: u8,
    stack: [usize; 16],
    sp: usize,
    pub key: [bool; 16],
    pub draw_flag: bool,
}

impl Chip8 {
    pub fn new() -> Self {
        Chip8{
            memory : [0; CHIP8_RAM_SIZE],
            v : [0; CHIP8_REGISTERS_SIZE],
            i : 0x0,
            pc : 0x200,
            sp : 0x0,
            delay_timer : 0,
            sound_timer : 0,
            stack : [0; 16],
            key : [false; 16],
            gfx : [0; CHIP8_WINDOW_WIDTH * CHIP8_WINDOW_HEIGHT],
            draw_flag: false,
        }
    }

    pub fn load(&mut self, program : &[u8]) {
        // load fonts
        for (i, byte) in DIGITS.iter().enumerate() {
            self.memory[i] = *byte;
        }

        // load program 
        self.memory[(0x200)..(0x200 + program.len())].copy_from_slice(program);
    }

    pub fn tick(&mut self) -> Result<(), String> {
        let opcode  =  (self.memory[self.pc] as u16) << 8 | (self.memory[self.pc+1] as u16);
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;
        let kk = (opcode & 0x00FF) as u8;

        let digit1 = (opcode & 0xF000) >> 12;
        let digit4 = opcode & 0x000F;

        self.pc += 2;
        // println!("{:x}", opcode);
        // println!("{:?}", self.key);

        match (digit1, x, y, digit4)  {
            (0,0,0,0) => return Ok(()),

            (0,0,0xE,0) =>{
                self.gfx.fill(0);
            },

            (0,0,0xE,0xE) => {
                self.sp -= 1;
                self.pc = self.stack[self.sp];
            },

            (1,_,_,_) => {
                self.pc = (opcode & 0x0FFF) as usize;
            },

            (2,_,_,_) => {
                self.stack[self.sp] = self.pc;
                self.sp +=1;
                self.pc = (opcode & 0x0FFF) as usize;
            },

            (3,_,_,_) => {
                if self.v[x] == kk {
                    self.pc += 2
                }

            },

            (4,_,_,_) => {
                if self.v[x] != kk {
                    self.pc += 2;
                }
            },

            (5,_,_,0) => {
                if self.v[x] == self.v[y] {
                    self.pc += 2;
                }
            },

            (6,_,_,_) => {
                self.v[x] = kk;
            },

            (7,_,_,_) => {
                self.v[x] = self.v[x].wrapping_add(kk);
            },

            (8,_,_,0) => {
                self.v[x] = self.v[y];
            },

            (8,_,_,1) => {
                self.v[x] |= self.v[y];
            },

            (8,_,_,2) => {
                self.v[x] &= self.v[y];
            },

            (8,_,_,3) => {
                self.v[x] ^= self.v[y];
            },

            (8,_,_,4) => {
                let (new_vx, carry) = self.v[x].overflowing_add(self.v[y]);
                let new_vf = if carry {1} else {0};

                self.v[x] = new_vx;
                self.v[0xF] = new_vf;
            },

            (8,_,_,5) => {
                let (new_vx, borrow) = self.v[x].overflowing_sub(self.v[y]);
                let new_vf = if borrow { 0 } else { 1 };

                self.v[x] = new_vx;
                self.v[0xF] = new_vf;
            },

            (8,_,_,6) => {
                self.v[0xF] = self.v[x] & 1;
                self.v[x] >>= 1;
            },

            (8,_,_,7) => {
                let (new_vx, borrow) = self.v[y].overflowing_sub(self.v[x]);
                let new_vf = if borrow { 0 } else { 1 };

                self.v[x] = new_vx;
                self.v[0xF] = new_vf;
            },

            (8,_,_,0xE) => {
                self.v[0xF] = (self.v[x] >> 7) & 1;
                self.v[x] <<= 1;
            },

            
            (9,_,_,0) => {
                if self.v[x] != self.v[y] {
                    self.pc += 2;
                }
            },

            (0xA,_,_,_) => {
                self.i = (opcode & 0x0FFF) as usize;
            },

            (0xB,_,_,_) => {
                self.pc = ((opcode & 0x0FFF) as usize) + self.v[0] as usize;
            },

            (0xC,_,_,_) => {
                self.v[x] = rand::thread_rng().gen::<u8>() & kk;
            },

            (0xD,_,_,_) => {
                let n = ((opcode & 0x000F)) as usize;
                self.draw_flag = true;

                self.v[0xF] = 0;

                for byte in 0..n {
                    let nx = (self.v[y] as usize + byte)%crate::CHIP8_WINDOW_HEIGHT;
                    for bit in 0..8 {
                        let ny = (self.v[x] as usize + bit)%crate::CHIP8_WINDOW_WIDTH;
                        let bit_val = (self.memory[self.i + byte] >> (7-bit))&1;

                        if bit_val == 1 && self.gfx[nx * CHIP8_WINDOW_WIDTH + ny] == 1 {
                            self.v[0xF] = 1;
                        }

                        self.gfx[nx * CHIP8_WINDOW_WIDTH + ny] ^= bit_val;
                    }
                }
            },

            (0xE,_,9,0xE) => {
                if self.key[self.v[x] as usize] { 
                    self.pc += 2;
                }
            },

            (0xE,_,0xA,1) => {
                if !self.key[self.v[x] as usize] {
                    self.pc += 2;
                }
            },

            (0xF,_,0,7) => {
                self.v[x] = self.delay_timer;
            },

            (0xF,_,0,0xA) => {
                let mut pressed = false;
                for i in 0..self.key.len() {
                    if self.key[i] {
                        self.v[x] = i as u8;
                        pressed = true;
                        break;
                    }
                }

                if !pressed {
                    // Redo opcode
                    self.pc -= 2;
                }
            },

            (0xF,_,1,5) => {
                self.delay_timer = self.v[x];
            },

            (0xF,_,1,8) => {
                self.sound_timer = self.v[x];
            },

            (0xF,_,1,0xE) => {
                let vx = self.v[x] as usize;
                self.i = self.i.wrapping_add(vx) as usize;
            },

            (0xF,_,2,9) => {
                self.i = 5*self.v[x] as usize;
            },

            (0xF,_,3,3) => {
                let d = self.v[x]%10 as u8;
                let t = (self.v[x]%100)/10 as u8;
                let h = (self.v[x])/100 as u8;

                self.memory[self.i] = h;
                self.memory[self.i+1] = t;
                self.memory[self.i+2] = d;
            },

            (0xF,_,5,5) => {
                for j in 0..=x {
                    self.memory[self.i + j] = self.v[j];
                }
            },

            (0xF,_,6,5) => {
                for j in 0..=x {
                    self.v[j] = self.memory[self.i + j];
                }
            }

            _ => return Err("Unknown code {:x}".to_string()),
        }

        self.update_timers();
        Ok(())
    }

    fn update_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                println!("beep");
            }
            self.sound_timer -= 1;
        }
    }
}

