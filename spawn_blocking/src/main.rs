use std::{thread, time};

use tokio::task;

fn sync_code(input: i32) -> i32 {
    thread::sleep(time::Duration::new(5, 0));

    return 42 + input;
}

#[tokio::main]
async fn main() {
    let input = 0;

    // example of sync code being called directly
    let _ = sync_code(0);

    // Example of sync code being called with spawn blocking.
    let blocked_code = task::spawn_blocking(move || {
        let result = sync_code(input);

        result
    })
    .await
    .unwrap();

    let blocked_code2 = task::spawn_blocking(move || {
        let result = sync_code(input);

        result
    })
    .await
    .unwrap();

    println!("result: {:?}", blocked_code);
    println!("result: {:?}", blocked_code2);
}
