mod system_tray;
mod screenshot;
mod utils;
mod window;
mod app;
mod canvas;
mod events;
mod texture;

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use app::run_app;
use system_tray::run_tray_icon;
use device_query::{DeviceQuery, DeviceState, Keycode};

fn main() -> Result<(), String> {
    { // Tray Application
        run_tray_icon();
    }

    run_keyboard_listener()?;

    Ok(())
}

fn run_keyboard_listener() -> Result<(), String> {
    let device_state = DeviceState::new();
    let is_app_running = Arc::new(AtomicBool::new(false));

    loop {
        let keys: Vec<Keycode> = device_state.get_keys();

        if keys.contains(&Keycode::LControl) && keys.contains(&Keycode::Z) {
            if !is_app_running.load(Ordering::SeqCst) {
                is_app_running.store(true, Ordering::SeqCst);

                let is_app_running_clone = Arc::clone(&is_app_running);
                std::thread::spawn(move || {
                    if let Err(e) = run_app() {
                        log::error!("Magnifier app failed to boot: {}", e);
                        panic!("Magnifier app failed to boot");
                    } else {
                        is_app_running_clone.store(false, Ordering::SeqCst);
                    }
                });
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(50));
    }
}
