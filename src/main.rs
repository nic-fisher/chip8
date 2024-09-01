use crate::cpu::CPU;
use crate::display::Display;
use std::thread;
use std::time::{Duration, Instant};
use std::{env, fs};
use winit::event::{ElementState, Event, VirtualKeyCode, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

mod cpu;
mod display;
mod instruction;
mod keyboard;

const CYCLES_PER_SECOND: f64 = 60.0;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let rom: Vec<u8> = fs::read(file_path).expect("Failed to read rom file");
    let mut cpu = CPU::new();

    cpu.load_rom(rom);

    let event_loop = EventLoop::new();
    let mut display = Display::new(&event_loop);

    // Duration of one cycle (60 FPS)
    let target_cycle_duration = Duration::from_secs_f64(1.0 / CYCLES_PER_SECOND);

    event_loop.run(move |event, _, control_flow| {
        let frame_start = Instant::now();
        cpu.execute_instruction();

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

        display.draw(&cpu);

        // Calculate how long to sleep to maintain 60 FPS
        let cycle_duration = Instant::now().duration_since(frame_start);
        if cycle_duration < target_cycle_duration {
            thread::sleep(target_cycle_duration - cycle_duration);
        }
    })
}
