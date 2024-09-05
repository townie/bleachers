use tokio::sync::mpsc;
use tokio::task;
use tokio::time::Instant;

pub async fn run_benchmark(num_messages: i32) {
    let (tx, mut rx) = mpsc::channel(100);
    // let num_messages = 1_000_000;

    let start = Instant::now();

    let producer = task::spawn(async move {
        for _ in 0..num_messages {
            tx.send(1).await.unwrap();
        }
    });

    let consumer = task::spawn(async move {
        for _ in 0..num_messages {
            let _ = rx.recv().await.unwrap();
        }
    });

    producer.await.unwrap();
    consumer.await.unwrap();

    let duration = start.elapsed();
    println!("tokio::mpsc  {} messages in {:?}", num_messages, duration);
}
