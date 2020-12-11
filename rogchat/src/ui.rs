use anyhow::{Result};
use gio::prelude::*;
use gtk::prelude::*;
use gtk::{Window, WindowType, Notebook, ScrolledWindow, TextTagTable};
use gtk::{Application, ApplicationWindow, WidgetExt, TextBufferExt};
use glib;
use super::core;

pub struct RogChat {
    pub window: Window,
    pub buffer: gtk::TextBuffer,
}

pub fn build(app: &Application, ui_rx: core::Receiver) {
    let window = ApplicationWindow::new(app);
    window.set_title("rogchat");

    let notebook = Notebook::new();
    let textbuffer = add_chat(&notebook, "all");
    window.add(&notebook);

    let buffer = textbuffer; 
    ui_rx.attach(None, move |m| {
        match m {
            core::Event::Message(str) => buffer.insert(&mut buffer.get_end_iter(), &format!("{}\n", str)),
            core::Event::Info(str) => buffer.insert(&mut buffer.get_end_iter(), &format!("{}\n", str)),
        };
        glib::source::Continue(true)
    });
    window.show_all();
}

fn add_chat(notebook: &Notebook, title: &str) -> gtk::TextBuffer {
    let buffer = gtk::TextBuffer::new(None::<&TextTagTable>);
    let v = gtk::TextView::with_buffer(&buffer);
    v.set_wrap_mode(gtk::WrapMode::Word);
    v.set_cursor_visible(false);
    v.set_editable(false);
    v.set_pixels_below_lines(5);
    let window = ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    window.add(&v);
    notebook.add(&window);
    notebook.set_tab_label_text(&window, title);
    buffer
}
