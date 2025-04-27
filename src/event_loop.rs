use std::time::{Duration, Instant};

use crate::canvas::{AppCanvas, AppCanvasProps};
use crate::events::handle_events;
use crate::events::LoopState;
use crate::texture::TextureWrapper;
use crate::utils::timer::calculate_elapsed_time;

use sdl2::EventPump;

const FPS: usize = 100;
const MS_PER_UPDATE: Duration = Duration::from_millis((1000 / FPS) as u64);

pub(crate) fn run_event_loop(
    mut event_pump: EventPump,
	mut canvas: AppCanvas,
	mut canvas_props: AppCanvasProps
) -> Result<(), String> {
    let texture_creator = canvas.get_texture_creator();
    let texture_wrapper = TextureWrapper::from_screenshot(&canvas_props, &texture_creator);

    canvas.show_window();

    let mut last_time_updated = Instant::now();
    let mut lag = Duration::new(0, 0);
    'running: loop {
        lag += calculate_elapsed_time(&mut last_time_updated);
        
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

        while lag >= MS_PER_UPDATE {
            canvas.update(&mut canvas_props, &texture_wrapper)?;
            lag -= MS_PER_UPDATE;
        }

        std::thread::sleep(Duration::from_millis(1));
    }

    Ok(())
}
