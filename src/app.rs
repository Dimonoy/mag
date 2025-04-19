use crate::canvas::{AppCanvas, AppCanvasProps};
use crate::screenshot::Screenshot;
use crate::window::create_window;

use sdl2::mouse::MouseState;
use sdl2::{
    event::Event, image::InitFlag, keyboard::Keycode as SDLKeycode, pixels::Color, rect::Rect, EventPump,
};

const MIN_ZOOM_SCALE: f32 = 1.0;
const MAX_ZOOM_SCALE: f32 = 10.0;

pub(crate) fn run_app() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    init_sdl2_image_context();

    let canvas_props = AppCanvasProps::default();
    let canvas = create_window(&canvas_props, &sdl_context);
    let event_pump = sdl_context.event_pump().expect("Event pump failed to initialize");

    run_event_loop(event_pump, canvas, canvas_props)?;

    Ok(())
}

fn run_event_loop(
    mut event_pump: EventPump,
	mut canvas: AppCanvas,
	mut canvas_props: AppCanvasProps
) -> Result<(), String> {
    let screenshot = Screenshot::capture();
    let mut texture = canvas.texture_creator.create_texture_streaming(
        sdl2::pixels::PixelFormatEnum::RGBA32,
        canvas_props.width,
        canvas_props.height
    ).expect("Failed to create the texture");

    texture.update(None, screenshot.as_bytes(), (canvas_props.width * 4) as usize)
        .expect("Texture update failed");

    canvas.window_canvas.window_mut().show();

    'running: loop {
        let mouse_state = event_pump.mouse_state();
        let mouse_x = mouse_state.x() as f32;
        let mouse_y = mouse_state.y() as f32;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} | Event::KeyDown { keycode: Some(SDLKeycode::Escape), .. } => break 'running,
                Event::KeyDown { keycode: Some(keycode), .. } => println!("{:?}", keycode),
                Event::MouseWheel { y, .. } if y != 0 => zoom(&mut canvas_props, &mouse_x, &mouse_y, y),
                Event::MouseMotion { xrel, yrel, mousestate, .. } => track_mouse(&mut canvas_props, xrel, yrel, mousestate),
                _ => ()
            }
        }

        // Calculate destination (draw) size
        let dst_width = canvas_props.width as f32 * canvas_props.zoom_scale;
        let dst_height = canvas_props.height as f32 * canvas_props.zoom_scale;

        // Border clamping
        let min_offset_x = if dst_width > canvas_props.width as f32 {
            canvas_props.width as f32 - dst_width
        } else {
            (canvas_props.width as f32 - dst_width) / 2.0
        };
        let max_offset_x = if dst_width > canvas_props.width as f32 {
            0.0
        } else {
            (canvas_props.width as f32 - dst_width) / 2.0
        };

        let min_offset_y = if dst_height > canvas_props.height as f32 {
            canvas_props.height as f32 - dst_height
        } else {
            (canvas_props.height as f32 - dst_height) / 2.0
        };
        let max_offset_y = if dst_height > canvas_props.height as f32 {
            0.0
        } else {
            (canvas_props.height as f32 - dst_height) / 2.0
        };

        // Clamp the offsets so the image never leaves the window area
        canvas_props.offset_x = crate::utils::clamp(canvas_props.offset_x, min_offset_x, max_offset_x);
        canvas_props.offset_y = crate::utils::clamp(canvas_props.offset_y, min_offset_y, max_offset_y);

        canvas.window_canvas.set_draw_color(Color::BLACK);
        canvas.window_canvas.clear();

        // Draw the texture
        let dst_rect = Rect::new(
          canvas_props.offset_x.round() as i32,
          canvas_props.offset_y.round() as i32,
            dst_width as u32,
            dst_height as u32,
        );

        canvas.window_canvas.copy(&texture, None, Some(dst_rect))?;
        canvas.window_canvas.present();
    }

    Ok(())
}

fn init_sdl2_image_context() {
    if let Err(e) = sdl2::image::init(InitFlag::JPG) {
        log::error!("SDL2 Image failed to load: {}", e);
        panic!("SDL2 Image was not loaded");
    }
}

fn zoom(canvas_props: &mut AppCanvasProps, mouse_x: &f32, mouse_y: &f32, y_direction: i32) {
    let prev_zoom = canvas_props.zoom_scale;

    canvas_props.zoom_scale = (canvas_props.zoom_scale + y_direction as f32 * 0.5)
        .max(MIN_ZOOM_SCALE)
        .min(MAX_ZOOM_SCALE);

    // Adjust offsets to zoom into the mouse cursor's position
    // Translate mouse position into the image coordinate space
    let mouse_x_img = (mouse_x - canvas_props.offset_x) / prev_zoom;
    let mouse_y_img = (mouse_y - canvas_props.offset_y) / prev_zoom;

    // Update offsets to ensure zoom focuses on cursor
    canvas_props.offset_x = mouse_x - mouse_x_img * canvas_props.zoom_scale;
    canvas_props.offset_y = mouse_y - mouse_y_img * canvas_props.zoom_scale;

    println!("ZoomScale: {} MouseX: {mouse_x} MouseY: {mouse_y} Y: {y_direction}", canvas_props.zoom_scale);
    println!("OffsetX: {} OffsetY: {}", canvas_props.offset_x, canvas_props.offset_y);
}

fn track_mouse(canvas_props: &mut AppCanvasProps, xrel: i32, yrel: i32, mousestate: MouseState) {
    if mousestate.left() {
        canvas_props.offset_x += xrel as f32;
        canvas_props.offset_y += yrel as f32;
    }
}
