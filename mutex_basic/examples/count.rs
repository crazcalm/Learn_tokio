use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() {
    let count = Arc::new(Mutex::new(0));

    for i in 0..5 {
        let my_count = Arc::clone(&count);
        tokio::spawn(async move {
            for j in 0..10 {
                let mut lock = my_count.lock().await;
                *lock += 1;
                println!(
                    "spawed tasker: {}, current_iteration: {}, count_value: {}",
                    i, j, lock
                );
            }
        });
    }

    loop {
        if *count.lock().await >= 50 {
            break;
        }
    }

    println!("Count hit 50");
}
