use crate::cpu::CPU;
use crate::display::Display;
use std::{env, fs};
use winit::event::{ElementState, Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

mod cpu;
mod display;
mod instruction;
mod keyboard;

const CYCLES_PER_FRAME: u8 = 10;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let rom: Vec<u8> = fs::read(file_path).expect("Failed to read rom file");
    let mut cpu = CPU::new();

    cpu.load_rom(rom);

    let event_loop = EventLoop::new();
    let mut display = Display::new(&event_loop);

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                    Some(VirtualKeyCode::Escape) => *control_flow = ControlFlow::Exit,
                    virtual_keycode => {
                        if let Some(key_index) = keyboard::key_code_to_index(virtual_keycode) {
                            if input.state == ElementState::Pressed {
                                cpu.key_press(key_index);
                            } else {
                                cpu.key_release(key_index);
                            }
                        }
                    }
                },
                _ => (),
            },
            _ => (),
        };

        for _ in 0..CYCLES_PER_FRAME {
            cpu.execute_instruction();
        }

        display.draw(&cpu);
    })
}
