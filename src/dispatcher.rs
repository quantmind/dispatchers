use std::collections::HashMap;
use thiserror::Error;

/// Error type for dispatchers
///
/// An error can be raised when sending or receiving a message across thread/coroutines.
#[derive(Error, Debug)]
pub enum DispatcherError {
    /// Error while sending a message
    #[error("error while sending message: {0}")]
    SendError(String),
    /// Error while receiving a message
    #[error("error while receinging message: {0}")]
    RecvError(String),
}

/// A trait for messages that can be dispatched
pub trait MessageType {
    /// Return the message type
    ///
    /// This is used to identify the type of a message and the callbacks to trigger
    fn message_type(&self) -> &str;
}

/// An observer is a type that can handle a message.
///
/// It is a callback that is called when a message is dispatched.
pub trait Observer<M> {
    fn call(&self, message: &M);
}

/// A reference to an observer for the same thread/coroutine
pub type LocalObserverRef<'a, M> = Box<dyn Observer<M> + 'a>;

/// A reference to an observer
pub type ObserverRef<M> = Box<dyn Observer<M> + Send>;

/// A local dispatcher
///
/// This dispatcher only works on the same thread/coroutine.
/// A dispatcher is a type that can dispatch messages to observers.
/// Observers are registered by message type and a tag.
/// A message can be dispatched to all observers that are registered for the message type.
#[derive(Default)]
pub struct LocalDispatcher<'a, M> {
    handlers: HashMap<String, HashMap<String, LocalObserverRef<'a, M>>>,
}

/// A dispatcher
///
/// A dispatcher is a type that can dispatch messages to observers.
/// Observers are registered by message type and a tag.
/// A message can be dispatched to all observers that are registered for the message type.
#[derive(Default)]
pub struct Dispatcher<M> {
    handlers: HashMap<String, HashMap<String, ObserverRef<M>>>,
}

impl<'a, M> LocalDispatcher<'a, M>
where
    M: MessageType,
{
    pub fn register_handler(
        &mut self,
        message_type: &str,
        observer: LocalObserverRef<'a, M>,
        tag: &str,
    ) {
        match self.handlers.get_mut(message_type) {
            Some(observers) => {
                observers.insert(tag.to_owned(), observer);
            }
            None => {
                let mut observers = HashMap::new();
                observers.insert(tag.to_owned(), observer);
                self.handlers.insert(message_type.to_owned(), observers);
            }
        }
    }

    pub fn unregister_handler(&mut self, message_type: &str, tag: &str) {
        if let Some(observers) = self.handlers.get_mut(message_type) {
            observers.remove(tag);
        }
    }

    /// Dispatch method
    ///
    /// Trigger callbacks for registered observers for the given message type
    pub fn dispatch(&self, message: &M) -> Result<usize, DispatcherError> {
        let message_type = message.message_type();
        if let Some(observers) = self.handlers.get(message_type) {
            for observer in observers.values() {
                observer.call(message);
            }
            return Ok(observers.len());
        }
        Ok(0usize)
    }
}

impl<M> Dispatcher<M>
where
    M: MessageType,
{
    pub fn register_handler(&mut self, message_type: &str, observer: ObserverRef<M>, tag: &str) {
        match self.handlers.get_mut(message_type) {
            Some(observers) => {
                observers.insert(tag.to_owned(), observer);
            }
            None => {
                let mut observers = HashMap::new();
                observers.insert(tag.to_owned(), observer);
                self.handlers.insert(message_type.to_owned(), observers);
            }
        }
    }

    pub fn unregister_handler(&mut self, message_type: &str, tag: &str) {
        if let Some(observers) = self.handlers.get_mut(message_type) {
            observers.remove(tag);
        }
    }

    /// Dispatch method
    ///
    /// Trigger callbacks for registered observers for the given message type
    pub fn dispatch(&self, message: &M) -> Result<usize, DispatcherError> {
        let message_type = message.message_type();
        if let Some(observers) = self.handlers.get(message_type) {
            for observer in observers.values() {
                observer.call(message);
            }
            return Ok(observers.len());
        }
        Ok(0usize)
    }
}
