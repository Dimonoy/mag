use screenshots::Screen;

#[derive(Default)]
pub(crate) struct Screenshot {
    screenshot: Option<image::RgbaImage>,
}

impl Screenshot {
    pub(crate) fn capture(&mut self) {
        let screens = Screen::all().unwrap();
        let screen = screens[0];
        let image = screen.capture().unwrap();

        self.screenshot = Some(image::RgbaImage::from_raw(
            image.width(),
            image.height(),
            image.to_vec()
        ).unwrap())
    }

    pub(crate) fn peek_image(&self) -> Option<&image::RgbaImage> {
        self.screenshot.as_ref()
    }

    pub(crate) fn get_dimensions(&self) -> (u32, u32) {
        let screenshot = self.screenshot.as_ref().expect("Hm... no screenshot taken, weird");

        screenshot.dimensions()
    }
}
