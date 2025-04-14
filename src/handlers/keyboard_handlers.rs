use winit::{event::ElementState, event_loop::ActiveEventLoop, keyboard::KeyCode, window::Window};

use crate::{actions::{capture_screenshot_and_create_window, close_window}, screenshot::Screenshot};

pub(crate) fn handle_raw_keyboard_events(
    key_code: KeyCode,
    state: ElementState,
    event_loop: &ActiveEventLoop,
    window: &mut Option<Window>,
    screenshot: &mut Option<Screenshot>,
    is_redraw_ready: &mut bool
) {
    match (key_code, state) {
        (KeyCode::F12, ElementState::Pressed) => capture_screenshot_and_create_window(event_loop, window, screenshot, is_redraw_ready),
        (KeyCode::Escape, ElementState::Pressed) => close_window(window, screenshot, is_redraw_ready),
        _ => (),
    }
}

