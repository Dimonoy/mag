use sdl2::{Sdl, image::InitFlag};

use crate::{canvas::{AppCanvas, AppCanvasProps}, window::create_window};

pub (crate) struct AppState {
    pub(crate) canvas: AppCanvas,
    pub(crate) canvas_props: AppCanvasProps,
    pub(crate) sdl_context: Sdl,
}

impl AppState {
    pub(crate) fn new() -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let canvas_props = AppCanvasProps::default();
        init_sdl2_image_context();

        Ok(Self {
            canvas: create_window(&canvas_props, &sdl_context),
            canvas_props,
            sdl_context,
        })
    }
}

fn init_sdl2_image_context() {
    if let Err(e) = sdl2::image::init(InitFlag::JPG) {
        log::error!("SDL2 Image failed to load: {}", e);
        panic!("SDL2 Image was not loaded");
    }
}
