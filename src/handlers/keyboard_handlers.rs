use winit::{event::ElementState, event_loop::ActiveEventLoop, keyboard::KeyCode};

use crate::{actions::close_window, app::AppWindowState};

pub(crate) fn handle_raw_keyboard_events(
    key_code: KeyCode,
    press_state: ElementState,
    event_loop: &ActiveEventLoop,
    window_state: &mut Option<AppWindowState>,
    is_redraw_ready: &mut bool
) {
    match (key_code, press_state) {
        (KeyCode::F12, ElementState::Pressed) => {
            *window_state = Some(AppWindowState::create(event_loop));
        },
        (KeyCode::Escape, ElementState::Pressed) => close_window(window_state, is_redraw_ready),
        _ => (),
    }
}

