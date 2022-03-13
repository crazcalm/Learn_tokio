use std::sync::Arc;
use tokio::sync::Barrier;

#[tokio::main]
async fn main() {
    let mut handles = Vec::with_capacity(10);
    let barrier = Arc::new(Barrier::new(10));

    for _ in 0..10 {
        let c = barrier.clone();

        // The same messages will be printed together.
        // You will NOT see any interleaving.
        handles.push(tokio::spawn(async move {
            println!("before wait");
            let wait_result = c.wait().await;
            println!("after wait");
            wait_result
        }));
    }

    // Will not resolve until all "after wait" messages have been printed.
    let mut num_leaders = 0;
    for handle in handles {
        let wait_result = handle.await.unwrap();
        if wait_result.is_leader() {
            num_leaders += 1;
        }
    }

    // Exactly one barrier will resolve as the leader
    assert_eq!(num_leaders, 1);
    println!("num_leaders: {}", num_leaders);
}
