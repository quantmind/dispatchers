use dispatchers::{Broadcaster, Dispatcher, MessageType, Observer};
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

struct Handler<F>
where
    F: Fn(&Message) + Send,
{
    fun: F,
}

impl<F> Handler<F>
where
    F: Fn(&Message) + Send,
{
    pub fn new<'a>(fun: F) -> Box<Self> {
        Box::new(Self { fun })
    }
}

impl<'a, F> Observer<Message> for Handler<F>
where
    F: Fn(&Message) + Send + 'a,
{
    fn call(&self, message: &Message) {
        (self.fun)(message)
    }
}

#[tokio::main]
async fn main() {
    let dispatcher = Broadcaster::<Message>::default();
    let mut shared = dispatcher.sync_clone();
    tokio::spawn(async move {
        //let mut value = 0;
        shared.register_handler(
            "print",
            Handler::new(|message: &Message| {
                println!("update: {}", message.value);
            }),
            "tag1",
        );
        shared.register_handler(
            "update",
            Handler::new(|message: &Message| {
                //value = message.value.clone();
            }),
            "tag1",
        );
        let mut receiver = shared.receiver();
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
