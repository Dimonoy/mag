use crate::canvas::AppCanvasProps;
use crate::event_loop::run_event_loop;
use crate::window::create_window;


pub(crate) fn run_app() -> Result<(), String> {
    let sdl_context = sdl2::init()?;

    let canvas_props = AppCanvasProps::default();
    let canvas = create_window(&canvas_props, &sdl_context);
    let event_pump = sdl_context.event_pump().expect("Event pump failed to initialize");

    run_event_loop(event_pump, canvas, canvas_props)?;

    Ok(())
}
