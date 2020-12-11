
#[derive(Debug)]
pub enum Event {
    Info(String),
    Message(String),
}

pub type Sender = glib::Sender<Event>;
