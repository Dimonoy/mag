use pixels::{Pixels, SurfaceTexture};
use winit::{dpi::PhysicalSize, window::Window};

use crate::screenshot::Screenshot;

#[derive(Default)]
pub struct Renderer<'win> {
    pixels: Option<Pixels<'win>>
}

impl<'win> Renderer<'win> {
    pub fn new(window: &'win Window) -> Self {
        let PhysicalSize { width, height } = window.inner_size();
        println!("Inner width/height: {width}, {height}");
        let surface_texture = SurfaceTexture::new(width, height, window);

        Self {
            pixels: Some(Pixels::new(width, height, surface_texture).unwrap())
        }
    }

    pub fn render_screenshot(&mut self, screenshot: &Screenshot) {
        let pixels = self.pixels.as_mut().expect("No no, capture_screenshot first!");
        let (width, height) = screenshot.get_dimensions();
        println!("Width/height of the screenshot: {width} {height}");

        // let _ = pixels.resize_buffer(width, height);
        let frame = pixels.frame_mut();
        println!("Frame length: {}", frame.len());

        let frame = pixels.frame_mut();
        if let Some(image) = screenshot.peek_image() {
            for (y, row) in image.rows().enumerate() {
                for (x, pixel) in row.enumerate() {
                    let idx = ((y * width as usize + x) * 4) as usize;
                    frame[idx..idx + 4].copy_from_slice(&pixel.0);
                }
            }
        }

        // for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        //     // let x = (i % width as usize) as i16;
        //     // let y = (i / width as usize) as i16;
        //
        //     pixel.copy_from_slice(&[0, 255, 0, 255]);
        // }

        let _ = pixels.render().expect("Failed to render frame");
    }
}
