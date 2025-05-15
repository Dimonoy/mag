use crate::{
    program_state::ProgramRuntime, utils::{
        keyboard::handle_keyboard_events, mouse::track_mouse_position, zoom::zoom
    }
};

use sdl2::{
    event::{Event, WindowEvent},
    keyboard::Keycode, mouse::MouseState,
};

#[derive(Debug)]
enum EventType {
    Quit,
    KeyDown(Keycode),
    MouseWheel(i32),
    MouseMotion(i32, i32, MouseState),
    FocusGained,
    Etc,
}

impl EventType {
    fn from_sdl2_event(event: Event) -> EventType {
        match event {
            Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => EventType::Quit,
            Event::KeyDown { keycode: Some(keycode), .. } => EventType::KeyDown(keycode),
            Event::MouseWheel { y, .. } if y != 0 => EventType::MouseWheel(y),
            Event::MouseMotion { xrel, yrel, mousestate, .. } => EventType::MouseMotion(xrel, yrel, mousestate),
            Event::Window { win_event: WindowEvent::FocusGained, .. } => EventType::FocusGained,
            _ => EventType::Etc,
        }
    }
}

pub(crate) fn handle_events(event: Event, program_runtime: &mut ProgramRuntime, mouse_x: f32, mouse_y: f32) {
    let event_type = EventType::from_sdl2_event(event);

    match event_type {
        EventType::Quit => {
            program_runtime.set_state_close();
            return;
        },
        EventType::KeyDown(keycode) => {
            handle_keyboard_events(&mut program_runtime.canvas.props, keycode, mouse_x, mouse_y);
        }
        EventType::MouseWheel(y) => {
            zoom(&mut program_runtime.canvas.props, mouse_x, mouse_y, y);
        }
        EventType::MouseMotion(xrel, yrel, mousestate) => {
            track_mouse_position(&mut program_runtime.canvas.props, xrel, yrel, mousestate);
        }
        EventType::FocusGained => {
            program_runtime.set_state_force_update();
            return;
        },
        EventType::Etc => (),
    }

    program_runtime.set_state_continue();
}
