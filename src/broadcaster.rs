use super::{Dispatcher, LocalDispatcher, MessageType, ObserverRef};
use std::fmt::Debug;
use tokio::sync::broadcast::{channel, Receiver, Sender, error::RecvError};

pub trait Dispatcher<'a, M> {
    /// Register an observer for a message type.
    fn register_handler(
        &mut self,
        message_type: &str,
        observer: ObserverRef<'a, M>,
        tag: &str,
    );
    /// Unregister a observers for a message type and a tag
    fn unregister_handler(&mut self, message_type: &str, tag: &str);
    /// Dispatch a message
    fn dispatch(&self, message: &M) -> usize;
}


pub struct Broadcaster<M> {
    sender: Sender<M>,
    /// This receiver is never used, it is just to keep the sender alive
    _receiver: Option<Receiver<M>>,
}

pub struct SyncBroadcaster<M> {
    sender: Sender<M>,
}

impl<M> Default for Broadcaster<M>
where
    M: Clone + MessageType + Send + std::default::Default,
{
    fn default() -> Self {
        Self::new(100)
    }
}

impl<M> Dispatcher<M> for Broadcaster<M>
where
    M: Clone + MessageType + Debug + Send + std::default::Default,
{
    fn register_handler(&mut self, message_type: &str, observer: ObserverRef<'a, M>, tag: &str) {
        self.local.register_handler(message_type, observer, tag);
    }

    fn unregister_handler(&mut self, message_type: &str, tag: &str) {
        self.local.unregister_handler(message_type, tag);
    }

    fn dispatch(&self, message: &M) -> usize {
        // dispatch to local observers
        let mut observers = self.local.dispatch(message);
        // dispatch to remote observers
        observers += self.sender.send(message.clone()).unwrap();
        observers
    }
}

impl<'a, M> Broadcaster<'a, M>
where
    M: Clone + MessageType + Send + std::default::Default,
{
    /// Create a new broadcaster
    ///
    /// This method should be called by the main application therad only,
    /// the broadcaster should be passed to other threads by cloning it.
    pub fn new(capacity: usize) -> Self {
        let (sender, receiver) = channel(capacity);
        Self {
            local: LocalDispatcher::default(),
            sender,
            _receiver: Some(receiver),
        }
    }

    pub fn from_sender(sender: Sender<M>) -> Self {
        Self {
            local: LocalDispatcher::default(),
            sender,
            _receiver: None,
        }
    }

    /// Create a clone which can be sent across thread/coroutines
    pub fn sync_clone(&self) -> SyncBroadcaster<M> {
        SyncBroadcaster {
            sender: self.sender.clone(),
        }
    }

    pub async fn listen(&self) -> Result<(), RecvError> {
        let mut receiver = self.sender.subscribe();
        loop {
            match receiver.recv().await {
                Ok(message) => self.local.dispatch(&message),
                Err(err) => {
                    return Err(err);
                }
            };
        }
    }
}

impl<M> SyncBroadcaster<M>
where
    M: Clone + MessageType + Send + std::default::Default,
{
    /// Get a receiver to receive messages from remote observers
    pub fn receiver(&self) -> Receiver<M> {
        self.sender.subscribe()
    }

    pub fn broadcaster(&self) -> Broadcaster<M> {
        Broadcaster::from_sender(self.sender.clone())
    }
}
