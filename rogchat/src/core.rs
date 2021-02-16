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

pub struct Buffer {
    channel: Channel,
    users: Vec<User>,
    messages: Vec<Message>,
}

// core provides 
// - buffers: to store messages, from a room/channel/conversation, and a list of participants
// - plugins: initialize and start plugins
// - config: manage configuration for itself and plugins
//
// plugins provides
// - protocols: translate a protocol (mattermost, irc,..) to core::Buffer
// - storage: store/log messages, query history ??
// - ui enhancement: ???
// - ...
//
// ui provides
// - a window with
//   - buffer list: plugin > group > .. > channel (buffer)
//   - user list: participant of the ui active channel
//   - buffers "zone":
//     - buffer display: display messages,
//     - can be split / tabbed (?)
//   - input field to send message or command to the plugin managing the current active buffer
//
