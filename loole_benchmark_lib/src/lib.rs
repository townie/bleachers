use loole::*;
use std::thread;
use std::time::Instant;

/// Benchmarks the `loole` crate by sending and receiving a specified number of messages
///
/// # Arguments
///
/// * `num_messages` - Number of messages to send and receive during the benchmark
pub fn run_benchmark(num_messages: i32) {
    // Create a bounded channel with a capacity of 1024 messages
    let (sender, receiver) = loole::unbounded();

    // Start the benchmark timer
    let start_time = Instant::now();

    // Create a producer thread to send messages
    let producer_thread = thread::spawn(move || {
        for i in 0..num_messages {
            sender.send(i).unwrap(); // Send data into the loole channel
        }
    });

    // Create a consumer thread to receive messages
    let consumer_thread = thread::spawn(move || {
        loop {
            let val = receiver.recv().unwrap(); // Receive data from the loole channel
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
