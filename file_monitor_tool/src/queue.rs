pub struct EventQueue<T> {
    events: Vec<T>,
    capacity: usize,
}

impl<T> EventQueue<T> {
    pub fn new(capacity: usize) -> Self {
        EventQueue {
            events: Vec::with_capacity(capacity),
            capacity,
        }
    }

    pub fn push(&mut self, event: T) {
        if self.events.len() >= self.capacity {
            self.events.remove(0);
        }
        self.events.push(event);
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.events.is_empty() {
            None
        } else {
            Some(self.events.remove(0))
        }
    }
}
