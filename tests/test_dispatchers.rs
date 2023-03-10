use dispatchers::*;
use std::cell::RefCell;

#[derive(Default)]
struct Container {
    pub value: i32,
}

#[derive(Default, Clone, Debug)]
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

struct Handler<F>
where
    F: Fn(&Message),
{
    fun: F,
}

impl<F> Handler<F>
where
    F: Fn(&Message),
{
    pub fn new(fun: F) -> Box<Self> {
        Box::new(Self { fun })
    }
}

impl<F> Observer<Message> for Handler<F>
where
    F: Fn(&Message),
{
    fn call(&self, message: &Message) {
        (self.fun)(message)
    }
}

#[test]
fn local_dispatcher() {
    let container1 = RefCell::new(Container::default());
    let container2 = RefCell::new(Container::default());

    let mut dispatcher = LocalDispatcher::<Message>::default();
    let message = Message::update(1);
    assert_eq!(message.value, 1);
    assert_eq!(dispatcher.dispatch(&message).unwrap(), 0);
    assert_eq!(dispatcher.dispatch(&Message::print()).unwrap(), 0);

    dispatcher.register_handler(
        "update",
        Handler::new(|message: &Message| {
            container1.borrow_mut().value = message.value;
        }),
        "tag1",
    );

    assert_eq!(container1.borrow().value, 0);
    assert_eq!(dispatcher.dispatch(&message).unwrap(), 1);
    assert_eq!(container1.borrow().value, 1);

    dispatcher.register_handler(
        "update",
        Handler::new(|message: &Message| {
            container2.borrow_mut().value = 2 * message.value;
        }),
        "tag2",
    );

    assert_eq!(container2.borrow().value, 0);
    assert_eq!(dispatcher.dispatch(&Message::update(5)).unwrap(), 2);
    assert_eq!(container1.borrow().value, 5);
    assert_eq!(container2.borrow().value, 10);

    dispatcher.unregister_handler("update", "tag3");
    assert_eq!(dispatcher.dispatch(&Message::update(6)).unwrap(), 2);
    assert_eq!(container1.borrow().value, 6);
    assert_eq!(container2.borrow().value, 12);

    dispatcher.unregister_handler("update", "tag2");
    assert_eq!(dispatcher.dispatch(&Message::update(7)).unwrap(), 1);
    assert_eq!(container1.borrow().value, 7);
    assert_eq!(container2.borrow().value, 12);
}
