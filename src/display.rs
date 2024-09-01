use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};
// use winit_input_helper::WinitInputHelper;

use crate::cpu::CPU;

const WIDTH: u32 = 640;
const HEIGHT: u32 = 320;

pub struct Display {
    // pub input: WinitInputHelper,
    pub window: Window,
    pub pixels: Pixels,
}

impl Display {
    pub fn new(event_loop: &EventLoop<()>) -> Display {
        let window_size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        let window = WindowBuilder::new()
            .with_title("CHIP-8")
            .with_resizable(false)
            .with_inner_size(window_size)
            .with_min_inner_size(window_size)
            .build(&event_loop)
            .unwrap();

        let surface_texture = SurfaceTexture::new(
            window.inner_size().width,
            window.inner_size().height,
            &window,
        );
        let pixels = Pixels::new(WIDTH, HEIGHT, surface_texture).unwrap();

        Display {
            // input: WinitInputHelper::new(),
            window: window,
            pixels: pixels,
        }
    }

    pub fn draw(&mut self, cpu: &CPU) {
        let frame = self.pixels.frame_mut();

        // Draw to screen
        // Each pixel is represented by 4 bytes in the frame buffer: R, G, B, and A.
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
            let x = i % WIDTH as usize / 10;
            let y = i / WIDTH as usize / 10;

            let display_pixel_index = cpu.get_display_pixel_index(x, y);
            let pixel_enabled = cpu.get_display_pixel(display_pixel_index);

            let rgba = if pixel_enabled { [0xFF; 4] } else { [0x00; 4] };

            pixel.copy_from_slice(&rgba);
        }

        self.pixels.render().unwrap();
    }
}
