use anyhow::Result;
use std::thread;
use std::process;
use gtk;
use gtk::{WidgetExt, TextBufferExt};
use glib;

mod core;
mod mattermost;
mod ui;

fn main() -> Result<()> {
    if gtk::init().is_err() {
        eprintln!("failed to initialize GTK");
        process::exit(1);
    }

    let (ui_tx, ui_rx) = glib::MainContext::channel(glib::PRIORITY_DEFAULT);
    let plugin = mattermost::Plugin::init(ui_tx.clone())?;
    let plugin_thread = thread::spawn(move || {
        plugin.run().unwrap();
    });

    let app = ui::build()?;
    let buffer = app.buffer;
    ui_rx.attach(None, move |m| {
        match m {
            core::Event::Message(str) => buffer.insert_at_cursor(&format!("{}\n", str)),
            core::Event::Info(str) => buffer.insert_at_cursor(&format!("{}\n", str)),
        };
        glib::source::Continue(true)
    });

    app.window.show_all();
    gtk::main();
    Ok(())
}
