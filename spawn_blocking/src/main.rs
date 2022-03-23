use std::{thread, time};

use tokio::task;

fn sync_code(input: i32) -> i32 {
    thread::sleep(time::Duration::new(5, 0));

    return 42 + input;
}

#[tokio::main]
async fn main() {
    let input = 0;

    let blocked_code = task::spawn_blocking(move || {
        let result = sync_code(input);

        result
    })
    .await
    .unwrap();

    print!("result: {:?}", blocked_code);
}
