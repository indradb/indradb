use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct Counter(Arc<Mutex<u64>>);

impl Counter {
    pub fn new() -> Self {
        Counter { 0: Arc::new(Mutex::new(0)) }
    }

    pub fn increment(&self) {
        *self.0.lock().unwrap() += 1;
    }

    pub fn decrement(&self) {
        *self.0.lock().unwrap() -= 1;
    }

    pub fn get(&self) -> u64 {
        *self.0.lock().unwrap()
    }
}