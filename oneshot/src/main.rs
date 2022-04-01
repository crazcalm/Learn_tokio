/// From the docs:
///
/// The oneshot channel supports sending a single value from a single producer to a single
/// consumer. This channel is usually used to send the result of a computation to a waiter.
use tokio::sync::oneshot;

async fn some_computation() -> String {
    "represents the result of the computation".to_string()
}

#[tokio::main]
async fn main() {
    let (tx, rx) = oneshot::channel();

    tokio::spawn(async move {
        let res = some_computation().await;
        tx.send(res).unwrap();
    });

    // Do other work while the computation is happening in the background

    // Wait for the computation result
    let res = rx.await.unwrap();
    println!("{}", res);
}
