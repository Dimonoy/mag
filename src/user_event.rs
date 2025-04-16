pub enum TrayIconEvents {
    Event(tray_icon::TrayIconEvent),
    Menu(tray_icon::menu::MenuEvent)
}
