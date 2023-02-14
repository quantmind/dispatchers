use dispatchers::{Broadcaster, MessageType};
use tokio;

#[derive(Default, Clone)]
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
    let dispatcher = Broadcaster::<'_, Message>::default();
    let sync_dispatcher = dispatcher.sync_clone();
    tokio::spawn(async move {
        let d = sync_dispatcher.broadcaster();
        let mut receiver = sync_dispatcher.receiver();
        loop {
            match receiver.recv().await {
                Ok(message) => {
                    d.dispatch(&message).await;
                }
                Err(err) => break,
            }
        }
    });
}
