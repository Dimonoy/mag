use pixels::{Pixels, SurfaceTexture};
use winit::{dpi::PhysicalSize, window::Window};

use crate::screenshot::Screenshot;

pub struct Renderer<'win> {
    pixels: Option<Pixels<'win>>,
    screenshot: &'win mut Screenshot,
}

impl<'win> Renderer<'win> {
    pub fn new(window: &'win Window, screenshot: &'win mut Screenshot) -> Self {
        let PhysicalSize { width, height } = window.inner_size();
        println!("Inner width/height: {width}, {height}");
        let surface_texture = SurfaceTexture::new(width, height, window);

        Self {
            pixels: Some(Pixels::new(320, 180, surface_texture).unwrap()),
            screenshot,
        }
    }

    pub fn render(&mut self) {
        let pixels = self.pixels.as_mut().expect("No no, capture_screenshot first!");
        let (width, height) = self.screenshot.get_dimensions();
        let _ = pixels.resize_buffer(width, height);

        self.screenshot.resize(width, height);

        let frame = pixels.frame_mut();

        if let Some(image) = self.screenshot.peek_image() {
            for (y, row) in image.rows().enumerate() {
                for (x, pixel) in row.enumerate() {
                    let idx = ((y * width as usize + x) * 4) as usize;
                    frame[idx..idx + 4].copy_from_slice(&pixel.0);
                }
            }
        }

        let _ = pixels.render().expect("Failed to render frame");
    }
}
