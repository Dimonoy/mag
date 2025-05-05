use crate::canvas::AppCanvasProps;
use crate::utils::zoom::zoom;

use sdl2::keyboard::Keycode;

pub(crate) fn handle_keyboard_events(
    canvas_props: &mut AppCanvasProps,
    keycode: Keycode,
    mouse_x: f32,
    mouse_y: f32,
) {
    match keycode {
        Keycode::Left => canvas_props.offset_x += 100.0, 
        Keycode::Right => canvas_props.offset_x -= 100.0, 
        Keycode::Up => canvas_props.offset_y += 100.0, 
        Keycode::Down => canvas_props.offset_y -= 100.0, 
        Keycode::Plus => zoom(canvas_props, mouse_x, mouse_y, 1),
        Keycode::Equals => zoom(canvas_props, mouse_x, mouse_y, 1),
        Keycode::Minus => zoom(canvas_props, mouse_x, mouse_y, -1),
        _ => (),
    }
}
