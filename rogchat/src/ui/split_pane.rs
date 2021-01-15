use glib;
use glib::subclass;
use glib::subclass::prelude::*;
use glib::translate::*;
use gtk;
use gtk::prelude::*;
use gtk::subclass::prelude::*;


glib::glib_wrapper! {
	pub struct SplitPane(
		Object<subclass::simple::InstanceStruct<SplitPanePriv>,
		subclass::simple::ClassStruct<SplitPanePriv>,
		SplitPaneClass>)
		@extends gtk::Widget, gtk::Container;

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
}

pub struct SplitPanePriv {
}

static PROPERTIES: [subclass::Property; 0] = [];

impl ObjectSubclass for SplitPanePriv {
    const NAME: &'static str = "SplitPane";
    type ParentType = gtk::Container;
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

impl ObjectImpl for SplitPanePriv {
    glib::glib_object_impl!();

    fn constructed(&self, obj: &glib::Object) {
        self.parent_constructed(obj);
    }
}


impl BoxImpl for SplitPanePriv {}
impl ContainerImpl for SplitPanePriv {}
impl WidgetImpl for SplitPanePriv {}
