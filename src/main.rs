use crate::cpu::CPU;
use pixels::{Pixels, SurfaceTexture};
use std::{env, fs};
use winit::dpi::LogicalSize;
use winit::event::VirtualKeyCode;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

mod cpu;
mod instruction;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 320;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_resizable(false)
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap()
    };

    let rom: Vec<u8> = fs::read(file_path).expect("Failed to read rom file");
    let mut cpu = CPU::new();

    cpu.load_rom(rom);

    for _ in 0..50 {
        cpu.execute_instruction();
    }

    event_loop.run(move |event, _, control_flow| {
        // Handle input events
        if input.update(&event) {
            // Close events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }

            if input.key_pressed(VirtualKeyCode::A) || input.quit() {
                // Each pixel is represented by 4 bytes in the frame buffer: R, G, B, and A.
                println!("Drawing to screen");
                let width = WIDTH as usize;
                let frame = pixels.frame_mut();

                for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
                    let x = i % width / 10;
                    let y = i / width / 10;

                    let display_pixel_index = cpu.get_display_pixel_index(x, y);
                    let pixel_enabled = cpu.get_display_pixel(display_pixel_index);

                    let rgba = if pixel_enabled { [0xFF; 4] } else { [0x00; 4] };

                    pixel.copy_from_slice(&rgba);
                }

                pixels.render().unwrap();
            }
        }
    })
}
