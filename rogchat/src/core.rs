

#[derive(Debug)]
pub enum Event {
    Message(String),
}

pub type Sender = std::sync::mpsc::Sender<Event>;
