use crate::canvas::{AppCanvas, AppCanvasProps};
use crate::events::handle_events;
use crate::events::LoopState;
use crate::texture::TextureWrapper;
use crate::window::create_window;

use sdl2::EventPump;

pub(crate) fn run_app() -> Result<(), String> {
    let sdl_context = sdl2::init()?;

    let canvas_props = AppCanvasProps::default();
    let canvas = create_window(&canvas_props, &sdl_context);
    let event_pump = sdl_context.event_pump().expect("Event pump failed to initialize");

    run_event_loop(event_pump, canvas, canvas_props)?;

    Ok(())
}

fn run_event_loop(
    mut event_pump: EventPump,
	mut canvas: AppCanvas,
	mut canvas_props: AppCanvasProps
) -> Result<(), String> {
    let texture_creator = canvas.get_texture_creator();
    let texture_wrapper = TextureWrapper::from_screenshot(&canvas_props, &texture_creator);

    canvas.show_window();

    let mut previous_offset_sum = 0.0;
    'running: loop {
        let mouse_state = event_pump.mouse_state();

        for event in event_pump.poll_iter() {
            match handle_events(
                event,
                &mut canvas_props,
                mouse_state.x() as f32,
                mouse_state.y() as f32,
            ) {
                LoopState::Exit => break 'running,
                LoopState::ForceUpdate => canvas.update(&mut canvas_props, &texture_wrapper)?,
                LoopState::Continue => (),
            }
        }

        if canvas_props.offset_x + canvas_props.offset_y != previous_offset_sum {
            canvas.update(&mut canvas_props, &texture_wrapper)?;
            println!("Continue");
            previous_offset_sum = canvas_props.offset_x + canvas_props.offset_y;
        }
    }

    Ok(())
}
