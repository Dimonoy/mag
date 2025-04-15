use winit::{
    event_loop::ActiveEventLoop,
    window::{Fullscreen, Window, WindowAttributes}
};

use crate::app::AppWindowState;

pub(crate) fn create_window(event_loop: &ActiveEventLoop) -> Window {
    let monitor = event_loop.primary_monitor().expect("Primary monitor is not available for some reason ?.?");

    let window_attributes = WindowAttributes::default()
        .with_inner_size(monitor.size())
        .with_min_inner_size(monitor.size())
        .with_fullscreen(Some(Fullscreen::Borderless(Some(monitor))))
        .with_visible(false);

    event_loop.create_window(window_attributes).expect("This event loop s*cks at creating windows :(")
}

pub(crate) fn close_window(window_state: &mut Option<AppWindowState>, is_redraw_ready: &mut bool) {
    if window_state.is_none() { return }

    *window_state = None;
    *is_redraw_ready = true;
}
