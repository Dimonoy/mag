use crate::{
    canvas::AppCanvasProps,
    utils::{
        mouse::track_left_click_holded_mouse,
        zoom::zoom,
    },
};

use sdl2::{
    event::Event,
    keyboard::Keycode, mouse::MouseState,
};

pub(crate) enum LoopState {
    Exit,
    Continue,
}

enum EventType {
    Quit,
    KeyDown(Keycode),
    MouseWheel(i32),
    MouseMotion(i32, i32, MouseState),
    Etc,
}

impl EventType {
    fn from_sdl2_event(event: Event) -> EventType {
        match event {
            Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => EventType::Quit,
            Event::KeyDown { keycode: Some(keycode), .. } => EventType::KeyDown(keycode),
            Event::MouseWheel { y, .. } if y != 0 => EventType::MouseWheel(y),
            Event::MouseMotion { xrel, yrel, mousestate, .. } => EventType::MouseMotion(xrel, yrel, mousestate),
            _ => EventType::Etc,
        }
    }
}

pub(crate) fn handle_events(event: Event, canvas_props: &mut AppCanvasProps, mouse_x: f32, mouse_y: f32) -> LoopState {
    let event_type = EventType::from_sdl2_event(event);

    match event_type {
        EventType::Quit => return LoopState::Exit,
        EventType::KeyDown(keycode) => println!("{:?}", keycode),
        EventType::MouseWheel(y) => zoom(canvas_props, mouse_x, mouse_y, y),
        EventType::MouseMotion(xrel, yrel, mousestate) => track_left_click_holded_mouse(canvas_props, xrel, yrel, mousestate),
        EventType::Etc => (),
    }

    LoopState::Continue
}
