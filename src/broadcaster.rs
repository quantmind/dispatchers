use super::{Dispatcher, LocalDispatcher, MessageType, ObserverRef};
use tokio::sync::broadcast::{channel, error, Receiver, Sender};

pub struct Broadcaster<'a, M> {
    pub local: LocalDispatcher<'a, M>,
    pub sender: Sender<M>,
    /// This receiver is never used, it is just to keep the sender alive
    _receiver: Option<Receiver<M>>,
}

impl<'a, M> Default for Broadcaster<'a, M>
where
    M: Clone + MessageType + Send + std::default::Default + std::fmt::Debug,
{
    fn default() -> Self {
        Self::new(100)
    }
}

impl<'a, M> Dispatcher<'a, M> for Broadcaster<'a, M>
where
    M: Clone + MessageType + Send + std::default::Default + std::fmt::Debug,
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
    M: Clone + MessageType + Send + std::default::Default + std::fmt::Debug,
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
    pub fn sync_clone(&self) -> Self {
        Broadcaster::from_sender(self.sender.clone())
    }

    pub fn send(&self, message: M) -> Result<usize, error::SendError<M>> {
        self.sender.send(message)
    }

    pub fn receiver(&self) -> Receiver<M> {
        self.sender.subscribe()
    }

    pub fn sender(&self) -> Sender<M> {
        self.sender.clone()
    }

    pub async fn listen(&self) -> Result<(), error::RecvError> {
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
