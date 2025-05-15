use sdl2::Sdl;

use crate::canvas::AppCanvas;

pub(crate) fn create_window(sdl_context: &Sdl) -> AppCanvas {
    match AppCanvas::new(sdl_context) {
        Ok(app_canvas) => app_canvas,
        Err(e) => {
            #[cfg(feature = "dev")]
            log::error!("SDL2 failed to create a window canvas: {}", e);
            panic!("SDL failed to create a window canvas: {}", e);
        },
    }
}
