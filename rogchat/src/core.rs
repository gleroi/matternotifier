#[derive(Debug)]
pub enum Event {
    Info(String),
    Message(Message),
}

#[derive(Debug)]
pub struct Message {
    pub timestamp: i64,
    pub channel_name: String,
    pub sender_name: String,
    pub content: String,
}



pub type Sender = glib::Sender<Event>;
pub type Receiver = glib::Receiver<Event>;
