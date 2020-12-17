use super::core;
use chrono::NaiveDateTime;
use glib;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, TextBuffer, TextBufferExt, WidgetExt};
use gtk::{Notebook, ScrolledWindow, TextTag, TextTagExt, TextTagTable, TextTagTableExt};
use gtk::{PanedExt};
use pango;

pub fn build(app: &Application, ui_rx: core::Receiver) {
    let window = ApplicationWindow::new(app);
    window.set_title("rogchat");

    let css = gtk::CssProvider::new();
    css.load_from_data(
        br#"
            textview text {
                background-color: #eeeeee;
                color: #232335;
            }
        "#,
    )
    .unwrap();
    gtk::StyleContext::add_provider_for_screen(
        &window.get_screen().unwrap(),
        &css,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );

    let notebook = Notebook::new();
    let textbuffer = add_chat(&notebook, "all");

    let tree = gtk::TreeStore::new(&[String::static_type()]);
    let root = tree.append(None);
    tree.set_value(&root, 0, &glib::Value::from("enedis"));
    for chan in &["zmaster", "zsev2", "suivi mep", "suivi prod", "linkypilot"] {
        let child = tree.append(Some(&root));
        tree.set_value(&child, 0, &glib::Value::from(chan));
    }

    let treeview = gtk::TreeView::with_model(&tree);
    let col = gtk::TreeViewColumn::new();
    let cell = gtk::CellRendererText::new();
    col.pack_start(&cell, true);
    col.add_attribute(&cell, "text", 0);
    treeview.append_column(&col);

    let pane = gtk::Paned::new(gtk::Orientation::Horizontal);
    pane.pack1(&treeview, false, false);
    pane.pack2(&notebook, true, true);
    window.add(&pane);

    let buffer = textbuffer;
    ui_rx.attach(None, move |m| {
        match m {
            core::Event::Message(msg) => {
                insert_with_tag(
                    &buffer,
                    "msg",
                    &format!(
                        "{} {} > {} : {}\n",
                        NaiveDateTime::from_timestamp(msg.timestamp / 1000, 0).format("%X"),
                        msg.channel_name,
                        msg.sender_name,
                        msg.content
                    ),
                );
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
    v.set_left_margin(3);

    let window = ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    window.add(&v);
    notebook.add(&window);
    notebook.set_tab_label_text(&window, title);
    buffer
}
