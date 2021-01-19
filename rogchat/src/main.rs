use anyhow::Result;
use gio::prelude::*;
use glib;
use gtk;
use std::thread;

mod core;
mod mattermost;
mod ui;
mod config;

fn main() -> Result<()> {
    let application = gtk::Application::new(Some("re.leroi.rogchat"), Default::default())
        .expect("Initialization failed...");


    application.connect_activate(|app| {
        let (ui_tx, ui_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
        ui::build(app, ui_rx);

        let cfg = config::read_file("rogchat.toml").expect("an error occured while reading configuration");
        for server in &cfg.servers {
            if server.plugin.name == "mattermost" {
                let plugin = mattermost::Plugin::init(ui_tx.clone(), server.clone()).unwrap();
                let _plugin_thread = thread::spawn(move || {
                    plugin.run().unwrap();
                });
            }
        }
    });

    application.run(&[]);
    Ok(())
}
