use super::{MessageType, Observer};
use std::collections::HashMap;
use std::fmt::Debug;
use tokio::sync::broadcast::{channel, error::RecvError, Receiver, Sender};


pub type ObserverRef<M> = Box<dyn Observer<M>>;


pub struct Broadcaster<M> {
    handlers: HashMap<String, HashMap<String, ObserverRef<M>>>,
    pub sender: Sender<M>,
    /// This receiver is never used, it is just to keep the sender alive
    _receiver: Option<Receiver<M>>,
}


impl<M> Default for Broadcaster<M>
where
    M: Clone + MessageType + Send + std::default::Default + std::fmt::Debug,
{
    fn default() -> Self {
        Self::new(100)
    }
}

impl<M> Broadcaster<M>
where
    M: Clone + MessageType + Send + std::default::Default + std::fmt::Debug,
{
    /// Create a new broadcaster
    ///
    /// This method should be called by the main application therad only,
    /// the broadcaster should be passed to other threads by cloning it.
    pub fn new(capacity: usize) -> Self {
        let (sender, receiver) = channel(capacity);
        Self {
            handlers: HashMap::default(),
            sender,
            _receiver: Some(receiver),
        }
    }

    pub fn from_sender(sender: Sender<M>) -> Self {
        Self {
            handlers: HashMap::default(),
            sender,
            _receiver: None,
        }
    }

    pub fn register_handler(&mut self, message_type: &str, observer: ObserverRef<M>, tag: &str) {}

    pub fn unregister_handler(&mut self, message_type: &str, tag: &str) {}

    pub fn dispatch(&self, message: &M) -> usize {
        // dispatch to local observers
        //let mut observers = self.local.dispatch(message);
        // dispatch to remote observers
        let mut observers = 0;
        observers += self.sender.send(message.clone()).unwrap();
        observers
    }

    /// Create a clone which can be sent across thread/coroutines
    pub fn sync_clone(&self) -> Broadcaster<M> {
        Broadcaster::from_sender(self.sender.clone())
    }

    pub async fn listen(&self) -> Result<(), RecvError> {
        let mut receiver = self.sender.subscribe();
        loop {
            match receiver.recv().await {
                Ok(message) => {
                    self.dispatch(&message);
                }
                Err(err) => {
                    return Err(err);
                }
            };
        }
    }
}
