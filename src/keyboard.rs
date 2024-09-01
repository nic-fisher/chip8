// Input mapping:
// Emulator     Chip8
// +-+-+-+-+    +-+-+-+-+
// |1|2|3|4|    |1|2|3|C|
// |Q|W|E|R|    |4|5|6|D|
// |A|S|D|F|    |7|8|9|E|
// |Z|X|C|V|    |A|0|B|F|
// +-+-+-+-+    +-+-+-+-+

use winit::event::VirtualKeyCode;

pub fn key_code_to_index(virtual_keycode: Option<VirtualKeyCode>) -> Option<usize> {
    return match virtual_keycode {
        Some(VirtualKeyCode::Key1) => Some(0x1),
        Some(VirtualKeyCode::Key2) => Some(0x2),
        Some(VirtualKeyCode::Key3) => Some(0x3),
        Some(VirtualKeyCode::Key4) => Some(0xC),
        Some(VirtualKeyCode::Q) => Some(0x4),
        Some(VirtualKeyCode::W) => Some(0x5),
        Some(VirtualKeyCode::E) => Some(0x6),
        Some(VirtualKeyCode::R) => Some(0xD),
        Some(VirtualKeyCode::A) => Some(0x7),
        Some(VirtualKeyCode::S) => Some(0x8),
        Some(VirtualKeyCode::D) => Some(0x9),
        Some(VirtualKeyCode::F) => Some(0xE),
        Some(VirtualKeyCode::Z) => Some(0xA),
        Some(VirtualKeyCode::X) => Some(0x0),
        Some(VirtualKeyCode::C) => Some(0xB),
        Some(VirtualKeyCode::V) => Some(0xF),
        _ => None,
    };
}
