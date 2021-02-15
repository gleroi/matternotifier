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

pub struct User {
    pub id: String,
    pub name: String,
}

trait Ui {
    fn create_channel(id: String, name: String, parent: String, users: Vec<User>);
}

// should go to ui probably
pub struct Buffer {
    channel: Channel,
    users: Vec<User>,
    messages: Vec<Message>,
}
