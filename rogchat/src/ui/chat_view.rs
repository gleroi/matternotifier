use glib;
use glib::subclass;
use glib::subclass::prelude::*;
use glib::translate::*;
use gtk;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{ContainerExt};
use std::cell::RefCell;

glib::glib_wrapper! {
    pub struct ChatView(
        Object<subclass::simple::InstanceStruct<ChatViewPriv>,
        subclass::simple::ClassStruct<ChatViewPriv>,
        ChatViewClass>)
        @extends gtk::Widget, gtk::Container, gtk::Bin;
    match fn {
        get_type => || ChatViewPriv::get_type().to_glib(),
    }
}

impl ChatView {
    pub fn new() -> ChatView {
        glib::Object::new(Self::static_type(), &[])
            .expect("failed to create ChatView")
            .downcast()
            .expect("created ChatView is of wrong type")
    }
}

pub trait ChatViewExt {
    fn set_buffer(&self, buffer: &gtk::TextBuffer, title: &str);
}

impl ChatViewExt for ChatView {

    fn set_buffer(&self, buffer: &gtk::TextBuffer, title: &str) {
        let priv_ = ChatViewPriv::from_instance(self);

        let text = priv_.get_text_view().unwrap();
        text.set_buffer(Some(buffer));
        let label = priv_.get_title().unwrap();
        label.set_label(title);
    }
}

pub struct ChatViewPriv {
    text_view: RefCell<Option<gtk::TextView>>,
    title: RefCell<Option<gtk::Label>>,
}

impl ChatViewPriv {
    fn set_text_view(&self, tv: Option<gtk::TextView>) {
        let mut tv_ref = self.text_view.borrow_mut();
        *tv_ref = tv;
    }
    fn get_text_view(&self) -> Option<gtk::TextView> {
        match *self.text_view.borrow() {
            Some(ref tv) => Some(tv.clone()),
            None => None
        }
    }

    fn set_title(&self, label: Option<gtk::Label>) {
        let mut title_ref = self.title.borrow_mut();
        *title_ref = label;
    }
    fn get_title(&self) -> Option<gtk::Label> {
        match *self.title.borrow() {
            Some(ref tv) => Some(tv.clone()),
            None => None
        }
    }
}

static PROPERTIES : [subclass::Property; 0] = [];

impl ObjectSubclass for ChatViewPriv {
    const NAME: &'static str = "ChatView";
    type ParentType = gtk::Bin;
    type Instance = subclass::simple::InstanceStruct<Self>;
    type Class = subclass::simple::ClassStruct<Self>;

    glib::glib_object_subclass!();

    fn class_init(klass: &mut Self::Class) {
        klass.install_properties(&PROPERTIES);
    }

    fn new() -> Self {
        Self {
            text_view: RefCell::new(None),
            title: RefCell::new(None)
        }
    }
}

impl ObjectImpl for ChatViewPriv {
    glib::glib_object_impl!();

    fn constructed(&self, obj: &glib::Object) {
        self.parent_constructed(obj);
        let v = gtk::TextView::new();
        v.set_wrap_mode(gtk::WrapMode::Word);
        v.set_cursor_visible(false);
        v.set_editable(false);
        v.set_pixels_below_lines(5);
        v.set_left_margin(3);
        self.set_text_view(Some(v.clone()));

        let window = gtk::ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
        window.add(&v);

        let label = gtk::Label::new(Some(""));
        label.set_justify(gtk::Justification::Left);
        self.set_title(Some(label.clone()));

        let pane = gtk::Box::new(gtk::Orientation::Vertical, 0);
        pane.pack_start(&label, false, false, 0);
        pane.pack_start(&window, true, true, 0);

        let self_ = obj.downcast_ref::<ChatView>().unwrap();
        self_.add(&pane);
    }
}


impl BinImpl for ChatViewPriv {}
impl ContainerImpl for ChatViewPriv {}
impl WidgetImpl for ChatViewPriv {}
