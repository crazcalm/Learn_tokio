use tokio::sync::watch;

async fn example(mut receiver: watch::Receiver<&str>, num: i32) {
    while receiver.changed().await.is_ok() {
        println!("example {} received = {:?}", num, *receiver.borrow());
    }
}

#[tokio::main]
async fn main() {
    let (tx, rx) = watch::channel("hello");

    let handle_1 = tokio::spawn(example(rx.clone(), 1));
    let handle_2 = tokio::spawn(example(rx.clone(), 2));
    let handle_3 = tokio::spawn(example(rx.clone(), 3));
    let handle_4 = tokio::spawn(example(rx, 4));

    // Will only store/share the latest value (and drop previous changes), so, if you value changes quickly and often,
    // then you should use a Broadcast channel.
    tx.send("world").unwrap();
    //tx.send("watch").unwrap();
    //tx.send("me").unwrap();
    //tx.send("change").unwrap();

    drop(tx);

    handle_1.await.unwrap();
    handle_2.await.unwrap();
    handle_3.await.unwrap();
    handle_4.await.unwrap();
}
