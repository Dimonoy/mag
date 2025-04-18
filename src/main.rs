mod system_tray;
mod screenshot;
mod utils;
mod window;
mod app;
mod canvas;

use std::sync::{Arc, Mutex};
use app::run_app;
use system_tray::run_tray_icon;
use device_query::{DeviceQuery, DeviceState, Keycode as DQKeycode};

fn main() -> Result<(), String> {
    { // Tray Application
        run_tray_icon();
    }

    run_keyboard_listener()?;

    Ok(())
}

fn run_keyboard_listener() -> Result<(), String> {
    let device_state = DeviceState::new();

    let is_app_running = Arc::new(Mutex::new(false));

    loop {
        let keys: Vec<DQKeycode> = device_state.get_keys();

        if keys.contains(&DQKeycode::LControl) && keys.contains(&DQKeycode::Z) && !*is_app_running.lock().unwrap() {
            *is_app_running.lock().unwrap() = true;

            let is_app_running_clone = Arc::clone(&is_app_running);
            std::thread::spawn(move || {
                if let Err(e) = run_app() {
                    log::error!("Magnifier app failed to boot: {}", e);
                    panic!("Magnifier app failed to boot");
                } else {
                    *is_app_running_clone.lock().unwrap() = false;
                }
            });
        }
    }
}
