use std::sync::Arc;
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};

async fn read_example(data: Arc<RwLock<i32>>) {
    let value = data.read().await;
    println!("read: {}", value);
    sleep(Duration::from_secs(5)).await;
}

async fn write_example(data: Arc<RwLock<i32>>) {
    let mut value = data.write().await;
    *value += 1;
    println!("write: {}", value);
}

#[tokio::main]
async fn main() {
    // Unlimited readers example
    //let lock = Arc::new(RwLock::new(0));

    // Max readers example:
    let lock = Arc::new(RwLock::with_max_readers(0, 10));

    let mut handles = vec![];
    for num in 0..100 {
        let lock_clone = lock.clone();
        let handle = match num {
            x if x % 20 == 0 => tokio::spawn(write_example(lock_clone)),
            _ => tokio::spawn(read_example(lock_clone)),
        };

        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }
}
