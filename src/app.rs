use crate::actions::create_window;
use crate::handlers::keyboard_handlers::handle_raw_keyboard_events;
use crate::renderer::Renderer;
use crate::screenshot::Screenshot;
use crate::user_event::UserEvent;

use ouroboros::self_referencing;
use winit::application::ApplicationHandler;
use winit::event::DeviceEvent::Key;
use winit::event::{RawKeyEvent, WindowEvent};
use winit::window::{Window, WindowId};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::PhysicalKey;

#[derive(Default)]
pub(crate) struct App {
    window_state: Option<AppWindowState>,
    is_redraw_ready: bool,
}

#[self_referencing]
pub(crate) struct AppWindowState {
    window: Window,
    screenshot: Screenshot,
    #[borrows(window, mut screenshot)]
    #[not_covariant]
    renderer: Renderer<'this>,
}

impl AppWindowState {
    pub fn create(event_loop: &ActiveEventLoop) -> Self {
        let screenshot = Screenshot::capture();
        let window = create_window(event_loop);

        AppWindowState::new(window, screenshot, |window: &Window, screenshot: &mut Screenshot| {
            Renderer::new(window, screenshot)
        })
    }
}

impl ApplicationHandler<UserEvent> for App {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {}

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        request_redraw(self.window_state.as_ref(), &mut self.is_redraw_ready);
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
                    &mut self.window_state,
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
            WindowEvent::Resized(_physical_size) => {},
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
                redraw(self.window_state.as_mut());
            },
            _ => (),
        }
    }
}

fn redraw(window_state: Option<&mut AppWindowState>) {
    let window_state_ref = match window_state {
        Some(w) => w,
        None => return
    }; // solution against race condition

    window_state_ref.with_renderer_mut(|renderer| renderer.render());
    window_state_ref.with_window(|window| window.set_visible(true));
}

fn request_redraw(window_state: Option<&AppWindowState>, is_redraw_ready: &mut bool) {
    let window_state_ref = match window_state {
        Some(w) => w,
        None => {
            *is_redraw_ready = true;
            return;
        },
    };

    if *is_redraw_ready {
        window_state_ref.with_window(|window| window.request_redraw());
        *is_redraw_ready = false;
    }
}
