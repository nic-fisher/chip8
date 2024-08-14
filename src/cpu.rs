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
        CPU {
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
                unimplemented!()
            }

            0x06 => {
                // Set register VX to nn
                unimplemented!()
            }

            0x07 => {
                // Add value nn to register VX
                unimplemented!()
            }

            0x0A => {
                // Set index register I to nnn
                unimplemented!()
            }

            0x0D => {
                // Draw / display
                unimplemented!()
            }

            _ => println!("Instruction not implemented"),
        }

        println!("B: #{:?}", i);
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
