use super::{Dispatcher, DispatcherError, LocalDispatcher, MessageType, ObserverRef};
use tokio::sync::broadcast::{channel, error, Receiver, Sender};

/// A dispatcher which broadcast messages to other threads/coroutines
pub struct Broadcaster<'a, M> {
    /// to dispatch messages to local observers
    pub local: LocalDispatcher<'a, M>,
    /// to send messages to other threads/coroutines
    broadcast_sender: Sender<M>,
    /// This receiver is never used, it is just to keep the sender alive
    _broadcast_receiver: Option<Receiver<M>>,
}

/// A dispatcher which which can be shared cross threads/coroutines
pub struct BroadcasterSync<M> {
    broadcast_sender: Sender<M>,
}

impl<'a, M> Default for Broadcaster<'a, M>
where
    M: Clone + MessageType + std::default::Default + std::fmt::Debug,
{
    fn default() -> Self {
        Self::new(100)
    }
}

impl<'a, M> Dispatcher<'a, M> for Broadcaster<'a, M>
where
    M: Clone + MessageType + std::default::Default + std::fmt::Debug,
{
    fn register_handler(&mut self, message_type: &str, observer: ObserverRef<'a, M>, tag: &str) {
        self.local.register_handler(message_type, observer, tag);
    }

    fn unregister_handler(&mut self, message_type: &str, tag: &str) {
        self.local.unregister_handler(message_type, tag);
    }

    fn dispatch(&self, message: &M) -> Result<usize, DispatcherError> {
        // dispatch to local observers
        let n1 = self.local.dispatch(message)?;
        // dispatch to remote observers
        self.broadcast_sender
            .send(message.clone())
            .map(|n| n1 + n)
            .map_err(|err| DispatcherError::SendError(err.to_string()))
    }
}

impl<'a, M> Clone for Broadcaster<'a, M>
where
    M: Clone + MessageType + std::default::Default + std::fmt::Debug,
{
    fn clone(&self) -> Self {
        Self {
            local: LocalDispatcher::default(),
            broadcast_sender: self.broadcast_sender.clone(),
            _broadcast_receiver: None,
        }
    }
}

impl<'a, M> Broadcaster<'a, M>
where
    M: Clone + MessageType + std::default::Default + std::fmt::Debug,
{
    /// Create a new broadcaster
    ///
    /// This method should be called by the main application therad only,
    /// the broadcaster should be passed to other threads by cloning it.
    pub fn new(capacity: usize) -> Self {
        let (broadcast_sender, broadcast_receiver) = channel(capacity);
        Self {
            local: LocalDispatcher::default(),
            broadcast_sender,
            _broadcast_receiver: Some(broadcast_receiver),
        }
    }

    /// Create a clone which can be sent across thread/coroutines
    pub fn sync_clone(&self) -> BroadcasterSync<M> {
        BroadcasterSync {
            broadcast_sender: self.sender(),
        }
    }

    pub fn send(&self, message: M) -> Result<usize, error::SendError<M>> {
        self.broadcast_sender.send(message)
    }

    pub fn receiver(&self) -> Receiver<M> {
        self.broadcast_sender.subscribe()
    }

    pub fn sender(&self) -> Sender<M> {
        self.broadcast_sender.clone()
    }
}

impl<M> BroadcasterSync<M>
where
    M: Clone + MessageType + Send + std::default::Default + std::fmt::Debug,
{
    pub fn receiver(&self) -> Receiver<M> {
        self.broadcast_sender.subscribe()
    }

    pub fn send(&self, message: M) -> Result<usize, error::SendError<M>> {
        self.broadcast_sender.send(message)
    }
}
