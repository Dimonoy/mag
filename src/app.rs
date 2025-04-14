use crate::handlers::keyboard_handlers::handle_raw_keyboard_events;
use crate::renderer::Renderer;
use crate::screenshot::Screenshot;
use crate::user_event::UserEvent;

use winit::application::ApplicationHandler;
use winit::event::DeviceEvent::Key;
use winit::event::{RawKeyEvent, WindowEvent};
use winit::window::{Window, WindowId};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::PhysicalKey;

#[derive(Default)]
pub(crate) struct App {
    window: Option<Window>,
    screenshot: Option<Screenshot>,
    is_redraw_ready: bool,
}

impl ApplicationHandler<UserEvent> for App {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {}

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        self.is_redraw_ready = request_redraw(self.window.as_ref(), self.is_redraw_ready);
    }

    fn device_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        match event {
            Key(RawKeyEvent { physical_key: PhysicalKey::Code(key_code), state }) =>
                handle_raw_keyboard_events(
                    key_code,
                    state,
                    event_loop,
                    &mut self.window,
                    &mut self.screenshot,
                    &mut self.is_redraw_ready
                ),
            _ => (),
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: UserEvent) {
        match event {
            UserEvent::TrayIconEvent(event) => {
                println!("Simple {:?}", event);
            },
            UserEvent::TrayIconMenuEvent(event) => {
                println!("Menu {:?}", event);
            },
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::Resized(physical_size) => {},
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::KeyboardInput { device_id, event, is_synthetic } => {
                println!("{:?} PhysicalKey: {:?} LogicalKey: {:?} State: {:?} Is Synthetic: {}",
                    device_id,
                    event.physical_key,
                    event.logical_key,
                    event.state,
                    is_synthetic);
            },
            WindowEvent::RedrawRequested => {
                redraw(self.window.as_ref(), self.screenshot.as_ref());
            },
            _ => (),
        }
    }
}

fn redraw(window: Option<&Window>, screenshot: Option<&Screenshot>) {
    let window = match window {
        Some(w) => w,
        None => return
    }; // solution against race condition
    let screenshot = match screenshot {
        Some(s) => s,
        None => return
    }; // solution against race condition

    let mut renderer = Renderer::new(window);

    renderer.render_screenshot(screenshot);
    window.set_visible(true);
}

fn request_redraw(window: Option<&Window>, is_redraw_ready: bool) -> bool {
    if let Some(window_ref) = window {
        if is_redraw_ready {
            window_ref.request_redraw();
            return false;
        }
    }
    true
}
