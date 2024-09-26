use crate::instruction::Instruction;
use rand::Rng;

const DISPLAY_WIDTH: u8 = 64;
const DISPLAY_HEIGHT: u8 = 32;
const FONTSET_START_ADDRESS: usize = 0x50;

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
            self.memory[FONTSET_START_ADDRESS + i] = *font;
        }
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        for (i, byte) in rom.iter().enumerate() {
            self.memory[0x200 + i] = *byte;
        }
    }

    pub fn key_press(&mut self, key_index: usize) {
        self.keys[key_index] = true;
    }

    pub fn key_release(&mut self, key_index: usize) {
        self.keys[key_index] = false;
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

    // CHIP-8 has two timers. They bouth count down at 60 hertz, until they reach 0.
    // Delay timer: This timer is intended to be used for timing events of games. Its value can be set and read.
    // Sound timer: This timer is used for sound effects. When its value is nonzero, a beeping sound is made. Its value can only be set.
    pub fn decrement_timers(&mut self) {
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }
        if self.sound_timer > 0 {
            self.sound_timer -= 1;
        }
    }

    pub fn execute_instruction(&mut self) {
        let instruction_bytes = self.fetch_instruction_bytes();
        // println!("Instruction: #{:#018b}", instruction_bytes);

        let instruction = Instruction::from_bytes(instruction_bytes);

        match instruction.op_code {
            0x00 => match instruction.nn {
                0xE0 => self.display = [false; 64 * 32], // Clear screen
                0xEE => {
                    // Return from a subroutine
                    self.sp -= 1;
                    self.pc = self.stack[self.sp as usize];
                }
                _ => println!("Instruction not implemented."),
            },

            0x01 => {
                // Jump to location nnn
                self.pc = instruction.nnn;
            }

            0x02 => {
                // Call subroutine at nnn
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = instruction.nnn;
            }

            0x03 => {
                // Skip next instruction if Vx == nn
                if self.registers[instruction.x] == instruction.nn {
                    self.pc += 2;
                }
            }

            0x04 => {
                // Skip next instruction if Vx != nn
                if self.registers[instruction.x] != instruction.nn {
                    self.pc += 2;
                }
            }

            0x05 => {
                // Skip next instruction if Vx == Vy
                if self.registers[instruction.x] == self.registers[instruction.y] {
                    self.pc += 2;
                }
            }

            0x06 => {
                // Set register x to nn
                self.registers[instruction.x] = instruction.nn;
            }

            0x07 => {
                // Add value nn to register x
                self.registers[instruction.x] =
                    self.registers[instruction.x].wrapping_add(instruction.nn);
            }

            0x08 => {
                match instruction.n {
                    0x00 => {
                        // Set register x to the value of register y
                        self.registers[instruction.x] = self.registers[instruction.y]
                    }
                    0x01 => {
                        // Set register x to the value of register x OR register y
                        self.registers[instruction.x] =
                            self.registers[instruction.x] | self.registers[instruction.y];
                    }
                    0x02 => {
                        // Set register x to the value of register x AND register y
                        self.registers[instruction.x] =
                            self.registers[instruction.x] & self.registers[instruction.y];
                    }
                    0x03 => {
                        // Set register x to the value of register x XOR register y
                        self.registers[instruction.x] =
                            self.registers[instruction.x] ^ self.registers[instruction.y];
                    }
                    0x04 => {
                        // Set register x to the value of register x PLUS register y
                        // If the result is greater than 8 bits (i.e., > 255,) the carry register is set to 1, otherwise 0
                        let (sum, overflow) = self.registers[instruction.x]
                            .overflowing_add(self.registers[instruction.y]);
                        self.registers[instruction.x] = sum;
                        self.set_carry_flag(overflow as u8)
                    }
                    0x05 => {
                        // Set register x to the value of register x minus register y
                        // If the register x is greater than register y set the carry register to 1, otherwise 0.
                        let (diff, overflow) = self.registers[instruction.x]
                            .overflowing_sub(self.registers[instruction.y]);
                        self.registers[instruction.x] = diff;
                        self.set_carry_flag(!overflow as u8)
                    }
                    0x06 => {
                        // This uses the newer implementation
                        // Set VX to the value of VY (older configuration, not implemented)
                        // Shift the value of VX one bit to the right (8XY6) or left (8XYE)
                        // Set VF to 1 if the bit that was shifted out was 1, or 0 if it was 0
                        let shifted_bit = self.registers[instruction.x] & 0b00000001;
                        self.registers[instruction.x] = self.registers[instruction.x] >> 1;
                        self.set_carry_flag(shifted_bit);
                    }
                    0x07 => {
                        // Set register x to the value of register y minus register x
                        // If the register y is greater than register x set the carry register to 1, otherwise 0.
                        let (diff, overflow) = self.registers[instruction.y]
                            .overflowing_sub(self.registers[instruction.x]);
                        self.registers[instruction.x] = diff;
                        self.set_carry_flag(!overflow as u8)
                    }
                    0x0E => {
                        // This uses the newer implementation
                        // Set VX to the value of VY (older configuration, not implemented)
                        // Shift the value of VX one bit to the left (8XYE)
                        // Set VF to 1 if the bit that was shifted out was 1, or 0 if it was 0
                        let shifted_bit: u8 = (self.registers[instruction.x] & 0b10000000) << 7;
                        self.registers[instruction.x] = self.registers[instruction.x] << 1;
                        self.set_carry_flag(shifted_bit);
                    }
                    _ => panic!("Unimplemented OP code"),
                }
            }

            0x09 => {
                // Skip next instruction if Vx != Vy
                if self.registers[instruction.x] != self.registers[instruction.y] {
                    self.pc += 2;
                }
            }

            0x0A => {
                // Set index register to nnn
                self.index = instruction.nnn;
            }

            0x0B => {
                // Set PC to nnn plus register[0]. This is the original implementation.
                // The newer implementation sets the PC nnn + registers[instruction.x]. The BNNN instruction was not widely used,
                // so it is recommended to use the original behaviour.
                self.pc = self.registers[0] as u16 + instruction.nnn;
            }

            0x0C => {
                // Generates a random number, binary ANDs it with the value NN, and puts the result in VX.
                let random_number: u8 = rand::thread_rng().gen();
                self.registers[instruction.x] = random_number & instruction.nn;
            }

            0x0D => {
                // Draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels.
                // The sprite memory location is stored in the index register. The display location is updated
                // with the result of the current pixel XOR'd with the new pixel value stored in the sprite.
                // The carry flag register (VF) is set to 1 if any screen pixels are flipped from set to unset when the
                // sprite is drawn.

                // Wrap position
                let x_position = self.registers[instruction.x] % DISPLAY_WIDTH as u8;
                let y_position = self.registers[instruction.y] % DISPLAY_HEIGHT as u8;
                self.set_carry_flag(0);
                let mut collision = false;

                for row in 0..instruction.n {
                    let sprite_byte = self.memory[self.index as usize + row as usize];
                    let y = y_position + row;

                    if y >= 32 {
                        break;
                    }

                    for col in 0..8 {
                        let x = x_position + col;
                        if x >= 64 {
                            break;
                        }

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

            0x0E => {
                //
                match instruction.nn {
                    0x9E => {
                        // Skip next instruction if key with the value of Vx is pressed.
                        // Checks the keyboard, and if the key corresponding to the value of Vx
                        // is currently in the pressed, PC is increased by 2.
                        let key_index = self.registers[instruction.x];
                        let pressed = self.keys[key_index as usize];
                        if pressed {
                            self.pc += 2;
                        }
                    }
                    0xA1 => {
                        // Skip next instruction if key with the value of Vx is not pressed.
                        // Checks the keyboard, and if the key corresponding to the value of Vx
                        // is currently not pressed, PC is increased by 2.
                        let key_index = self.registers[instruction.x];
                        let pressed = self.keys[key_index as usize];
                        if !pressed {
                            self.pc += 2;
                        }
                    }
                    _ => unimplemented!(),
                }
            }

            0x0F => {
                match instruction.nn {
                    0x07 => {
                        // sets register X to the current value of the delay timer
                        self.registers[instruction.x] = self.delay_timer;
                    }
                    0x15 => {
                        // sets the delay timer to the value in register X
                        self.delay_timer = self.registers[instruction.x];
                    }
                    0x18 => {
                        // sets the sound timer to the value in register X
                        self.sound_timer = self.registers[instruction.x];
                    }
                    0x1E => {
                        // The index register I will get the value in VX added to it.
                        self.index = self.index + self.registers[instruction.x] as u16;
                    }
                    0x0A => {
                        // Wait for a key press, store the value of the key in Vx.
                        // All execution stops until a key is pressed, then the value of that key is stored in Vx.
                        let mut pressed = false;

                        for (i, key) in self.keys.into_iter().enumerate() {
                            if key {
                                pressed = true;
                                self.registers[instruction.x] = i as u8;
                                break;
                            }
                        }

                        if !pressed {
                            self.pc -= 2;
                        }
                    }
                    0x29 => {
                        // Set I to location of sprite for digit Vx.

                        // Returns the font digit, not the start location of the font
                        let digit = self.registers[instruction.x] as u16;

                        // Each digit is 5 bytes long, so we can multiple the digit by 5 to get the
                        // start location of the font digit
                        self.index = FONTSET_START_ADDRESS as u16 + 5 * digit;
                    }
                    0x33 => {
                        // The interpreter takes the decimal value of Vx, and  places
                        // the hundreds digit in memory at location in I, the tens
                        // digit at location I+1, and the ones digit at location I+2.

                        let value_x = self.registers[instruction.x];
                        let hundreds_digit = value_x / 100;
                        let tens_digit = (value_x % 100) / 10;
                        let ones_digit = value_x % 10;

                        self.memory[self.index as usize] = hundreds_digit;
                        self.memory[self.index as usize + 1 as usize] = tens_digit;
                        self.memory[self.index as usize + 2 as usize] = ones_digit;
                    }
                    0x55 => {
                        // Store registers V0 through Vx in memory starting at location I.
                        // The interpreter copies the values of registers V0 through Vx
                        // into memory, starting at the address in I.
                        for register_index in 0..=instruction.x {
                            let memory_location = self.index as usize + register_index;
                            self.memory[memory_location] = self.registers[register_index];
                        }
                    }
                    0x65 => {
                        // Read registers V0 through Vx from memory starting at location I.
                        // The interpreter reads values from memory starting at location I
                        // into registers V0 through Vx.
                        for register_index in 0..=instruction.x {
                            let memory_location = self.index as usize + register_index;
                            self.registers[register_index] = self.memory[memory_location];
                        }
                    }
                    _ => unimplemented!(),
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
