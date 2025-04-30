use tray_icon::menu::{Menu, MenuEvent, MenuEventReceiver, MenuId, MenuItem};
use tray_icon::{TrayIcon, TrayIconBuilder};

enum SystemTrayMenuItems {
    Quit,
}

impl SystemTrayMenuItems {
    fn from_menu_id(menu_id: &MenuId) -> Option<Self> {
        match menu_id.0.parse::<u32>().unwrap() {
            3 => Some(Self::Quit),
            _ => None,
        }
    }
}

#[derive(Debug, Default)]
pub(crate) struct SystemTray;

impl SystemTray {
    pub(crate) fn run_tray_icon<'a>(self) -> &'a MenuEventReceiver {
        #[cfg(target_os = "linux")]
        std::thread::spawn(move || {
            gtk::init().unwrap();

            let _tray_app = self.create_and_run_tray_icon_app();

            gtk::main();
        });

        #[cfg(not(target_os = "linux"))]
        let _tray_app = create_and_run_tray_icon_app();

        MenuEvent::receiver()
    }

    fn try_load_icon(&self, path: &std::path::Path) -> Result<tray_icon::Icon, String> {
        let image = match image::open(path) {
            Ok(image_result) => image_result.into_rgba8(),
            Err(_) => return Err(String::from("Failed to open icon path")),
        };
        let (icon_rgba, icon_width, icon_height) = {
            let (width, height) = image.dimensions();
            let rgba = image.into_raw();
            (rgba, width, height)
        };
        Ok(tray_icon::Icon::from_rgba(icon_rgba, icon_width, icon_height).expect("Failed to open icon!"))
    }

    fn create_tray_icon_menu(&self) -> Menu {
        let menu = Menu::new();
        let quit = MenuItem::new("Quit", true, None);
        if let Err(err) = menu.append(&quit) {
            eprintln!("{err:?}");
        }

        menu
    }

    fn create_and_run_tray_icon_app(&self) -> TrayIcon {
        let menu = self.create_tray_icon_menu();
        let mut tray_app = TrayIconBuilder::new()
            .with_menu(Box::new(menu))
            .with_tooltip("Mag");

        match self.try_load_icon(std::path::Path::new("./assets/logo-32x32.png")) {
            Ok(icon) => tray_app = tray_app.with_icon(icon),
            Err(error_message) => {
                #[cfg(feature = "dev")]
                log::warning!(error_message);
                ()
            },
        }

        tray_app.build().unwrap()
    }
}

/// Returns true if Quit was clicked in SysTray menu
pub(crate) fn is_systray_menu_quit_clicked(systray_menu_events_receiver: &MenuEventReceiver) -> bool {
    if let Ok(event) = systray_menu_events_receiver.try_recv() {
        if let Some(SystemTrayMenuItems::Quit) = SystemTrayMenuItems::from_menu_id(event.id()) {
            return true;
        }
    }

    false
}
