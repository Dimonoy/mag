use std::time::{Duration, Instant};

use crate::renderer::Renderer;
use crate::screenshot::Screenshot;
use crate::user_event::UserEvent;

use winit::application::ApplicationHandler;
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::DeviceEvent::Key;
use winit::event::{ElementState, RawKeyEvent, WindowEvent};
use winit::window::{Fullscreen, Window, WindowAttributes, WindowId};
use winit::event_loop::ActiveEventLoop;
use winit::keyboard::PhysicalKey;
use winit::keyboard::KeyCode;

#[derive(Default)]
pub struct App {
    window: Option<Window>,
    screenshot: Option<Screenshot>,
    is_redraw_ready: bool,
}

impl App {
    fn create_window(&mut self, event_loop: &ActiveEventLoop) {
        self.window = {
            let monitor = event_loop.primary_monitor().expect("SHIT");

            let window_attributes = WindowAttributes::default()
                .with_inner_size(monitor.size())
                .with_min_inner_size(monitor.size())
                .with_fullscreen(Some(Fullscreen::Borderless(Some(monitor))))
                .with_visible(false);

            Some(event_loop.create_window(window_attributes).expect("Event loop s*cks at creating windows :("))
        };
    }
}

impl ApplicationHandler<UserEvent> for App {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {}

    fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
        if let Some(window_ref) = self.window.as_ref() {
            if self.is_redraw_ready {
                window_ref.request_redraw();
                self.is_redraw_ready = false;
            }
        }
    }

    fn device_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        match event {
            Key(RawKeyEvent { physical_key: PhysicalKey::Code(key_code), state }) => match (key_code, state) {
                (KeyCode::F12, ElementState::Pressed) => {
                    if self.window.is_some() { return }

                    self.screenshot = Some(Screenshot::default());
                    self.screenshot.as_mut().map(|screenshot_ref| {
                        screenshot_ref.capture();
                    });
                    self.create_window(event_loop);
                    self.is_redraw_ready = true;
                },
                (KeyCode::Escape, ElementState::Pressed) => {
                    if self.window.is_none() { return }
                    self.window = None;
                    self.screenshot = None;
                    self.is_redraw_ready = true;
                },
                _ => (),
            }
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
                let window_ref = self.window.as_ref().expect("I am pretty sure the window is not created yet o^O");

                let mut renderer = Renderer::new(window_ref);

                renderer.render_screenshot(self.screenshot.as_ref().expect("Strange, I capture screen right before creating window -_-"));
                window_ref.set_visible(true);
            },
            _ => (),
        }
    }
}
