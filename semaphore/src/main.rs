use std::sync::Arc;
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

async fn example(permit: OwnedSemaphorePermit, limiting_semaphore: Arc<Semaphore>) {
    println!("start of example function");

    let limit_sem = limiting_semaphore.acquire().await.unwrap();
    println!("--> start of critical section");
    println!("--> end of critical section");
    drop(limit_sem);

    println!("End of example function");
    drop(permit);
}

#[tokio::main]
async fn main() {
    let semaphore = Arc::new(Semaphore::new(10));
    let limiting_semaphore = Arc::new(Semaphore::new(2));
    let mut join_handles = Vec::new();

    // Bounding can be demonstrated by having a small number of semaphore permits
    // and a high number of tasks you would like to complete.
    //
    // Limiting can be demonstrated the limiting_semaphore which is used to
    // dictate how many threads are allowed to run that critical section of code
    // at once.
    for _ in 0..10 {
        let permit = semaphore.clone().acquire_owned().await.unwrap();
        join_handles.push(tokio::spawn(example(permit, limiting_semaphore.clone())));
    }

    for handle in join_handles {
        handle.await.unwrap();
    }
}
