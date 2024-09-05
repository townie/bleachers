use concurrent_queue::ConcurrentQueue;
use std::fmt;
use std::sync::Arc;
use std::thread;
use std::time::Instant;
/// Benchmarks the `concurrent-queue` crate by sending and receiving a specified number of messages
///
/// # Arguments
///
/// * `num_messages` - Number of messages to send and receive during the benchmark
pub fn run_benchmark(num_messages: i32) {
    // Create a concurrent queue
    let queue = Arc::new(ConcurrentQueue::unbounded());

    // Clone the queue for the producer and consumer
    let producer_queue = Arc::clone(&queue);
    let consumer_queue = Arc::clone(&queue);

    // Start the benchmark timer
    let start_time = Instant::now();

    // Create a producer thread
    let producer_thread = thread::spawn(move || {
        for i in 0..num_messages {
            producer_queue.push(i).unwrap(); // Push data into the queue
        }
    });

    // Create a consumer thread
    let consumer_thread = thread::spawn(move || {
        loop {
            let val = consumer_queue.pop().unwrap(); // Pop data from the queue
            if val == num_messages - 1 {
                println!("Received last message: {}", val);
                break;
            }
        }
    });

    // Wait for both threads to finish
    producer_thread.join().unwrap();
    consumer_thread.join().unwrap();

    // Calculate elapsed time
    let elapsed_time = start_time.elapsed();
    println!("Processed {} messages in {:?}", num_messages, elapsed_time);
}
