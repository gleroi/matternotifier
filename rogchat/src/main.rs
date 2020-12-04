use anyhow::Result;
use std::thread;
use std::sync::mpsc;

mod core;
mod mattermost;
mod ui;

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel::<core::Event>();
    let plugin = mattermost::Plugin::init(tx.clone())?;
    let plugin_thread = thread::spawn(move || {
        plugin.run().unwrap();
    });
    let core_thread = thread::spawn(move || {
        loop {
            let m = rx.recv().unwrap();
            dbg!(m);
        }
    });

    ui::test_gtk();
    Ok(())
}
