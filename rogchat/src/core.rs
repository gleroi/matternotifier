#[derive(Debug)]
pub enum Event {
    Info(String),
    Message(Message),
    NewChannel(Channel),
}

#[derive(Debug)]
pub struct Message {
    pub timestamp: i64,
    pub channel_id: String,
    pub channel_name: String,
    pub sender_name: String,
    pub content: String,
}

#[derive(Debug)]
pub struct Channel {
    pub name: String,
    pub id: String,
    pub parent_id: Option<String>,
}

pub type Sender = glib::Sender<Event>;
pub type Receiver = glib::Receiver<Event>;
