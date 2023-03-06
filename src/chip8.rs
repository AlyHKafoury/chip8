use rand::prelude::*;
use std::collections::HashMap;
use std::io::Read;
use std::io::BufReader;
use std::fs::File;

pub struct Chip8 {
    opcode: u16,
    memory: [u8; 4096],
    v: [u8; 16],
    index: u16,
    pc: u16,
    gfx: [u8; 2048],
    delay_timer: u16,
    sound_timer: u16,
    stack: [u16; 16],
    sp: u16,
    key: [u8; 16],
    draw_flag: bool,
    activations: Vec<u16>,
}

impl Chip8 {
    pub fn new() -> Self {
        Self {
            opcode: 0,
            memory: [0; 4096],
            v: [0; 16],
            index: 0,
            pc: 0,
            gfx: [0; 2048],
            delay_timer: 0,
            sound_timer: 0,
            stack: [0; 16],
            sp: 0,
            key: [0; 16],
            draw_flag: false,
            activations: Vec::<u16>::new(),
        }
    }

    pub(crate) fn init(&mut self) -> () {
        self.pc = 0x200;
        let font = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, //0
            0x20, 0x60, 0x20, 0x20, 0x70, //1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, //2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, //3
            0x90, 0x90, 0xF0, 0x10, 0x10, //4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, //5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, //6
            0xF0, 0x10, 0x20, 0x40, 0x40, //7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, //8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, //9
            0xF0, 0x90, 0xF0, 0x90, 0x90, //A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, //B
            0xF0, 0x80, 0x80, 0x80, 0xF0, //C
            0xE0, 0x90, 0x90, 0x90, 0xE0, //D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, //E
            0xF0, 0x80, 0xF0, 0x80, 0x80, //F
        ];
        for i in 0..font.len() {
            self.memory[i] = font[i];
        }
    }

    pub(crate) fn load_game(&mut self, filename: &str) -> () {
        let f = File::open(filename).unwrap();
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();

        reader.read_to_end(&mut buffer).unwrap();

        for i in 0..buffer.len() {
            self.memory[i + 512] = buffer[i];
        }
    }

    pub(crate) fn set_keys(&self) -> () {}

    pub(crate) fn emulate(&mut self) -> () {
        self.opcode = u16::from(self.memory[self.pc as usize]) << 8
            | u16::from(self.memory[(self.pc + 1) as usize]);
        self.activations.push(self.opcode);
        match self.opcode & 0xF000 {
            0x0000 => match self.opcode & 0x000F {
                0x0000 => {
                    self.gfx = [0; 2048];
                    self.draw_flag = true;
                    self.pc += 2;
                }
                0x000E => {
                    self.sp -= 1;
                    self.pc = self.stack[self.sp as usize];
                    self.pc += 2;
                }
                _ => println!("Invalid OpCode"),
            },
            0x1000 => self.pc = self.opcode & 0x0FFF,
            0x2000 => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = self.opcode & 0x0FFF;
            }
            0x3000 => {
                println!("{}",self.v[((self.opcode & 0x0F00) >> 8) as usize] == (self.opcode & 0x00FF) as u8 );
                if self.v[((self.opcode & 0x0F00) >> 8) as usize] == (self.opcode & 0x00FF) as u8 {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0x4000 => {
                if self.v[((self.opcode & 0x0F00) >> 8) as usize] != (self.opcode & 0x00FF) as u8 {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0x5000 => {
                if self.v[((self.opcode & 0x0F00) >> 8) as usize]
                    == self.v[((self.opcode & 0x00F0) >> 4) as usize]
                {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0x6000 => {
                self.v[((self.opcode & 0x0F00) >> 8) as usize] = (self.opcode & 0x00FF) as u8;
                self.pc += 2;
            }
            0x7000 => {
                self.v[((self.opcode & 0x0F00) >> 8) as usize] += (self.opcode & 0x00FF) as u8;
                self.pc += 2;
            }
            0x8000 => match self.opcode & 0x000F {
                0x0000 => {
                    self.v[usize::from((self.opcode & 0x0F00) >> 8)] =
                        self.v[usize::from((self.opcode & 0x00F0) >> 4)];
                    self.pc += 2;
                }
                0x0001 => {
                    self.v[usize::from((self.opcode & 0x0F00) >> 8)] |=
                        self.v[usize::from((self.opcode & 0x00F0) >> 4)];
                    self.pc += 2;
                }
                0x0002 => {
                    self.v[usize::from((self.opcode & 0x0F00) >> 8)] &=
                        self.v[usize::from((self.opcode & 0x00F0) >> 4)];
                    self.pc += 2;
                }
                0x0003 => {
                    self.v[usize::from((self.opcode & 0x0F00) >> 8)] ^=
                        self.v[usize::from((self.opcode & 0x00F0) >> 4)];
                    self.pc += 2;
                }
                0x0004 => {
                    if self.v[((self.opcode & 0x00F0) >> 4) as usize]
                        > (0xFF - self.v[((self.opcode & 0x0F00) >> 8) as usize])
                    {
                        self.v[0xF] = 1;
                    } else {
                        self.v[0xF] = 0;
                    }
                    self.v[((self.opcode & 0x0F00) >> 8) as usize] +=
                        self.v[((self.opcode & 0x00F0) >> 4) as usize];
                    self.pc += 2;
                }
                0x0005 => {
                    if self.v[((self.opcode & 0x00F0) >> 4) as usize]
                        > self.v[((self.opcode & 0x0F00) >> 8) as usize]
                    {
                        self.v[0xF] = 0;
                    } else {
                        self.v[0xF] = 1;
                    }
                    self.v[((self.opcode & 0x0F00) >> 8) as usize] -=
                        self.v[((self.opcode & 0x00F0) >> 4) as usize];
                    self.pc += 2;
                }
                0x0006 => {
                    self.v[0xF] = self.v[usize::from((self.opcode & 0x0F00) >> 8)] & 0x1;
                    self.v[usize::from((self.opcode & 0x0F00) >> 8)] >>= 1;
                    self.pc += 2;
                }
                0x0007 => {
                    if self.v[usize::from((self.opcode & 0x0F00) >> 8)]
                        > self.v[usize::from((self.opcode & 0x00F0) >> 4)]
                    {
                        self.v[0xF] = 0;
                    } else {
                        self.v[0xF] = 1;
                    }
                    self.v[((self.opcode & 0x0F00) >> 8) as usize] = self.v
                        [((self.opcode & 0x00F0) >> 4) as usize]
                        - self.v[((self.opcode & 0x0F00) >> 8) as usize];
                    self.pc += 2;
                }
                0x000E => {
                    self.v[0xF] = self.v[((self.opcode & 0x0F00) >> 8) as usize] >> 7;
                    self.v[((self.opcode & 0x0F00) >> 8) as usize] <<= 1;
                    self.pc += 2;
                }
                _ => println!("Invalid OpCode"),
            },
            0x9000 => {
                if self.v[usize::from((self.opcode & 0x0F00) >> 8)]
                    != self.v[usize::from((self.opcode & 0x00F0) >> 4)]
                {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }
            0xA000 => {
                self.index = self.opcode & 0x0FFF;
                self.pc += 2;
            }
            0xB000 => {
                self.pc = (self.opcode & 0x0FFF) + self.v[0] as u16;
            }
            0xC000 => {
                self.v[usize::from((self.opcode & 0x0F00) >> 8)] =
                    ((random::<u16>() % 0xFF) & (self.opcode & 0x00FF)) as u8;
                self.pc += 2;
            }
            0xD000 => {
                let x = self.v[usize::from(self.opcode & 0x0F00) >> 8] as u16;
                let y = self.v[usize::from(self.opcode & 0x00F0) >> 4] as u16;
                let height = self.opcode & 0x000F;
                let mut pixel: u8;

                self.v[0xF] = 0;
                for yline in 0..height {
                    pixel = self.memory[usize::from(self.index + yline)];
                    for xline in 0..8 {
                        if (pixel & (0x80 >> xline)) != 0 {
                            if self.gfx[usize::from(x + xline + ((y + yline) * 64))] == 1 {
                                self.v[0xF] = 1;
                            }
                            self.gfx[usize::from(x + xline + ((y + yline) * 64))] ^= 1;
                        }
                    }
                }
                self.draw_flag = true;
                self.pc += 2;
            }
            0xE000 => match self.opcode & 0x00FF {
                0x009E => {
                    if self.key[self.v[usize::from((self.opcode & 0x0F00) >> 8)] as usize] != 0 {
                        self.pc += 4;
                    } else {
                        self.pc += 2;
                    }
                }
                0x00A1 => {
                    if self.key[self.v[usize::from((self.opcode & 0x0F00) >> 8)] as usize] == 0 {
                        self.pc += 4;
                    } else {
                        self.pc += 2;
                    }
                }
                _ => print!("Invalid OpCode"),
            },
            0xF000 => {
                match self.opcode & 0x00FF {
                    0x0007 => {
                        self.v[usize::from(self.opcode & 0x0F00) >> 8] = self.delay_timer as u8;
                        self.pc += 2;
                    }
                    0x000A => {
                        let mut key_press = false;

                        for i in 0..16 {
                            if self.key[i] != 0 {
                                self.v[usize::from(self.opcode & 0x0F00) >> 8] = i as u8;
                                key_press = true;
                            }
                        }

                        // If we didn't received a keypress, skip this cycle and try again.
                        if !key_press {
                            return;
                        }
                        self.pc += 2;
                    }
                    0x0015 => {
                        self.delay_timer = self.v[usize::from(self.opcode & 0x0F00) >> 8] as u16;
                        self.pc += 2;
                    }
                    0x0018 => {
                        self.sound_timer = self.v[usize::from(self.opcode & 0x0F00) >> 8] as u16;
                        self.pc += 2;
                    }
                    0x001E => {
                        if self.index + self.v[usize::from(self.opcode & 0x0F00) >> 8] as u16
                            > 0xFFF
                        {
                            self.v[0xF] = 1;
                        } else {
                            self.v[0xF] = 0;
                        }

                        self.index += self.v[usize::from(self.opcode & 0x0F00) >> 8] as u16;
                        self.pc += 2;
                    }
                    0x0029 => {
                        self.index = self.v[usize::from(self.opcode & 0x0F00) >> 8] as u16 * 0x5;
                        self.pc += 2;
                    }
                    0x0033 => {
                        self.memory[self.index as usize] =
                            self.v[usize::from(self.opcode & 0x0F00) >> 8] / 100;
                        self.memory[self.index as usize + 1] =
                            (self.v[usize::from(self.opcode & 0x0F00) >> 8] / 10) % 10;
                        self.memory[self.index as usize + 2] =
                            (self.v[usize::from(self.opcode & 0x0F00) >> 8] % 100) % 10;
                        self.pc += 2;
                    }
                    0x0055 => {
                        for i in 0..=((self.opcode & 0x0F00) >> 8) {
                            self.memory[(self.index + i) as usize] = self.v[i as usize];
                        }

                        self.index += ((self.opcode & 0x0F00) >> 8) as u16 + 1;
                        self.pc += 2;
                    }
                    0x0065 => {
                        for i in 0..=((self.opcode & 0x0F00) >> 8) {
                            self.v[i as usize] = self.memory[(self.index + i) as usize];
                        }

                        self.index += ((self.opcode & 0x0F00) >> 8) as u16 + 1;
                        self.pc += 2;
                    }
                    _ => print!("Invalid OpCode"),
                }
            }
            _ => print!("Invalid OpCode"),
        }

        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                println!("BEEP!\n");
            }
            self.sound_timer -= 1;
        }
    }

    pub fn draw_flag(&self) -> bool {
        self.draw_flag
    }

    fn debug_memory(&self) {
        for x in self.stack.clone() {
            println!("{:#x}", x);
        }
        for x in self.activations.clone() {
            println!("{:#x}", x);
        }
    }

    pub fn debug_render(&self) {
        for y in 0..32
        {
            for x in 0..64
            {
                if self.gfx[(y*64) + x] == 0 {
                    print!("O");
                }
                else {
                    print!(" ");
                }

            }
            println!("");
        }
        println!("");
    }
}
