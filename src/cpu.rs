use std::ops::Add;

use crate::instruction::Instruction;

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
    display: [bool; 64 * 32],
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
            self.memory[0x200 + i] = *byte;
        }
    }

    pub fn execute_instruction(&mut self) {
        let instruction_bytes = self.fetch_instruction_bytes();
        println!("Instruction: #{:#018b}", instruction_bytes);

        let i = Instruction::from_bytes(instruction_bytes);

        match i.op_code {
            0x00 => match i.nn {
                0xE0 => println!("Clear screen"),
                _ => println!("Instruction not implemented."),
            },

            0x01 => {
                // Jump to location nnn
                println!("Jump to location: #{}", i.nnn);
                self.pc = i.nnn;
            }

            0x06 => {
                // Set register x to nn
                println!("Set register #{} to: #{}", i.x, i.nn);
                self.registers[(i.x) as usize] = i.nn;
            }

            0x07 => {
                // Add value nn to register x
                println!("Add value #{} to register #{}", i.nn, i.x);

                self.registers[(i.x) as usize] = self.registers[(i.x) as usize].add(i.nn);
            }

            0x0A => {
                // Set index register to nnn
                println!("Set index register to nnn: #{}", i.nnn);
                self.index = i.nnn;
            }

            0x0D => {
                println!("Printing to screen")
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
