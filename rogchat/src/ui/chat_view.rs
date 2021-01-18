use glib;
use glib::subclass;
use glib::subclass::prelude::*;
use glib::translate::*;
use gtk;
use gtk::prelude::*;
use gtk::subclass::prelude::*;

use gtk::{ContainerExt};

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
    fn set_buffer(&self, buffer: &gtk::TextBuffer);
}

impl ChatViewExt for ChatView {

    fn set_buffer(&self, buffer: &gtk::TextBuffer) {
        let text = self.get_child().unwrap()
            .downcast::<gtk::ScrolledWindow>().expect("ChatView expected a scrolledwindow")
            .get_child().unwrap().downcast::<gtk::TextView>().expect("ChatView expected a TextView");
        text.set_buffer(Some(buffer));
    }
}

pub struct ChatViewPriv {

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
        Self {}
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
        let window = gtk::ScrolledWindow::new(None::<&gtk::Adjustment>, None::<&gtk::Adjustment>);
        window.add(&v);
        let self_ = obj.downcast_ref::<ChatView>().unwrap();
        self_.add(&window);
    }
}


impl BinImpl for ChatViewPriv {}
impl ContainerImpl for ChatViewPriv {}
impl WidgetImpl for ChatViewPriv {}
