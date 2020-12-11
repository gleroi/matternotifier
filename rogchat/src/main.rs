use anyhow::Result;
use std::thread;
use gio::prelude::*;
use gtk;
use glib;

mod core;
mod mattermost;
mod ui;

fn main() -> Result<()> {
    let application = gtk::Application::new(
        Some("re.leroi.rogchat"),
        Default::default(),
    )
    .expect("Initialization failed...");


    application.connect_activate(|app| {
        let (ui_tx, ui_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        let plugin = mattermost::Plugin::init(ui_tx.clone()).unwrap();
        let plugin_thread = thread::spawn(move || {
            plugin.run().unwrap();
        });
        ui::build(app, ui_rx);
    });
    
    application.run(&[]);
    Ok(())
}
