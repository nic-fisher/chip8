use std::ops::Add;

use crate::instruction::Instruction;

const DISPLAY_WIDTH: u8 = 64;
const DISPLAY_HEIGHT: u8 = 32;

pub struct CPU {
    memory: [u8; 4096],
    registers: [u8; 16],
    index: u16,
    pc: u16,
    stack: [u16; 16],
    sp: u8,
    delay_timer: u8,
    sound_timer: u8,
    keys: [bool; 16],
    pub display: [bool; DISPLAY_WIDTH as usize * DISPLAY_HEIGHT as usize],
}

impl CPU {
    pub fn new() -> CPU {
        let mut cpu = CPU {
            memory: [0; 4096],
            registers: [0; 16],
            index: 0,
            pc: 0x200,
            stack: [0; 16],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            keys: [false; 16],
            display: [false; 64 * 32],
        };

        cpu.load_fonts();
        cpu
    }

    fn load_fonts(&mut self) {
        // Fonts should be loaded from 0x50
        let fonts = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];

        for (i, font) in fonts.iter().enumerate() {
            self.memory[0x50 + i] = *font;
        }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        for (i, byte) in rom.iter().enumerate() {
            // println!("Loading into memory location: {:#02x}", 0x200 + i);
            self.memory[0x200 + i] = *byte;
        }
    }

    pub fn get_display_pixel_index(&self, x: usize, y: usize) -> usize {
        y as usize * DISPLAY_WIDTH as usize + x as usize
    }

    pub fn get_display_pixel(&self, index: usize) -> bool {
        self.display[index]
    }

    pub fn set_carry_flag(&mut self, value: u8) {
        self.registers[0xF] = value;
    }

    pub fn update_display_pixel(&mut self, index: usize, value: bool) {
        self.display[index] = value;
    }

    pub fn execute_instruction(&mut self) {
        let instruction_bytes = self.fetch_instruction_bytes();
        println!("Instruction: #{:#018b}", instruction_bytes);

        let instruction = Instruction::from_bytes(instruction_bytes);

        match instruction.op_code {
            0x00 => match instruction.nn {
                0xE0 => println!("Clear screen"),
                _ => println!("Instruction not implemented."),
            },

            0x01 => {
                // Jump to location nnn
                println!("Jump to location: {:#02x}", instruction.nnn);
                self.pc = instruction.nnn;
            }

            0x06 => {
                // Set register x to nn
                println!(
                    "Set register {:#02x} to: {:#02x}",
                    instruction.x, instruction.nn
                );
                self.registers[(instruction.x) as usize] = instruction.nn;
            }

            0x07 => {
                // Add value nn to register x
                println!(
                    "Add value {:#02x} to register {:#02x}",
                    instruction.nn, instruction.x
                );

                self.registers[(instruction.x) as usize] =
                    self.registers[(instruction.x) as usize].add(instruction.nn);
            }

            0x0A => {
                // Set index register to nnn
                println!("Set index register to nnn: {:#02x}", instruction.nnn);
                self.index = instruction.nnn;
            }

            0x0D => {
                // Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels.
                // The sprite memory location is stored in the index register. The display location is updated
                // with the result of the current pixel XOR'd with the new pixel value stored in the sprite.
                // The carry flag register (VF) is set to 1 if any screen pixels are flipped from set to unset when the
                // sprite is drawn.

                // Wrap position
                let x_position = self.registers[(instruction.x) as usize] % DISPLAY_WIDTH as u8;
                let y_position = self.registers[(instruction.y) as usize] % DISPLAY_HEIGHT as u8;
                self.set_carry_flag(0);
                let mut collision = false;

                for row in 0..instruction.n {
                    let sprite_byte = self.memory[self.index as usize + row as usize];
                    let y = y_position + row;

                    for col in 0..8 {
                        let x = x_position + col;
                        let current_pixel_index =
                            self.get_display_pixel_index(x as usize, y as usize);
                        let current_pixel = self.get_display_pixel(current_pixel_index);
                        // Example of the below:
                        // sprite_byte = 0011 1000
                        // compare_value = 1000 0000 (1 shifted to the left 7 - col)
                        // 0011 1000 & 1000 0000 = 0000 0000
                        let new_pixel = (sprite_byte & (1 << (7 - col))) != 0;
                        self.update_display_pixel(current_pixel_index, current_pixel ^ new_pixel);
                        collision = collision || (current_pixel && new_pixel);
                    }
                }

                if collision {
                    self.set_carry_flag(1);
                } else {
                    self.set_carry_flag(0);
                }
            }

            _ => println!("Instruction not implemented"),
        }
    }

    // An instruction is two bytes. A big-endian system stores the most significant byte of a word
    // at the smallest memory address and the least significant byte at the largest.
    fn fetch_instruction_bytes(&mut self) -> u16 {
        let instruction = (self.memory[self.pc as usize] as u16) << 8
            | (self.memory[self.pc as usize + 1] as u16);
        self.pc += 2;
        instruction
    }
}
