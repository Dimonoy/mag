use winit::{
    event_loop::ActiveEventLoop,
    window::{Fullscreen, Window, WindowAttributes}
};

use crate::screenshot::Screenshot;

pub(crate) fn create_window(event_loop: &ActiveEventLoop) -> Option<Window> {
    let monitor = event_loop.primary_monitor().expect("Primary monitor is not available for some reason ?.?");

    let window_attributes = WindowAttributes::default()
        .with_inner_size(monitor.size())
        .with_min_inner_size(monitor.size())
        .with_fullscreen(Some(Fullscreen::Borderless(Some(monitor))))
        .with_visible(false);

    Some(event_loop.create_window(window_attributes).expect("This event loop s*cks at creating windows :("))
}

pub(crate) fn capture_screenshot_and_create_window(
    event_loop: &ActiveEventLoop,
    window: &mut Option<Window>,
    screenshot: &mut Option<Screenshot>,
    is_redraw_ready: &mut bool
) {
    if window.is_some() { return }

    *screenshot = Some(Screenshot::default());
    screenshot.as_mut().map(|screenshot_ref| {
        screenshot_ref.capture();
    });
    *window = create_window(event_loop);
    *is_redraw_ready = true;
}

pub(crate) fn close_window(
    window: &mut Option<Window>,
    screenshot: &mut Option<Screenshot>,
    is_redraw_ready: &mut bool
) {
    if window.is_none() { return }
    *window = None;
    *screenshot = None;
    *is_redraw_ready = true;
}
