use dispatchers::{Broadcaster, MessageType};

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

fn main() {
    let mut dispatcher = Broadcaster::<'_, Message>::default();
}
