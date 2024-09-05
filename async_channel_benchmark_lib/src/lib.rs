use async_channel::bounded;
use tokio::task;
use tokio::time::Instant;

/// Benchmarks the `async-channel` crate by sending and receiving a specified number of messages asynchronously
///
/// # Arguments
///
/// * `num_messages` - Number of messages to send and receive during the benchmark
pub async fn run_benchmark(num_messages: i32) {
    // Create a bounded async channel with capacity 1024
    let (sender, receiver) = bounded(1024);

    // Start the benchmark timer
    let start_time = Instant::now();

    // Create a producer task to send messages asynchronously
    let producer = task::spawn(async move {
        for i in 0..num_messages {
            sender.send(i).await.unwrap(); // Send data into the async channel
        }
    });

    // Create a consumer task to receive messages asynchronously
    let consumer = task::spawn(async move {
        loop {
            let val = receiver.recv().await.unwrap(); // Receive data from the async channel
            if val == num_messages - 1 {
                println!("Received last message: {}", val);
                break;
            }
        }
    });

    // Wait for both tasks to finish
    producer.await.unwrap();
    consumer.await.unwrap();

    // Calculate elapsed time
    let elapsed_time = start_time.elapsed();
    println!("Processed {} messages in {:?}", num_messages, elapsed_time);
}
