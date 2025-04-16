use tray_icon::menu::{Menu, MenuItem};
use tray_icon::{TrayIcon, TrayIconBuilder};

pub enum TrayIconEvents {
    Event(tray_icon::TrayIconEvent),
    Menu(tray_icon::menu::MenuEvent)
}

fn load_icon(path: &std::path::Path) -> tray_icon::Icon {
    let (icon_rgba, icon_width, icon_height) = {
        let image = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();
        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon!")
}

fn create_tray_icon_menu() -> Menu {
    let menu = Menu::new();
    let item1 = MenuItem::new("Item 1", true, None);
    if let Err(err) = menu.append(&item1) {
        eprintln!("{err:?}");
    }

    menu
}

fn create_and_run_tray_icon_app() -> TrayIcon {
    let menu = create_tray_icon_menu();
    let tray_app = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_icon(load_icon(std::path::Path::new("./assets/logo-32x32.png")))
        .with_tooltip("Mag")
        .build()
        .unwrap();

    tray_app
}

// pub fn setup_tray_user_event_proxies(event_loop: &winit::event_loop::EventLoop<TrayIconEvents>) {
//     let proxy = event_loop.create_proxy();
//     tray_icon::TrayIconEvent::set_event_handler(Some(move |event| {
//         let _ = proxy.send_event(TrayIconEvents::Event(event));
//     }));
//
//     let proxy = event_loop.create_proxy();
//     tray_icon::menu::MenuEvent::set_event_handler(Some(move |event| {
//         let _ = proxy.send_event(TrayIconEvents::Menu(event));
//     }));
// }

pub fn run_tray_icon() {
    #[cfg(target_os = "linux")]
    std::thread::spawn(|| {
        gtk::init().unwrap();

        let _tray_app = create_and_run_tray_icon_app();

        gtk::main();
    });

    #[cfg(not(target_os = "linux"))]
    let _tray_app = create_and_run_tray_icon_app();
}
