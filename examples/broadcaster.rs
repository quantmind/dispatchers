use dispatchers::{Broadcaster, MessageType};
use tokio;

#[derive(Default, Debug, Clone)]
struct Message {
    pub value: i32,
    pub message_type: String,
}

#[derive(Default)]
struct Data {
    value: i32,
}

impl MessageType for Message {
    fn message_type(&self) -> &str {
        &self.message_type
    }
}

impl Message {
    pub fn update(value: i32) -> Self {
        Self {
            value,
            message_type: "update".to_owned(),
        }
    }
    pub fn print() -> Self {
        Self {
            value: 0,
            message_type: "print".to_owned(),
        }
    }
}

#[tokio::main]
async fn main() {
    let dispatcher = Broadcaster::<Message>::default();
    let shared = dispatcher.sync_clone();
    tokio::spawn(async move {
        let mut receiver = shared.sender.subscribe();
        loop {
            match receiver.recv().await {
                Ok(message) => {
                    shared.dispatch(&message);
                }
                Err(err) => {
                    break;
                }
            };
        }
    });
}
