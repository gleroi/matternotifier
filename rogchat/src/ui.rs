use super::core;
use glib;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, TextBuffer, TextBufferExt, WidgetExt};
use gtk::{Notebook, ScrolledWindow, TextTag, TextTagExt, TextTagTable, TextTagTableExt};
use pango;

pub fn build(app: &Application, ui_rx: core::Receiver) {
    let window = ApplicationWindow::new(app);
    window.set_title("rogchat");

    let notebook = Notebook::new();
    let textbuffer = add_chat(&notebook, "all");
    window.add(&notebook);

    let buffer = textbuffer;
    ui_rx.attach(None, move |m| {
        match m {
            core::Event::Message(str) => {
                insert_with_tag(&buffer, "msg", &format!("{}\n", str));
            }
            core::Event::Info(str) => {
                insert_with_tag(&buffer, "info", &format!("{}\n", str));
            }
        };
        glib::source::Continue(true)
    });
    window.show_all();
}

fn insert_with_tag(buffer: &TextBuffer, tag_name: &str, content: &str) {
    let mut end = buffer.get_end_iter();
    buffer.insert(&mut end, content);
    let mut start = end.clone();
    start.backward_chars(content.len() as i32);
    buffer.apply_tag_by_name(tag_name, &start, &end)
}

fn add_chat(notebook: &Notebook, title: &str) -> gtk::TextBuffer {
    let tags = TextTagTable::new();

    let info_tag = TextTag::new(Some("info"));
    info_tag.set_property_background(Some("#DDDDDD"));
    info_tag.set_property_style(pango::Style::Italic);
    tags.add(&info_tag);
    
    let msg_tag = TextTag::new(Some("msg"));
    msg_tag.set_property_background(Some("#e4eaf5"));
    tags.add(&msg_tag);

    let buffer = TextBuffer::new(Some(&tags));
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
