use crate::canvas::AppCanvasProps;

use sdl2::rect::Rect;

pub(crate) fn calculate_resulting_resolution(canvas_props: &AppCanvasProps) -> (f32, f32) {
    (canvas_props.width as f32 * canvas_props.zoom_scale, canvas_props.height as f32 * canvas_props.zoom_scale)
}

/// Clamp the offsets so the image never leaves the window area
pub(crate) fn clamp_screen_borders(canvas_props: &mut AppCanvasProps, resulting_width: &f32, resulting_height: &f32) {
    let min_offset_x = canvas_props.width as f32 - resulting_width;
    let max_offset_x = 0.0;

    let min_offset_y = canvas_props.height as f32 - resulting_height;
    let max_offset_y = 0.0;

    canvas_props.offset_x = canvas_props.offset_x.clamp(min_offset_x, max_offset_x);
    canvas_props.offset_y = canvas_props.offset_y.clamp(min_offset_y, max_offset_y);
}

pub(crate) fn get_resolution_rectangle(canvas_props: &AppCanvasProps, resulting_width: f32, resulting_height: f32) -> Rect {
    Rect::new(
        canvas_props.offset_x.round() as i32,
        canvas_props.offset_y.round() as i32,
        resulting_width as u32,
        resulting_height as u32,
    )
}
