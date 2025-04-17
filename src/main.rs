mod system_tray;
mod screenshot;
mod utils;
mod window;
mod app_state;
mod canvas;

use std::sync::{Arc, Mutex};

use device_query::{DeviceQuery, DeviceState, Keycode as DQKeycode};
use screenshot::Screenshot;
use system_tray::run_tray_icon;

use sdl2::{
    pixels::Color,
	event::Event,
	keyboard::Keycode as SDLKeycode,
	rect::Rect,
};

use crate::app_state::AppState;

const MIN_ZOOM_SCALE: f32 = 1.0;
const MAX_ZOOM_SCALE: f32 = 10.0;

fn run_app() -> Result<(), String> {
    let screenshot = Screenshot::capture();
    let mut app_state = AppState::new()?;
    let mut texture = app_state.canvas.texture_creator.create_texture_streaming(
            sdl2::pixels::PixelFormatEnum::RGBA32,
            app_state.canvas_props.width,
            app_state.canvas_props.height
        ).expect("Failed to create the texture");

    texture.update(None, screenshot.as_bytes(), (app_state.canvas_props.width * 4) as usize)
        .expect("Texture update failed");

    let mut event_pump = app_state.sdl_context.event_pump()?;

    app_state.canvas.window_canvas.window_mut().show();

    'running: loop {
        let mouse_state = event_pump.mouse_state();
        let mouse_x = mouse_state.x() as f32;
        let mouse_y = mouse_state.y() as f32;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} |
                Event::KeyDown { keycode: Some(SDLKeycode::Escape), .. } => break 'running,

                Event::KeyDown { keycode, .. } => {
                    println!("{:?}", keycode);
                },

                Event::MouseWheel { y, .. } if y != 0 => {
                    let prev_zoom = app_state.canvas_props.zoom_scale;

                    app_state.canvas_props.zoom_scale = 
                        (app_state.canvas_props.zoom_scale + y as f32 * 0.5).max(MIN_ZOOM_SCALE).min(MAX_ZOOM_SCALE);

                    // Adjust offsets to zoom into the mouse cursor's position
                    // Translate mouse position into the image coordinate space
                    let mouse_x_img = (mouse_x - app_state.canvas_props.offset_x) / prev_zoom;
                    let mouse_y_img = (mouse_y - app_state.canvas_props.offset_y) / prev_zoom;

                    // Update offsets to ensure zoom focuses on cursor
                    app_state.canvas_props.offset_x = mouse_x - mouse_x_img * app_state.canvas_props.zoom_scale;
                    app_state.canvas_props.offset_y = mouse_y - mouse_y_img * app_state.canvas_props.zoom_scale;
                },

                // Mouse drag to pan
                Event::MouseMotion { xrel, yrel, mousestate, .. } => {
                    if mousestate.left() {
                        app_state.canvas_props.offset_x += xrel as f32;
                        app_state.canvas_props.offset_y += yrel as f32;
                    }
                },

                _ => ()
            }
        }

        // Calculate destination (draw) size
        let dst_width = app_state.canvas_props.width as f32 * app_state.canvas_props.zoom_scale;
        let dst_height = app_state.canvas_props.height as f32 * app_state.canvas_props.zoom_scale;

        // Border clamping
        let min_offset_x = if dst_width > app_state.canvas_props.width as f32 {
            app_state.canvas_props.width as f32 - dst_width
        } else {
            (app_state.canvas_props.width as f32 - dst_width) / 2.0
        };
        let max_offset_x = if dst_width > app_state.canvas_props.width as f32 {
            0.0
        } else {
            (app_state.canvas_props.width as f32 - dst_width) / 2.0
        };

        let min_offset_y = if dst_height > app_state.canvas_props.height as f32 {
            app_state.canvas_props.height as f32 - dst_height
        } else {
            (app_state.canvas_props.height as f32 - dst_height) / 2.0
        };
        let max_offset_y = if dst_height > app_state.canvas_props.height as f32 {
            0.0
        } else {
            (app_state.canvas_props.height as f32 - dst_height) / 2.0
        };

        // Clamp the offsets so the image never leaves the window area
        app_state.canvas_props.offset_x = utils::clamp(app_state.canvas_props.offset_x, min_offset_x, max_offset_x);
        app_state.canvas_props.offset_y = utils::clamp(app_state.canvas_props.offset_y, min_offset_y, max_offset_y);

        app_state.canvas.window_canvas.set_draw_color(Color::BLACK);
        app_state.canvas.window_canvas.clear();

        // Draw the texture
        let dst_rect = Rect::new(
          app_state.canvas_props.offset_x.round() as i32,
          app_state.canvas_props.offset_y.round() as i32,
            dst_width as u32,
            dst_height as u32,
        );

        app_state.canvas.window_canvas.copy(&texture, None, Some(dst_rect))?;
        app_state.canvas.window_canvas.present();
    }

    Ok(())
}

fn run() -> Result<(), String> {
    let device_state = DeviceState::new();

    let is_app_running = Arc::new(Mutex::new(false));

    loop {
        let keys: Vec<DQKeycode> = device_state.get_keys();

        if keys.contains(&DQKeycode::LControl) && keys.contains(&DQKeycode::Z) && !*is_app_running.lock().unwrap() {
            *is_app_running.lock().unwrap() = true;

            let is_app_running_clone = Arc::clone(&is_app_running);
            std::thread::spawn(move || {
                if let Err(e) = run_app() {
                    log::error!("Magnifier app failed to boot: {}", e);
                    panic!("Magnifier app failed to boot");
                } else {
                    *is_app_running_clone.lock().unwrap() = false;
                }
            });
        }
    }
}

fn main() -> Result<(), String> {
    { // Tray Application
        run_tray_icon();
    }

    run()?;

    Ok(())
}

