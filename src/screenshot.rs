use image::EncodableLayout;
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

    pub(crate) fn as_bytes(&self) -> &[u8] {
        self.screenshot.as_ref().expect("Screenshot was not taken").as_bytes()
    }
}
