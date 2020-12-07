use anyhow::{Result};
use gio::prelude::*;
use gio::ListStore;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button, Grid, Label, ListBox, Notebook, ScrolledWindow};

pub fn build() -> Result<Application> {
    let application =
        Application::new(Some("re.leroi.rogchat"), Default::default())
            .expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("rogchat");
        window.set_default_size(350, 70);

        let notebook = Notebook::new();
        add_chat(&notebook, "Coucou");
        add_chat(&notebook, "Hello");
        window.add(&notebook);
        window.show_all();
    });

    Ok(application)
}

fn add_chat(notebook: &Notebook, title: &str) {
    let v = gtk::TextView::new();
    let window = ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    window.add(&v);
    notebook.add(&window);
    notebook.set_tab_label_text(&window, title);
}
