use std::sync::Arc;
use tokio::sync::Notify;

async fn example(notify2: Arc<Notify>) {
    notify2.notified().await;
    println!("received notification");
}

#[tokio::main]
async fn main() {
    // from tokio docs
    let notify = Arc::new(Notify::new());
    let notify2 = notify.clone();

    let handle = tokio::spawn(example(notify2));

    println!("sending notification");
    notify.notify_one();

    handle.await.unwrap();
}
