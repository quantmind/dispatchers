use dispatchers::{Broadcaster, Dispatcher, MessageType, Observer};
use tokio;

#[derive(Default, Debug, Clone)]
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
    pub fn exit() -> Self {
        Self {
            value: 0,
            message_type: "exit".to_owned(),
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

impl<F> Observer<Message> for Handler<F>
where
    F: Fn(&Message) + Send,
{
    fn call(&self, message: &Message) {
        (self.fun)(message)
    }
}

#[tokio::main]
async fn main() {
    let dispatcher = Broadcaster::<Message>::default();
    let d = dispatcher.sync_clone();
    tokio::spawn(async move {
        // the sender is used to send messages to the main thread
        let sender = d.sender();
        let mut shared = d.sync_clone();

        // registers local handlers
        shared.register_handler(
            "update",
            Handler::new(|message: &Message| {
                println!("update: {}", message.value);
            }),
            "tag1",
        );
        shared.register_handler(
            "exit",
            Handler::new(|_message: &Message| {
                sender.send(Message::exit()).unwrap();
            }),
            "tag1",
        );

        // start loop for receiving messages from other threads
        let mut receiver = shared.receiver();
        loop {
            match receiver.recv().await {
                Ok(message) => {
                    shared.local.dispatch(&message);
                    if message.message_type == "exit" {
                        break;
                    }
                }
                Err(_err) => {
                    break;
                }
            };
        }
    });

    let mut receiver = dispatcher.receiver();
    let mut counter = 0;
    loop {
        tokio::select! {
            _ = tokio::time::sleep(tokio::time::Duration::from_millis(500)) => {
                counter += 1;
                if counter < 6 {
                    dispatcher.dispatch(&Message::update(counter));
                } else if counter == 6 {
                    dispatcher.dispatch(&Message::exit());
                }
            }

            Ok(message) = receiver.recv() => {
                if message.message_type == "exit" {
                    println!("exit");
                    break;
                }
            }
        }
    }
}
