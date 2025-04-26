use crate::canvas::AppCanvasProps;

use sdl2::mouse::MouseState;

pub(crate) fn track_left_click_holded_mouse(canvas_props: &mut AppCanvasProps, xrel: i32, yrel: i32, mousestate: MouseState) {
    if mousestate.left() {
        canvas_props.offset_x += xrel as f32;
        canvas_props.offset_y += yrel as f32;
    }
}
