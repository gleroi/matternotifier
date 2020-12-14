use anyhow::Result;
use gio::prelude::*;
use glib;
use gtk;
use std::thread;

mod core;
mod mattermost;
mod ui;

fn main() -> Result<()> {
    let application = gtk::Application::new(Some("re.leroi.rogchat"), Default::default())
        .expect("Initialization failed...");

    application.connect_activate(|app| {
        let (ui_tx, ui_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        ui::build(app, ui_rx);
        let plugin = mattermost::Plugin::init(ui_tx.clone()).unwrap();
        let _plugin_thread = thread::spawn(move || {
            plugin.run().unwrap();
        });
    });

    application.run(&[]);
    Ok(())
}
