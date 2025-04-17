use sdl2::{
    render::{TextureCreator, WindowCanvas},
	video::WindowContext,
	Sdl,
};

const APPLICATION_TITLE: &str = "Mag(nifier)";
const RESOLUTION_WIDTH: u32 = 1920;
const RESOLUTION_HEIGHT: u32 = 1080;

pub(crate) struct AppCanvas {
    pub(crate) window_canvas: WindowCanvas,
    pub(crate) texture_creator: TextureCreator<WindowContext>,
}

pub(crate) struct AppCanvasProps {
    pub(crate) zoom_scale: f32,
    pub(crate) offset_x: f32,
    pub(crate) offset_y: f32,
    pub(crate) width: u32,
    pub(crate) height: u32,
}

impl AppCanvas {
    pub(crate) fn new(app_canvas_props: &AppCanvasProps, sdl_context: &Sdl) -> Result<Self, String> {
        let window = sdl_context.video()?.window(APPLICATION_TITLE, app_canvas_props.width, app_canvas_props.height)
            .fullscreen()
            .always_on_top()
            .hidden()
            .build()
            .map_err(|e| e.to_string())?;
        let window_canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        let texture_creator = window_canvas.texture_creator();

        Ok(Self {
            window_canvas,
            texture_creator,
        })
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
