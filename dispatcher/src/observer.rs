use std::cell::{RefCell, RefMut};

/// An observer is a type that can handle a message.
///
/// It mus implement the handle method.
pub trait Observer<'a, M> {
    fn handle(&mut self, message: &M);
}

//pub struct RefMutObserver<'a, M> {
//    ref_mut: RefMut<'a, Box<dyn Observer<M>>>,
//}
/// An observer which embed a mutable reference to an observer.
pub struct RefMutObserver<'a, O> {
    ref_mut: RefMut<'a, Box<O>>,
}

pub struct MutableObserver<O> {
    observer: RefCell<Box<O>>,
}

impl<O> MutableObserver<O> {
    pub fn new(observer: O) -> Self {
        Self {
            observer: RefCell::new(Box::new(observer)),
        }
    }

    /// Borrow the observer as a mutable reference observer with given lifetime.
    pub fn borrow<'a>(&'a mut self) -> RefMutObserver<'a, O> {
        RefMutObserver {
            ref_mut: self.observer.borrow_mut()
        }
    }
}

//impl<'a, M, O> Observer<'a, M> for RefMutObserver<'a, O>{
//    fn handle(&mut self, message: &M) {
//    }
//}
