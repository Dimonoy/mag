use sdl2::{
    render::{Texture, TextureCreator},
    video::WindowContext
};

use crate::{
    canvas::AppCanvasProps,
    screenshot::Screenshot,
};

pub(crate) struct TextureWrapper<'rc> {
    texture: Texture<'rc>,
}

impl<'tc> TextureWrapper<'tc> {
    pub(crate) fn new(canvas_props: &AppCanvasProps, texture_creator: &'tc TextureCreator<WindowContext>) -> Self {
        let texture = texture_creator.create_texture_streaming(
            sdl2::pixels::PixelFormatEnum::RGBA32,
            canvas_props.width,
            canvas_props.height
        ).expect("Failed to create the texture");

        Self { texture }
    }

    pub(crate) fn from_screenshot(canvas_props: &AppCanvasProps, texture_creator: &'tc TextureCreator<WindowContext>) -> Self {
        let screenshot = Screenshot::capture();
        let mut texture = TextureWrapper::new(canvas_props, texture_creator);

        texture.update(screenshot.as_bytes(), (canvas_props.width * 4) as usize);

        texture
    }
    
    pub(crate) fn update(&mut self, pixel_data: &[u8], pitch: usize) {
        self.texture.update(None, pixel_data, pitch).expect("Texture update failed");
    }

    pub(crate) fn texture(&self) -> &Texture {
        &self.texture
    }
}
