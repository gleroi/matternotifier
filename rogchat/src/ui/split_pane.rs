use super::chat_view::ChatView;
use glib;
use glib::subclass;
use glib::subclass::prelude::*;
use glib::translate::*;
use gtk;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use std::cell::RefCell;

glib::glib_wrapper! {
    pub struct SplitPane(
        Object<subclass::simple::InstanceStruct<SplitPanePriv>,
        subclass::simple::ClassStruct<SplitPanePriv>,
        SplitPaneClass>)
        @extends gtk::Widget, gtk::Container, gtk::Bin;

    match fn {
        get_type => || SplitPanePriv::get_type().to_glib(),
    }
}

impl SplitPane {
    pub fn new() -> SplitPane {
        glib::Object::new(Self::static_type(), &[])
            .expect("Failed to create SplitPane")
            .downcast()
            .expect("Created SplitPane is of wrong type")
    }

    fn remove_active_class(w: &gtk::Widget) {
        let context = w.get_style_context();
        context.remove_class("active");
    }

    fn add_active_class(w: &gtk::Widget) {
        let context = w.get_style_context();
        context.add_class("active");
    }
}

pub trait SplitPaneExt {
    fn set_active_pane(&self, w: Option<gtk::Widget>);
    fn get_active_pane(&self) -> Option<gtk::Widget>;
}

impl SplitPaneExt for SplitPane {
    fn set_active_pane(&self, w: Option<gtk::Widget>) {
        let priv_ = SplitPanePriv::from_instance(self);
        let previous = self.get_active_pane();
        if let Some(ref previous_widget) = previous {
            Self::remove_active_class(previous_widget);
        }
        if let Some(ref widget) = w {
            Self::add_active_class(widget);
        }
        priv_.set_active_pane(w);
    }

    fn get_active_pane(&self) -> Option<gtk::Widget> {
        let priv_ = SplitPanePriv::from_instance(self);
        let active_pane_ref = priv_.active_pane.borrow();
        active_pane_ref.clone()
    }
}

pub struct SplitPanePriv {
    active_pane: RefCell<Option<gtk::Widget>>,
}

impl SplitPanePriv {
    fn set_active_pane(&self, w: Option<gtk::Widget>) {
        let mut active_pane_ref = self.active_pane.borrow_mut();
        *active_pane_ref = w;
    }
}

static PROPERTIES: [subclass::Property; 1] = [subclass::Property("active-pane", |active_pane| {
    glib::ParamSpec::object(
        active_pane,
        "Last active pane of this container",
        "Last widget that has been clicked on",
        gtk::Widget::static_type(),
        glib::ParamFlags::READWRITE,
    )
})];

impl ObjectSubclass for SplitPanePriv {
    const NAME: &'static str = "SplitPane";
    type ParentType = gtk::Bin;
    type Instance = subclass::simple::InstanceStruct<Self>;
    type Class = subclass::simple::ClassStruct<Self>;

    glib::glib_object_subclass!();

    fn class_init(klass: &mut Self::Class) {
        klass.install_properties(&PROPERTIES);
    }

    fn new() -> Self {
        Self {
            active_pane: RefCell::new(None),
        }
    }
}

impl ObjectImpl for SplitPanePriv {
    glib::glib_object_impl!();

    //TODO:
    // - remplacer les Paned par des Box
    // - gerer le set_buffer et active ici plutot que dans la chatview pour mettre une mise en
    // valeur du chat actif
    fn constructed(&self, obj: &glib::Object) {
        self.parent_constructed(obj);

        let self_ = obj.downcast_ref::<SplitPane>().unwrap();
        let context = self_.get_style_context();
        let css = gtk::CssProvider::new();
        css.load_from_data(
            br#"
            .active {
                border-width: 3px;
                border-color: red;
            }
            .active label {
                font-weight: bold;
            }
        "#,
        )
        .unwrap();
        context.add_provider(&css, 1);

        let chats_pane = gtk::Paned::new(gtk::Orientation::Horizontal);
        let chat_view1 = ChatView::new();
        let self1 = self_.clone();
        chat_view1.connect_button_press_event(move |widget, _| {
            self1.set_active_pane(Some(widget.clone().upcast::<gtk::Widget>()));
            Inhibit(false)
        });
        chats_pane.pack1(&chat_view1, true, false);

        let chat_view2 = ChatView::new();
        let self2 = self_.clone();
        chat_view2.connect_button_press_event(move |widget, _| {
            self2.set_active_pane(Some(widget.clone().upcast::<gtk::Widget>()));
            Inhibit(false)
        });
        chats_pane.pack2(&chat_view2, true, false);

        self_.set_active_pane(Some(chat_view1.upcast::<gtk::Widget>()));
        self_.add(&chats_pane);
    }

    fn set_property(&self, _obj: &glib::Object, id: usize, value: &glib::Value) {
        let prop = &PROPERTIES[id];
        match *prop {
            subclass::Property("active-pane", ..) => {
                let active_pane = value
                    .get()
                    .expect("SplitPane::active-pane property expect a gtk::Widget");
                self.set_active_pane(active_pane);
            }
            _ => unimplemented!(),
        }
    }

    fn get_property(&self, _obj: &glib::Object, id: usize) -> Result<glib::Value, ()> {
        let prop = &PROPERTIES[id];
        match *prop {
            subclass::Property("active-pane", ..) => Ok(self.active_pane.borrow().to_value()),
            _ => unimplemented!(),
        }
    }
}

impl BinImpl for SplitPanePriv {}
impl ContainerImpl for SplitPanePriv {}
impl WidgetImpl for SplitPanePriv {}
