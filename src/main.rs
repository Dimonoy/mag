mod event_loop;
mod system_tray;
mod screenshot;
mod utils;
mod window;
mod app;
mod canvas;
mod events;
mod texture;
mod program_state;

use app::run_app;
use program_state::ProgramExitType;
use system_tray::{is_systray_menu_quit_clicked, SystemTray};
use device_query::{DeviceQuery, DeviceState, Keycode};
use tray_icon::menu::MenuEventReceiver;

fn main() {
    #[cfg(feature = "dev")]
    env_logger::init();

    run_keyboard_listener();
}

fn run_keyboard_listener() {
    let systray_menu_events_receiver: &MenuEventReceiver = SystemTray::default().run_tray_icon();
    let device_state = DeviceState::new();

    loop {
        let keys: Vec<Keycode> = device_state.get_keys();

        if keys.contains(&Keycode::LControl) && keys.contains(&Keycode::LShift) && keys.contains(&Keycode::Z) {
            let systray_menu_events_receiver_clone = systray_menu_events_receiver.clone();
            let program = std::thread::spawn(move || {
                match run_app(&systray_menu_events_receiver_clone) {
                    Ok(event_loop_exit_status) => event_loop_exit_status,
                    Err(e) => {
                        #[cfg(feature = "dev")]
                        log::error!("Magnifier app failed to boot: {}", e);
                        panic!("Magnifier app failed to boot: {}", e);
                    }
                }
            });

            if let ProgramExitType::Exit = program.join().unwrap() {
                break;
            }
        }

        if is_systray_menu_quit_clicked(systray_menu_events_receiver) {
            break;
        }

        std::thread::sleep(std::time::Duration::from_millis(20));
    }
}
