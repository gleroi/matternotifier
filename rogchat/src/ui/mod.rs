use super::core;
use chrono::NaiveDateTime;
use glib;
use glib::prelude::*;
use gtk::prelude::*;
use gtk::PanedExt;
use gtk::{Application, ApplicationWindow, TextBuffer, TextBufferExt, WidgetExt};
use gtk::{ScrolledWindow, TextTag, TextTagExt, TextTagTable, TextTagTableExt};
use pango;

mod chat_view;
mod split_pane;

use chat_view::{ChatView, ChatViewExt};
use split_pane::{SplitPane, SplitPaneExt};

pub fn build(app: &Application, ui_rx: core::Receiver) {
    let window = ApplicationWindow::new(app);
    window.set_title("rogchat");

    let chats_pane = SplitPane::new();

    let channel_tree = gtk::TreeStore::new(&[
        String::static_type(),
        String::static_type(),
        TextBuffer::static_type(),
    ]);
    append_channel(&channel_tree, None, "all", "");
    let channel_window = add_channel_list(&channel_tree, chats_pane.clone());

    let pane = gtk::Paned::new(gtk::Orientation::Horizontal);
    pane.pack1(&channel_window, true, false);
    pane.pack2(&chats_pane, true, true);
    window.add(&pane);

    chats_pane.grab_focus();

    ui_rx.attach(None, move |m| {
        match m {
            core::Event::Message(msg) => {
                let buffer = get_buffer_or_default(&channel_tree, &msg.channel_id);
                insert_with_tag(
                    &buffer,
                    "msg",
                    &format!(
                        "{} {} : {}\n",
                        NaiveDateTime::from_timestamp(msg.timestamp / 1000, 0).format("%X"),
                        msg.sender_name,
                        msg.content
                    ),
                );
            }
            core::Event::NewChannel(channel) => {
                let mut root = None;
                if let Some(parent_id) = channel.parent_id {
                    root = find_iter_by_channel_id(&channel_tree, &parent_id);
                }
                append_channel(&channel_tree, root.as_ref(), &channel.name, &channel.id);
            }
            core::Event::Info(str) => {
                let buffer = get_buffer_or_default(&channel_tree, "");
                insert_with_tag(&buffer, "info", &format!("{}\n", str));
            }
        };
        glib::source::Continue(true)
    });
    window.show_all();
    println!("windows type hint {:?}", window.get_type_hint());
}

fn append_channel(
    channel_tree: &gtk::TreeStore,
    root: Option<&gtk::TreeIter>,
    name: &str,
    id: &str,
) {
    let child = channel_tree.append(root);
    channel_tree.set_value(&child, 0, &glib::Value::from(name));
    channel_tree.set_value(&child, 1, &glib::Value::from(id));
    channel_tree.set_value(&child, 2, &glib::Value::from(&create_buffer()));
}

fn get_buffer_or_default(channel_tree: &gtk::TreeStore, id: &str) -> gtk::TextBuffer {
    if let Some(iter) = find_iter_by_channel_id(channel_tree, id) {
        return channel_tree
            .get_value(&iter, 2)
            .get::<TextBuffer>()
            .expect("expected a TextBuffer")
            .expect("unexpected None TextBuffer");
    }
    let iter = find_iter_by_channel_id(channel_tree, id).unwrap();
    channel_tree
        .get_value(&iter, 2)
        .get::<TextBuffer>()
        .expect("expected a TextBuffer")
        .expect("unexpected None TextBuffer")
}

fn find_iter_by_channel_id(
    channel_tree: &gtk::TreeStore,
    search_id: &str,
) -> Option<gtk::TreeIter> {
    let mut found_iter = None;
    channel_tree.foreach(|tree, _path, iter| {
        if let Some(id) = tree.get_value(iter, 1).get::<String>().unwrap() {
            if &id == search_id {
                found_iter = Some(iter.to_owned());
                return true;
            }
        }
        return false;
    });
    return found_iter;
}

fn insert_with_tag(buffer: &TextBuffer, tag_name: &str, content: &str) {
    let mut end = buffer.get_end_iter();
    buffer.insert(&mut end, content);
    let mut start = end.clone();
    start.backward_chars(content.len() as i32);
    buffer.apply_tag_by_name(tag_name, &start, &end)
}

fn add_channel_list(channel_tree: &gtk::TreeStore, chats_pane: SplitPane) -> gtk::ScrolledWindow {
    let channel_view = gtk::TreeView::with_model(channel_tree);
    channel_view.set_activate_on_single_click(true);
    let col = gtk::TreeViewColumn::new();
    col.set_title("channel");
    let cell = gtk::CellRendererText::new();
    col.pack_start(&cell, true);
    col.add_attribute(&cell, "text", 0);
    channel_view.append_column(&col);
    channel_view.connect_row_activated(move |view, path, _column| {
        let model = view.get_model().unwrap();
        if let Some(iter) = model.get_iter(path) {
            let buffer = model
                .get_value(&iter, 2)
                .get::<TextBuffer>()
                .unwrap()
                .unwrap();
            let title_value = model.get_value(&iter, 0);
            let title = title_value.get::<&str>().unwrap().unwrap();
            let chatview = chats_pane
                .get_active_pane()
                .unwrap()
                .downcast::<ChatView>()
                .unwrap();
            chatview.set_buffer(&buffer, title);
        }
    });

    let channel_window = ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    channel_window.add(&channel_view);
    channel_window
}

fn create_buffer() -> gtk::TextBuffer {
    let tags = TextTagTable::new();

    let info_tag = TextTag::new(Some("info"));
    info_tag.set_property_background(Some("#DDDDDD"));
    info_tag.set_property_style(pango::Style::Italic);
    tags.add(&info_tag);

    let msg_tag = TextTag::new(Some("msg"));
    msg_tag.set_property_background(Some("#e4eaf5"));
    tags.add(&msg_tag);

    TextBuffer::new(Some(&tags))
}
