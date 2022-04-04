use tokio::sync::broadcast;

async fn process_1(msg: String) {
    println!("Process 1 --> msg: {}", msg);
}

async fn process_2(msg: String) {
    println!("Process 2 --> msg: {}", msg);
}

#[tokio::main]
async fn main() {
    let (tx1, mut rx1) = broadcast::channel(16);

    // Create a new sender
    let tx2 = tx1.clone();

    // Creates a new receiver
    let mut rx2 = tx1.subscribe();

    let handle_1 = tokio::spawn(async move {
        while let Ok(msg) = rx1.recv().await {
            process_1(msg).await;
        }
    });

    let handle_2 = tokio::spawn(async move {
        while let Ok(msg) = rx2.recv().await {
            process_2(msg).await;
        }
    });

    tx1.send("data set 1".to_string()).unwrap();
    tx2.send("data set 2".to_string()).unwrap();

    // Dropping tx so that the recievers will close.
    drop(tx1);
    drop(tx2);

    // Making sure the handlers have finished
    handle_1.await.unwrap();
    handle_2.await.unwrap();
}
