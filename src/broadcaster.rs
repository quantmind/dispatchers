use super::{Dispatcher, LocalDispatcher, MessageType, ObserverRef};
use std::fmt::Debug;
use tokio::sync::broadcast::{channel, Receiver, Sender};

pub struct Broadcaster<'a, M> {
    local: LocalDispatcher<'a, M>,
    sender: Sender<M>,
    /// This receiver is never used, it is just to keep the sender alive
    _receiver: Option<Receiver<M>>,
}

pub struct SyncBroadcaster<M> {
    sender: Sender<M>,
}

impl<'a, M> Default for Broadcaster<'a, M>
where
    M: Clone + MessageType + std::default::Default,
{
    fn default() -> Self {
        Self::new(100)
    }
}

impl<'a, M> Clone for Broadcaster<'a, M>
where
    M: Clone + MessageType + std::default::Default,
{
    /// Clone the broadcaster so it can be moved to another thread
    fn clone(&self) -> Self {
        Self::from_sender(self.sender.clone())
    }
}

impl<'a, M> Dispatcher<'a, M> for Broadcaster<'a, M>
where
    M: Clone + MessageType + Debug + std::default::Default,
{
    fn register_handler(&mut self, message_type: &str, observer: ObserverRef<'a, M>, tag: &str) {
        self.local.register_handler(message_type, observer, tag);
    }

    fn unregister_handler(&mut self, message_type: &str, tag: &str) {
        self.local.unregister_handler(message_type, tag);
    }

    fn dispatch(&self, message: &M) -> usize {
        // dispatch to local observers
        self.local.dispatch(message);
        // dispatch to remote observers
        self.sender.send(message.clone()).unwrap()
    }
}

impl<'a, M> Broadcaster<'a, M>
where
    M: Clone + MessageType + std::default::Default,
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

    pub fn sync_clone(&self) -> SyncBroadcaster<M> {
        SyncBroadcaster {
            sender: self.sender.clone(),
        }
    }
}

impl<M> SyncBroadcaster<M>
where
    M: Clone + MessageType + std::default::Default,
{
    /// Get a receiver to receive messages from remote observers
    pub fn receiver(&self) -> Receiver<M> {
        self.sender.subscribe()
    }

    pub fn broadcaster(&self) -> Broadcaster<M> {
        Broadcaster::from_sender(self.sender.clone())
    }
}
