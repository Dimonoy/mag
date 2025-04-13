mod user_event;
mod tray;
mod app;
mod screenshot;
mod renderer;

use crate::user_event::UserEvent;
use crate::app::App;

use tray::{run_tray_icon, setup_tray_user_event_proxies};
use winit::event_loop::EventLoop;

fn main() {
    let event_loop = EventLoop::<UserEvent>::with_user_event().build().expect("The Winit event loop f*cks with us -_-");
    event_loop.set_control_flow(winit::event_loop::ControlFlow::Wait);

    // Tray Application
    run_tray_icon();
    setup_tray_user_event_proxies(&event_loop);

    let mut app = App::default();
    let _ = event_loop.run_app(&mut app);
}
