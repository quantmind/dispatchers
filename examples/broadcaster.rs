use dispatchers::{Broadcaster, MessageType, Observer};
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
    pub fn new(fun: F) -> Box<Self> {
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
    let mut input_dispatcher = Broadcaster::<Message>::default();
    let output_dispatcher = Broadcaster::<Message>::default();
    input_dispatcher.register_handler(
        "update",
        Handler::new(|message| {
            println!("input update: {}", message.value);
        }),
        "tag1",
    );
    let input_shared = input_dispatcher.clone();
    let output_shared = output_dispatcher.clone();
    tokio::spawn(async move {
        // start loop for receiving messages from other threads
        let mut receiver = output_shared.receiver();
        loop {
            match receiver.recv().await {
                Ok(message) => {
                    if message.message_type == "exit" {
                        input_shared.send(Message::exit()).unwrap();
                        break;
                    } else {
                        println!("output update: {}", message.value);
                        input_shared.send(message).unwrap();
                    }
                }
                Err(_err) => {
                    break;
                }
            };
        }
    });

    let mut input_receiver = input_dispatcher.receiver();
    let mut counter = 0;
    loop {
        tokio::select! {
            _ = tokio::time::sleep(tokio::time::Duration::from_millis(100)) => {
                counter += 1;
                if counter < 6 {
                    output_dispatcher.dispatch(Message::update(counter)).unwrap();
                } else if counter == 6 {
                    output_dispatcher.dispatch(Message::exit()).unwrap();
                }
            }

            Ok(message) = input_receiver.recv() => {
                input_dispatcher.dispatch_local(&message).unwrap();
                if message.message_type == "exit" {
                    println!("exit");
                    break;
                }
            }
        }
    }
}
