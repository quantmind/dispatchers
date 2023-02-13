use std::cell::RefCell;


struct Data {
    value: i32,
}

struct Container {
    value: RefCell<Data>,
}


impl Container {
    pub fn register(&mut self, dispatcher: &Dispatcher) {
        let mut value = self.value.borrow_mut();
        value.value = 42;
    }
}
