use tray_icon::menu::MenuEventReceiver;

use crate::event_loop::run_event_loop;
use crate::program_state::{ProgramExitType, ProgramRuntime};
use crate::window::create_window;

pub(crate) fn run_app(systray_menu_events_receiver: &MenuEventReceiver) -> Result<ProgramExitType, String> {
    sdl2::hint::set("SDL_RENDER_SCALE_QUALITY", "best");
    let sdl_context = sdl2::init()?;

    let canvas = create_window(&sdl_context);
    let program_runtime = ProgramRuntime::new(canvas);
    let event_pump = sdl_context.event_pump().expect("Event pump failed to initialize");

    let program_exit_type = run_event_loop(event_pump, program_runtime, systray_menu_events_receiver)?;

    Ok(program_exit_type)
}
