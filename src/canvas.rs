use crate::{
    texture::TextureWrapper,
    utils::{
        screen::calculate_resulting_resolution,
        screen::clamp_screen_borders,
        screen::get_resolution_rectangle,
    },
};

use sdl2::{
    rect::Rect,
    render::{Texture, TextureCreator, WindowCanvas},
    video::WindowContext,
    Sdl,
};


const APPLICATION_TITLE: &str = "Mag(nifier)";
const RESOLUTION_WIDTH: u32 = 1920;
const RESOLUTION_HEIGHT: u32 = 1080;

pub struct AppCanvas {
    window_canvas: WindowCanvas,
    pub props: AppCanvasProps,
}

#[derive(Debug)]
pub(crate) struct AppCanvasProps {
    pub(crate) zoom_scale: f32,
    pub(crate) offset_x: f32,
    pub(crate) offset_y: f32,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl AppCanvas {
    pub(crate) fn new(sdl_context: &Sdl) -> Result<Self, String> {
        let app_canvas_props = AppCanvasProps::default();
        let window = sdl_context.video()?.window(APPLICATION_TITLE, app_canvas_props.width, app_canvas_props.height)
            .fullscreen()
            .always_on_top()
            .hidden()
            .build()
            .map_err(|e| e.to_string())?;
        let window_canvas = window.into_canvas()
            .present_vsync()
            .accelerated()
            .build()
            .map_err(|e| e.to_string())?;

        Ok(Self { window_canvas, props: app_canvas_props })
    }

    pub(crate) fn get_texture_creator(&self) -> TextureCreator<WindowContext> {
        self.window_canvas.texture_creator()
    }

    pub(crate) fn show_window(&mut self) {
        self.window_canvas.window_mut().show();
    }

    fn update_texture(&mut self, texture: &Texture, target_rect: Rect) -> Result<(), String>{
        self.window_canvas.copy(&texture, None, Some(target_rect))?;
        self.window_canvas.present();
        Ok(())
    }

    pub(crate) fn update(&mut self, texture_wrapper: &TextureWrapper) -> Result<(), String> {
        let (resulting_width, resulting_height) = calculate_resulting_resolution(&self.props);
        clamp_screen_borders(&mut self.props, &resulting_width, &resulting_height);

        let target_rect = get_resolution_rectangle(&self.props, resulting_width, resulting_height);

        #[cfg(feature = "dev")]
        log::info!("Target rectangle: {:?}", target_rect);

        self.update_texture(texture_wrapper.texture(), target_rect)?;
        Ok(())
    }
}

impl Default for AppCanvasProps {
    fn default() -> Self {
        Self {
            width: RESOLUTION_WIDTH,
            height: RESOLUTION_HEIGHT,
            zoom_scale: 1.0,
            offset_x: 0.0,
            offset_y: 0.0,
        }
    }
}
