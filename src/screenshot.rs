use image::imageops::{resize, FilterType};
use screenshots::Screen;

pub(crate) struct Screenshot {
    screenshot: Option<image::RgbaImage>,
}

impl Screenshot {
    pub(crate) fn capture() -> Self {
        let screens = Screen::all().unwrap();
        let screen = screens[0];
        let image = screen.capture().unwrap();

        Self {
            screenshot: Some(image::RgbaImage::from_raw(
                image.width(),
                image.height(),
                image.to_vec()
            ).unwrap())
        }
    }

    pub(crate) fn peek_image(&self) -> Option<&image::RgbaImage> {
        self.screenshot.as_ref()
    }

    pub(crate) fn get_dimensions(&self) -> (u32, u32) {
        self.screenshot
            .as_ref()
            .expect("Hm... no screenshot taken, weird")
            .dimensions()
    }

    pub(crate) fn resize(&mut self, width: u32, height: u32) {
        self.screenshot = Some(resize(
            self.screenshot.as_ref().expect("Weird, screenshot was made though ?-?"), width, height, FilterType::Nearest
        ));
    }
}
