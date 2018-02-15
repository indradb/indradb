use std::sync::{Arc, Mutex};

// We used a mutex instead of an atomic because in benchmarks it seems to be
// faster, and we can explicitly enforce u64 even on 32-bit systems (as of the
// time of this writing, `AtomicUsize` is stable but `AtomicU64` is not)
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