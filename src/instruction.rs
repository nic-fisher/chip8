// Reference: http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#0nnn
// op_code - A 4-bit value, the first 4 bits of the instruction
// nnn or addr - A 12-bit value, the lowest 12 bits of the instruction
// nn or byte - An 8-bit value, the lowest 8 bits of the instruction
// n or nibble - A 4-bit value, the lowest 4 bits of the instruction
// x - A 4-bit value, the lower 4 bits of the high byte of the instruction
// y - A 4-bit value, the upper 4 bits of the low byte of the instruction

#[derive(Debug)]
pub struct Instruction {
    pub op_code: u8,
    pub nnn: u16,
    pub nn: u8,
    pub n: u8,
    pub x: usize,
    pub y: usize,
}

impl Instruction {
    pub fn from_bytes(bytes: u16) -> Self {
        Self {
            op_code: (bytes >> 12) as u8,
            nnn: bytes & 0x0FFF,
            nn: (bytes & 0x00FF) as u8,
            n: (bytes & 0x000F) as u8,
            x: (bytes >> 8 & 0xF) as usize,
            y: (bytes >> 4 & 0xF) as usize,
        }
    }
}
