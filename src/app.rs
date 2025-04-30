use tray_icon::menu::MenuEventReceiver;

use crate::canvas::AppCanvasProps;
use crate::event_loop::{run_event_loop, EventLoopExitStatus};
use crate::window::create_window;

pub(crate) fn run_app(systray_menu_events_receiver: &MenuEventReceiver) -> Result<EventLoopExitStatus, String> {
    let sdl_context = sdl2::init()?;

    let canvas_props = AppCanvasProps::default();
    let canvas = create_window(&canvas_props, &sdl_context);
    let event_pump = sdl_context.event_pump().expect("Event pump failed to initialize");

    let event_loop_exit_status = run_event_loop(event_pump, canvas, canvas_props, systray_menu_events_receiver)?;

    Ok(event_loop_exit_status)
}
