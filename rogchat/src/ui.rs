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
        let g1 = create_grid("Coucou");
        let g2 = create_grid("Hello");
        notebook.add(&g1);
        notebook.set_tab_label_text(&g1, "grille 1");
        notebook.add(&g2);
        notebook.set_tab_label_text(&g2, "grille 2");
        window.add(&notebook);
        window.show_all();
    });

    Ok(application)
}

fn create_grid(title: &str) -> gtk::ScrolledWindow {
    let v = gtk::TextView::new();
    let window = ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
    window.add(&v);
    window
}
