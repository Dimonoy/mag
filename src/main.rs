mod user_event;
mod system_tray;
mod screenshot;
mod utils;
mod window;

use crate::window::AppWindow;

use screenshot::Screenshot;
use system_tray::run_tray_icon;

use screenshots::image::EncodableLayout;
use sdl2::{
    pixels::Color,
	event::Event,
	keyboard::Keycode,
	rect::Rect,
};


const MIN_ZOOM_SCALE: f32 = 1.0;
const MAX_ZOOM_SCALE: f32 = 10.0;


fn run_app() -> Result<(), String> {
    let screenshot = Screenshot::capture();
    let image = screenshot.peek_image().expect("Screenshot was not taken.");

    let mut application_window = AppWindow::new()?;

    let mut texture = application_window.texture_creator.create_texture_streaming(
        sdl2::pixels::PixelFormatEnum::RGBA32,
        application_window.window_width,
        application_window.window_height
    ).expect("Failed to create the texture");

    texture.update(None, image.as_bytes(), (application_window.window_width * 4) as usize)
        .expect("Texture update failed");

    let mut event_pump = application_window.sdl_context.event_pump()?;

    application_window.window_canvas.window_mut().show();

    'running: loop {
        let mouse_state = event_pump.mouse_state();
        let mouse_x = mouse_state.x() as f32;
        let mouse_y = mouse_state.y() as f32;

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,

                Event::MouseWheel { y, .. } if y != 0 => {
                    let prev_zoom = application_window.zoom_scale;

                    application_window.zoom_scale = 
                        (application_window.zoom_scale + y as f32 * 0.5).max(MIN_ZOOM_SCALE).min(MAX_ZOOM_SCALE);

                    // Adjust offsets to zoom into the mouse cursor's position
                    // Translate mouse position into the image coordinate space
                    let mouse_x_img = (mouse_x - application_window.offset_x) / prev_zoom;
                    let mouse_y_img = (mouse_y - application_window.offset_y) / prev_zoom;

                    // Update offsets to ensure zoom focuses on cursor
                    application_window.offset_x = mouse_x - mouse_x_img * application_window.zoom_scale;
                    application_window.offset_y = mouse_y - mouse_y_img * application_window.zoom_scale;
                },

                // Mouse drag to pan
                Event::MouseMotion { xrel, yrel, mousestate, .. } => {
                    if mousestate.left() {
                        application_window.offset_x += xrel as f32;
                        application_window.offset_y += yrel as f32;
                    }
                },

                _ => {}
            }
        }

        // Calculate destination (draw) size
        let dst_width = application_window.window_width as f32 * application_window.zoom_scale;
        let dst_height = application_window.window_height as f32 * application_window.zoom_scale;

        // Border clamping
        let min_offset_x = if dst_width > application_window.window_width as f32 {
            application_window.window_width as f32 - dst_width
        } else {
            (application_window.window_width as f32 - dst_width) / 2.0
        };
        let max_offset_x = if dst_width > application_window.window_width as f32 {
            0.0
        } else {
            (application_window.window_width as f32 - dst_width) / 2.0
        };

        let min_offset_y = if dst_height > application_window.window_height as f32 {
            application_window.window_height as f32 - dst_height
        } else {
            (application_window.window_height as f32 - dst_height) / 2.0
        };
        let max_offset_y = if dst_height > application_window.window_height as f32 {
            0.0
        } else {
            (application_window.window_height as f32 - dst_height) / 2.0
        };

        // Clamp the offsets so the image never leaves the window area
      application_window.offset_x = utils::clamp(application_window.offset_x, min_offset_x, max_offset_x);
      application_window.offset_y = utils::clamp(application_window.offset_y, min_offset_y, max_offset_y);

        application_window.window_canvas.set_draw_color(Color::BLACK);
        application_window.window_canvas.clear();

        // Draw the texture
        let dst_rect = Rect::new(
          application_window.offset_x.round() as i32,
          application_window.offset_y.round() as i32,
            dst_width as u32,
            dst_height as u32,
        );

        application_window.window_canvas.copy(&texture, None, Some(dst_rect))?;
        application_window.window_canvas.present();
    }

    Ok(())
}

fn main() -> Result<(), String> {
    { // Tray Application
        run_tray_icon();
    }

    run_app()?;

    Ok(())
}

