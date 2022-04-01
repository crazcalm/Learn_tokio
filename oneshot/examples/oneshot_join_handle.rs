/// From docs
///
/// If the task produces a computation result as its final action before terminating, the
/// JoinHandle can be used to receive that value instead of allocating resources for the oneshot
/// channel. Awaiting on JoinHandle returns Result. If the task panics, the Joinhandle yeilds Err
/// with the panic cause.
use tokio::sync::oneshot;

async fn some_computation() -> String {
    "the result of the computation".to_string()
}

#[tokio::main]
async fn main() {
    let join_handle = tokio::spawn(async move { some_computation().await });

    // Do other work while the computation is happening in the background

    // Wait for the computation result
    let res = join_handle.await.unwrap();
    println!("{}", res);
}
