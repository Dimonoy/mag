use crate::canvas::AppCanvasProps;

const MIN_ZOOM_SCALE: f32 = 1.0;
const MAX_ZOOM_SCALE: f32 = 8.0;
const ZOOM_SCALE_STEP: f32 = 0.5;

pub(crate) fn zoom(canvas_props: &mut AppCanvasProps, mouse_x: f32, mouse_y: f32, y_direction: i32) {
    let prev_zoom = canvas_props.zoom_scale;

    canvas_props.zoom_scale = (canvas_props.zoom_scale + y_direction as f32 * ZOOM_SCALE_STEP)
        .clamp(MIN_ZOOM_SCALE, MAX_ZOOM_SCALE);

    // Adjust offsets to zoom into the mouse cursor's position
    // Translate mouse position into the image coordinate space
    let mouse_x_img = (mouse_x - canvas_props.offset_x) / prev_zoom;
    let mouse_y_img = (mouse_y - canvas_props.offset_y) / prev_zoom;

    // Update offsets to ensure zoom focuses on cursor
    canvas_props.offset_x = mouse_x - mouse_x_img * canvas_props.zoom_scale;
    canvas_props.offset_y = mouse_y - mouse_y_img * canvas_props.zoom_scale;
}
