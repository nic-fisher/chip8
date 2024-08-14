use crate::cpu::CPU;
use std::{env, fs};

mod cpu;
mod instruction;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let rom: Vec<u8> = fs::read(file_path).expect("Failed to read rom file");
    let mut cpu = CPU::new();

    cpu.load_rom(rom);

    cpu.execute_instruction();
}
