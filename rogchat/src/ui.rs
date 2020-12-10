use anyhow::{Result};
use gtk::prelude::*;
use gtk::{Window, WindowType, Notebook, ScrolledWindow, TextTagTable};

pub struct RogChat {
    pub window: Window,
    pub buffer: gtk::TextBuffer,
}

pub fn build() -> Result<RogChat> {
    let window = Window::new(WindowType::Toplevel);
    window.set_title("rogchat");
    window.set_default_size(350, 70);

    let notebook = Notebook::new();
    let textbuffer = add_chat(&notebook, "all");
    window.add(&notebook);
    window.connect_delete_event(move |_,_| {
        gtk::main_quit();
        Inhibit(false)
    });
    Ok(RogChat {
        window: window,
        buffer: textbuffer,
    })
}

fn add_chat(notebook: &Notebook, title: &str) -> gtk::TextBuffer {
    let buffer = gtk::TextBuffer::new(None::<&TextTagTable>);
    let v = gtk::TextView::with_buffer(&buffer);
    let window = ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    window.add(&v);
    notebook.add(&window);
    notebook.set_tab_label_text(&window, title);
    buffer
}
