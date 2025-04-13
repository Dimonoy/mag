use std::time::Instant;

use image::Rgba;
use screenshots::Screen;

#[derive(Default)]
pub struct Screenshot {
    screenshot: Option<image::RgbaImage>,
}

impl Screenshot {
    pub fn capture(&mut self) {
        let start = Instant::now();
        let screens = Screen::all().unwrap();
        let screen = screens[0];
        let image = screen.capture().unwrap();
        // image.save(format!("target/{}.png", screen.display_info.id)).expect("Screenshot PNG failed");

        self.screenshot = Some(image::RgbaImage::from_raw(
            image.width(),
            image.height(),
            image.to_vec()
        ).unwrap())
    }

    pub fn peek_image(&self) -> Option<&image::RgbaImage> {
        self.screenshot.as_ref()
    }

    pub fn get_dimensions(&self) -> (u32, u32) {
        let screenshot = self.screenshot.as_ref().expect("Hm... no screenshot taken, weird");

        screenshot.dimensions()
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> &Rgba<u8> {
        let screenshot = self.screenshot.as_ref().expect("Hm... no screenshot taken, weird");

        screenshot.get_pixel(x, y)
    }
}
