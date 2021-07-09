use std::sync::atomic::{AtomicUsize, Ordering};
use std::fmt;

pub static STATS: Stats = Stats{
    alloc_bytes: Counter("allocator.alloc_bytes", AtomicUsize::new(0)),
    alloc_calls: Counter("allocator.alloc_calls", AtomicUsize::new(0)),
    dealloc_bytes: Counter("allocator.dealloc_bytes", AtomicUsize::new(0)),
    dealloc_calls: Counter("allocator.dealloc_calls", AtomicUsize::new(0)),
};

#[derive(Debug)]
pub struct Stats{
    pub alloc_bytes: Counter,
    pub alloc_calls: Counter,
    pub dealloc_calls: Counter,
    pub dealloc_bytes: Counter,
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
            "{}, {}, {}, {}",
            self.alloc_bytes,
            self.alloc_calls,
            self.dealloc_bytes,
            self.dealloc_calls,
        )
    }
}

#[derive(Debug)]
pub struct Counter (&'static str, AtomicUsize);

impl fmt::Display for Counter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.0, self.get())
    }
}

impl Counter {
    pub fn inc(&self) {
        self.1.fetch_add(1, Ordering::SeqCst);
    }

    pub fn inc_by(&self, b: usize) {
        self.1.fetch_add(b, Ordering::SeqCst);
    }

    pub fn get(&self) -> usize {
        self.1.load(Ordering::SeqCst)
    }
}
