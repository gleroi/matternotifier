use anyhow::Result;
use std::thread;
use std::sync::mpsc;

mod core;
mod mattermost;

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel::<core::Event>();
    let plugin = mattermost::Plugin::init(tx.clone())?;
    let thread = thread::spawn(move || {
        plugin.run().unwrap();
    });
    loop {
        let m = rx.recv()?;
        dbg!(m);
    }
}
