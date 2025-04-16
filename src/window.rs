use sdl2::{
    render::{TextureCreator, WindowCanvas},
	video::WindowContext,
	Sdl,
    image::InitFlag,
};


const APPLICATION_TITLE: &str = "Mag(nifier)";
const WINDOW_WIDTH: u32 = 1920;
const WINDOW_HEIGHT: u32 = 1080;


pub(crate) struct AppWindow {
    pub(crate) sdl_context: Sdl,
    pub(crate) window_canvas: WindowCanvas,
    pub(crate) texture_creator: TextureCreator<WindowContext>,
    pub(crate) window_width: u32,
    pub(crate) window_height: u32,
    pub(crate) zoom_scale: f32,
    pub(crate) offset_x: f32,
    pub(crate) offset_y: f32,
}

impl sdl2::image::LoadTexture for AppWindow {
    fn load_texture<P: AsRef<std::path::Path>>(&self, filename: P) -> Result<sdl2::render::Texture, String> {
        <TextureCreator<WindowContext> as sdl2::image::LoadTexture>::load_texture(&self.texture_creator, filename)
    }

    #[doc(alias = "IMG_LoadTexture")]
    fn load_texture_bytes(&self, buf: &[u8]) -> Result<sdl2::render::Texture, String> {
        <TextureCreator<WindowContext> as sdl2::image::LoadTexture>::load_texture_bytes(&self.texture_creator, buf)
    }
}

impl AppWindow {
    pub(crate) fn new() -> Result<Self, String> {
        let sdl_context = sdl2::init()?;
        let _image_context = sdl2::image::init(InitFlag::JPG)?;
        let (window_width, window_height) = (WINDOW_WIDTH, WINDOW_HEIGHT);
        let window = sdl_context.video()?.window(APPLICATION_TITLE, window_width, window_height)
            .fullscreen()
            .always_on_top()
            .hidden()
            .build()
            .map_err(|e| e.to_string())?;
        let window_canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
        let texture_creator = window_canvas.texture_creator();

        Ok(Self {
            sdl_context,
            window_canvas,
            texture_creator,
            window_width,
            window_height,
            zoom_scale: 1.0,
            offset_x: 0.0,
            offset_y: 0.0
        })
    }
}

