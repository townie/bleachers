use kanal::bounded;
use std::thread;
use std::time::Instant;

/// Benchmarks the `kanal` crate by sending and receiving a specified number of messages
///
/// # Arguments
///
/// * `num_messages` - Number of messages to send and receive during the benchmark
pub fn run_benchmark(num_messages: i32) {
    // Create a bounded channel with a capacity of 1024 messages
    let (sender, receiver) = bounded(1024);

    // Start the benchmark timer
    let start_time = Instant::now();

    // Create a producer thread to send messages
    let producer_thread = thread::spawn(move || {
        for i in 0..num_messages {
            // Send each message into the kanal channel
            sender.send(i).unwrap();
        }
    });

    // Create a consumer thread to receive messages
    let consumer_thread = thread::spawn(move || {
        for _ in 0..num_messages {
            // Receive each message from the kanal channel
            let _value = receiver.recv().unwrap();
        }
    });

    // Wait for both threads to finish
    producer_thread.join().unwrap();
    consumer_thread.join().unwrap();

    // Calculate elapsed time
    let elapsed_time = start_time.elapsed();
    println!(
        "kanal:: Processed {} messages in {:?}",
        num_messages, elapsed_time
    );
}
