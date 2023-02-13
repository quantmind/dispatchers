use dispatchers::*;



#[derive(Default)]
struct Message {
    pub value: i32,
    pub message_type: String,
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


#[test]
fn simple_dispatcher() {
    let mut dispatcher = LocalDispatcher::<'_, Message>::default();
}
