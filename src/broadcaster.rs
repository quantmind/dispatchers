use super::{Dispatcher, DispatcherError, MessageType, ObserverRef};
use tokio::sync::broadcast::{channel, error, Receiver, Sender};

/// A dispatcher which broadcast messages to other threads/coroutines
pub struct Broadcaster<M> {
    /// to dispatch messages to local observers
    local: Dispatcher<M>,
    /// to send messages to other threads/coroutines
    broadcast_sender: Sender<M>,
    /// This receiver is never used, it is just to keep the sender alive
    _broadcast_receiver: Option<Receiver<M>>,
}

impl<M> Default for Broadcaster<M>
where
    M: Clone + MessageType + std::default::Default,
{
    fn default() -> Self {
        Self::new(100)
    }
}

impl<M> Clone for Broadcaster<M>
where
    M: Clone + MessageType + std::default::Default,
{
    fn clone(&self) -> Self {
        Self {
            local: Dispatcher::default(),
            broadcast_sender: self.broadcast_sender.clone(),
            _broadcast_receiver: None,
        }
    }
}

impl<M> Broadcaster<M>
where
    M: Clone + MessageType + std::default::Default,
{
    /// Create a new broadcaster
    ///
    /// This method should be called by the main application thread only,
    /// the broadcaster should be passed to other threads by cloning it.
    pub fn new(capacity: usize) -> Self {
        let (broadcast_sender, broadcast_receiver) = channel(capacity);
        Self {
            local: Dispatcher::default(),
            broadcast_sender,
            _broadcast_receiver: Some(broadcast_receiver),
        }
    }

    /// Register a handler for a message type
    pub fn register_handler(&mut self, message_type: &str, observer: ObserverRef<M>, tag: &str) {
        self.local.register_handler(message_type, observer, tag);
    }

    /// Unregister a handler for a message type
    pub fn unregister_handler(&mut self, message_type: &str, tag: &str) {
        self.local.unregister_handler(message_type, tag);
    }

    /// Dispatch a message to all observers
    pub fn dispatch(&self, message: &M) -> Result<usize, DispatcherError> {
        // dispatch to local observers
        let n1 = self.local.dispatch(message)?;
        // dispatch to remote observers
        self.broadcast_sender
            .send(message.clone())
            .map(|n| n1 + n)
            .map_err(|err| DispatcherError::SendError(err.to_string()))
    }

    /// Dispatch a message to local observers only
    pub fn dispatch_local(&self, message: &M) -> Result<usize, DispatcherError> {
        self.local.dispatch(message)
    }

    /// Dispatch a message to remote observers only
    pub fn send(&self, message: M) -> Result<usize, error::SendError<M>> {
        self.broadcast_sender.send(message)
    }

    /// Get a receiver to receive messages from other threads/coroutines
    pub fn receiver(&self) -> Receiver<M> {
        self.broadcast_sender.subscribe()
    }

    /// Get a sender to send messages to other threads/coroutines
    pub fn sender(&self) -> Sender<M> {
        self.broadcast_sender.clone()
    }
}
