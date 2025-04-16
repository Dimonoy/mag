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
}
