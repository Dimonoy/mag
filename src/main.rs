mod event_loop;
mod system_tray;
mod screenshot;
mod utils;
mod window;
mod app;
mod canvas;
mod events;
mod texture;

use app::run_app;
use system_tray::run_tray_icon;
use device_query::{DeviceQuery, DeviceState, Keycode};

fn main() {
    { // Tray Application
        run_tray_icon();
    }

    run_keyboard_listener();
}

fn run_keyboard_listener() {
    let device_state = DeviceState::new();

    loop {
        let keys: Vec<Keycode> = device_state.get_keys();

        if keys.contains(&Keycode::LControl) && keys.contains(&Keycode::Z) {
            let program = std::thread::spawn(|| {
                if let Err(e) = run_app() {
                    log::error!("Magnifier app failed to boot: {}", e);
                    panic!("Magnifier app failed to boot");
                }
            });
            let _ = program.join();
        }

        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}
