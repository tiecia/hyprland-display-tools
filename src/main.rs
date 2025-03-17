use hyprland::event_listener::{EventListener, MonitorAddedEventData};
use std::env;
use std::process::Command;

fn main() -> hyprland::Result<()> {
    let mut event_listener = EventListener::new();

    event_listener.add_monitor_added_handler(on_monitor_added);
    event_listener.add_monitor_removed_handler(on_monitor_removed);

    log_xdg_runtime_location();
    log_hyprland_signature();

    println!("Listening for events...");

    event_listener
        .start_listener()
        .expect("Unable to listen for events.");

    Ok(())
}

fn on_monitor_added(data: MonitorAddedEventData) {
    let monitor_id = data.id;
    let monitor_description = data.description;
    let monitor_name = data.name;
    println!("Monitor connected: {monitor_name}, {monitor_id}, {monitor_description}");

    reset_bar();
}

fn on_monitor_removed(monitor_name: String) {
    println!("Monitor disconnected: {monitor_name}");

    reset_bar();
}

fn reset_bar() {
    close_bar();
    open_bar();
}

fn close_bar() {
    Command::new("ags")
        .arg("-q")
        .status()
        .expect("Unable to close bar.");
}

fn open_bar() {
    Command::new("ags").spawn().expect("Unable to open bar.");
}

fn log_xdg_runtime_location() {
    env::var("XDG_RUNTIME_DIR")
        .map(|dir| println!("XDG_RUNTIME_DIR: {dir}"))
        .unwrap_or_else(|_| println!("XDG_RUNTIME_DIR is not set."));
}

fn log_hyprland_signature() {
    env::var("HYPRLAND_INSTANCE_SIGNATURE")
        .map(|dir| println!("HYPRLAND_INSTANCE_SIGNATURE: {dir}"))
        .unwrap_or_else(|_| println!("HYPRLAND_INSTANCE_SIGNATURE is not set."));
}
