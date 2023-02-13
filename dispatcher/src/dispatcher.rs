use super::Observer;
use std::collections::HashMap;

pub trait MessageType {
    fn message_type(&self) -> &str;
}

/// The observer reference type
pub type ObserverRef<'a, M> = Box<dyn Observer<'a, M>>;

pub trait Dispatcher<'a, M> {
    /// Register an observer for a message type.
    fn register_handler(&mut self, message_type: &str, observer: &dyn Observer<'a, M>, tag: &str);
    /// Unregister a handler for a message type.
    fn unregister_handler(&mut self, message_type: &str, tag: &str);
    /// Dispatch a message.
    fn dispatch(&mut self, message: &M) -> usize;
}

pub struct LocalDispatcher<'a, M> {
    handlers: HashMap<String, HashMap<String, &'a dyn Observer<'a, M>>>,
}

impl<'a, M> Default for LocalDispatcher<'a, M> {
    fn default() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }
}

impl<'a, M> LocalDispatcher<'a, M> {
    pub fn register_handler(
        &mut self,
        message_type: &str,
        observer: &'a dyn Observer<'a, M>,
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
        match self.handlers.get_mut(message_type) {
            Some(observers) => {
                observers.remove(tag);
            }
            None => {}
        }
    }
}

impl<'a, M: MessageType> LocalDispatcher<'a, M> {
    /// Dispatch method
    ///
    /// Trigger callbacks for registered observers for the given message type
    pub fn dispatch(&mut self, message: &M) -> usize {
        let message_type = message.message_type();
        if let Some(observers) = self.handlers.get_mut(message_type) {
            for observer in observers.values_mut() {
                //observer.handle(message);
            }
            return observers.len();
        }
        0usize
    }
}
