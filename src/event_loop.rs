use std::time::{Duration, Instant};

use crate::canvas::AppCanvas;
use crate::events::handle_events;
use crate::program_state::{ProgramExitType, ProgramRuntime};
use crate::system_tray::is_systray_menu_quit_clicked;
use crate::texture::TextureWrapper;
use crate::utils::timer::calculate_elapsed_time;

use sdl2::EventPump;
use tray_icon::menu::MenuEventReceiver;

const FPS: usize = 80;
const MS_PER_UPDATE: Duration = Duration::from_millis((1000 / FPS) as u64);

pub(crate) fn run_event_loop(
    mut event_pump: EventPump,
    mut program_runtime: ProgramRuntime,
    systray_menu_events_receiver: &MenuEventReceiver,
) -> Result<ProgramExitType, String> {
    let mut program_exit_type = ProgramExitType::Close;
    let texture_creator = program_runtime.canvas.get_texture_creator();
    let texture_wrapper = TextureWrapper::from_screenshot(&program_runtime.canvas.props, &texture_creator);

    program_runtime.canvas.show_window();

    let mut last_time_updated = Instant::now();
    let mut lag = Duration::new(0, 0);
    'running: loop {
        lag += calculate_elapsed_time(&mut last_time_updated);
        
        if is_systray_menu_quit_clicked(systray_menu_events_receiver) {
            program_runtime.set_state_exit();
            break 'running;
        }

        let (mouse_x, mouse_y) = (event_pump.mouse_state().x() as f32, event_pump.mouse_state().y() as f32);

        for event in event_pump.poll_iter() {
            if let Err(exit_type) = program_runtime.execute(&texture_wrapper) {
                program_exit_type = exit_type;
                break 'running;
            }
            handle_events(event, &mut program_runtime, mouse_x, mouse_y);
        }

        canvas_update_with_lag(&mut program_runtime.canvas, &texture_wrapper, &mut lag)?;

        std::thread::sleep(Duration::from_millis(1));
    }

    Ok(program_exit_type)
}

pub(crate) fn canvas_update_with_lag(
    canvas: &mut AppCanvas,
    texture_wrapper: &TextureWrapper,
    lag: &mut Duration
) -> Result<(), String> {
    while *lag >= MS_PER_UPDATE {
        canvas.update(texture_wrapper)?;
        *lag -= MS_PER_UPDATE;
    }

    Ok(())
}
