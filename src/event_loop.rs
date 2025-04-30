use std::time::{Duration, Instant};

use crate::canvas::{AppCanvas, AppCanvasProps};
use crate::events::handle_events;
use crate::events::LoopState;
use crate::system_tray::is_systray_menu_quit_clicked;
use crate::texture::TextureWrapper;
use crate::utils::timer::calculate_elapsed_time;

use sdl2::EventPump;
use tray_icon::menu::MenuEventReceiver;

const FPS: usize = 80;
const MS_PER_UPDATE: Duration = Duration::from_millis((1000 / FPS) as u64);

pub(crate) enum EventLoopExitStatus {
    FullExit,
    SoftExit,
}

impl EventLoopExitStatus {
    pub(crate) fn is_full_exit(&self) -> bool {
        if let Self::FullExit = self {
            true
        } else {
            false
        }
    }
}

pub(crate) fn run_event_loop(
    mut event_pump: EventPump,
	mut canvas: AppCanvas,
	mut canvas_props: AppCanvasProps,
    systray_menu_events_receiver: &MenuEventReceiver,
) -> Result<EventLoopExitStatus, String> {
    let mut event_loop_exit_status = EventLoopExitStatus::SoftExit;
    let texture_creator = canvas.get_texture_creator();
    let texture_wrapper = TextureWrapper::from_screenshot(&canvas_props, &texture_creator);

    canvas.show_window();

    let mut last_time_updated = Instant::now();
    let mut lag = Duration::new(0, 0);
    'running: loop {
        lag += calculate_elapsed_time(&mut last_time_updated);
        
        let mouse_state = event_pump.mouse_state();

        if is_systray_menu_quit_clicked(systray_menu_events_receiver) {
            event_loop_exit_status = EventLoopExitStatus::FullExit;
            break 'running;
        }

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

        canvas_update_with_lag(&mut canvas, &mut canvas_props, &texture_wrapper, &mut lag)?;

        std::thread::sleep(Duration::from_millis(1));
    }

    Ok(event_loop_exit_status)
}

pub(crate) fn canvas_update_with_lag(
    canvas: &mut AppCanvas,
    canvas_props: &mut AppCanvasProps,
    texture_wrapper: &TextureWrapper,
    lag: &mut Duration
) -> Result<(), String> {
    while *lag >= MS_PER_UPDATE {
        canvas.update(canvas_props, texture_wrapper)?;
        *lag -= MS_PER_UPDATE;
    }

    Ok(())
}
