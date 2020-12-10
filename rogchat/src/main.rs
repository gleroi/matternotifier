use anyhow::Result;
use std::thread;
use std::sync::mpsc;
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

    let (tx, rx) = mpsc::channel::<core::Event>();
    let plugin = mattermost::Plugin::init(tx.clone())?;
    let plugin_thread = thread::spawn(move || {
        plugin.run().unwrap();
    });

    let app = ui::build()?;

    let core_thread = thread::spawn(move || {
        loop {
            let m = rx.recv().unwrap();
            dbg!(m);
            match m {
                core::Event::Message(str) => glib::idle_add_local(|| {
                    app.buffer.insert_at_cursor(&str);
                    glib::source::Continue(false)
                }) ,
            };
        }
    });

    app.window.show_all();
    gtk::main();
    Ok(())
}
