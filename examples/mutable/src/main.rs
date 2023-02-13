use dispatcher::{Dispatcher, LocalDispatcher, MessageType, Observer};
use std::cell::RefCell;

#[derive(Default)]
struct Message {
    pub value: i32,
    pub message_type: String,
}

#[derive(Default)]
struct Data {
    value: i32,
}

#[derive(Default)]
struct Container {
    data: RefCell<Data>,
}

struct ContainerUpdate<'a> {
    container: &'a Container,
}

struct ContainerPrint<'a> {
    container: &'a Container,
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

impl Container {
    pub fn register<'a>(&'a self, dispatcher: &mut dyn Dispatcher<'a, Message>) {
        dispatcher.register_handler(
            "update",
            Box::new(ContainerUpdate { container: self }),
            "tag1",
        );
        dispatcher.register_handler(
            "print",
            Box::new(ContainerPrint { container: self }),
            "tag2",
        );
    }

    pub fn value(&self) -> i32 {
        self.data.borrow().value
    }

    pub fn set_value(&self, value: i32) {
        self.data.borrow_mut().value = value;
    }
}

impl Observer<Message> for Container {
    fn handle(&self, message: &Message) {
        let mut data = self.data.borrow_mut();
        data.value = message.value;
    }
}

impl<'a> Observer<Message> for ContainerUpdate<'a> {
    fn handle(&self, message: &Message) {
        self.container.set_value(message.value);
    }
}

impl<'a> Observer<Message> for ContainerPrint<'a> {
    fn handle(&self, _: &Message) {
        println!("Message: {}", self.container.value());
    }
}

fn main() {
    let container = Container::default();
    let mut dispatcher = LocalDispatcher::<'_, Message>::default();
    container.register(&mut dispatcher);
    dispatcher.dispatch(&Message::update(42));
    dispatcher.dispatch(&Message::print());
    dispatcher.dispatch(&Message::update(55));
    dispatcher.dispatch(&Message::print());
}
