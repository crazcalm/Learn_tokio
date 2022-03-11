use std::time::Duration;
use tokio::sync::mpsc;

async fn sum(list: &[i32]) -> i32 {
    tokio::time::sleep(Duration::new(2, 0)).await;
    list.iter().fold(0, |acc, x| acc + x)
}

#[tokio::main]
async fn main() {
    let person_1 = vec![1, 2, 3, 4, 5];
    let person_2 = vec![6, 7, 8, 9, 10];
    let person_3 = vec![12, 11, 23, 12, 12];
    let person_4 = vec![123, 123, 123, 123, 123];

    let people = vec![person_1, person_2, person_3];

    let (tx, mut rx) = mpsc::channel(4);

    for person in people {
        let tx_clone = tx.clone();
        tokio::spawn(async move {
            if let Err(_) = tx_clone.send(sum(&person[..]).await).await {
                println!("receiver dropped");
                return;
            }
        });
    }

    // last person get the original tx so that the channel will close
    tokio::spawn(async move {
        if let Err(_) = tx.send(sum(&person_4[..]).await).await {
            println!("receiver dropped");
            return;
        }
    });

    let mut total = 0;
    while let Some(num) = rx.recv().await {
        total += num;
        println!("total: {:?}", total);
    }
}
