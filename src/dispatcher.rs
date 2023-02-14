use std::collections::HashMap;

pub trait MessageType {
    fn message_type(&self) -> &str;
}

/// An observer is a type that can handle a message.
pub trait Observer<M> {
    fn call(&self, message: &M);
}

pub type ObserverRef<'a, M> = Box<dyn Observer<M> + 'a>;

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

/// A local dispatcher
///
/// This dispatcher only works on the same thread/coroutine.
#[derive(Default)]
pub struct LocalDispatcher<'a, M> {
    handlers: HashMap<String, HashMap<String, ObserverRef<'a, M>>>,
}

impl<'a, M> Dispatcher<'a, M> for LocalDispatcher<'a, M>
where
    M: MessageType,
{
    fn register_handler(
        &mut self,
        message_type: &str,
        observer: ObserverRef<'a, M>,
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

    fn unregister_handler(&mut self, message_type: &str, tag: &str) {
        if let Some(observers) = self.handlers.get_mut(message_type) {
            observers.remove(tag);
        }
    }

    /// Dispatch method
    ///
    /// Trigger callbacks for registered observers for the given message type
    fn dispatch(&self, message: &M) -> usize {
        let message_type = message.message_type();
        if let Some(observers) = self.handlers.get(message_type) {
            for observer in observers.values() {
                observer.call(message);
            }
            return observers.len();
        }
        0usize
    }
}
