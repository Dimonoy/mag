pub enum UserEvent {
    TrayIconEvent(tray_icon::TrayIconEvent),
    TrayIconMenuEvent(tray_icon::menu::MenuEvent)
}
