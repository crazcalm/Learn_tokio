use tokio::sync::Notify;

use std::collections::VecDeque;
use std::sync::Mutex;

struct Channel<T> {
    values: Mutex<VecDeque<T>>,
    notify: Notify,
}

impl<T> Channel<T> {
    pub fn send(&self, value: T) {
        self.values.lock().unwrap().push_back(value);

        // Notify the consumer a value is available
        self.notify.notify_one();
    }

    pub async fn recv(&self) -> T {
        loop {
            // Drain values
            if let Some(value) = self.values.lock().unwrap().pop_front() {
                return value;
            }
        }

        // Wait for values to be available
        self.notify.notified().await;
    }
}

fn main() {
    println!("Hello, world!");
}
