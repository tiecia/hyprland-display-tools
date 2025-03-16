use hyprland::event_listener::{EventListener, MonitorAddedEventData};

fn main() -> hyprland::Result<()> {
    let mut event_listener = EventListener::new();

    event_listener.add_monitor_added_handler(on_monitor_added);
    event_listener.add_monitor_removed_handler(on_monitor_removed);

    event_listener
        .start_listener()
        .expect("Unable to listen for events.");

    Ok(())
}

fn on_monitor_added(data: MonitorAddedEventData) {
    let monitor_id = data.id;
    let monitor_description = data.description;
    let monitor_name = data.name;
    println!("Monitor connected: {monitor_name}, {monitor_id}, {monitor_description}")
}

fn on_monitor_removed(monitor_name: String) {
    println!("Monitor disconnected: {monitor_name}")
}
