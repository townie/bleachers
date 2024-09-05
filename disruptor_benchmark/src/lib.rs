// use disruptor::{Consumer, Disruptor, Producer};
use disruptor::*;
use std::thread;
use std::time::Instant;

/// Benchmarks the disruptor crate by sending and receiving a specified number of messages
///
/// # Arguments
///
/// * `num_messages` - Number of messages to send and receive during the benchmark
pub fn run_benchmark(num_messages: i32) {
    const BUFFER_SIZE: usize = 1024; // Size of the disruptor ring buffer

    // Create a disruptor with the specified buffer size
    let mut disruptor = Disruptor::new(BUFFER_SIZE, 1).unwrap(); // Using 1 for wait_strategy

    // Split the disruptor into producer and consumer handles
    let (producer, mut consumer) = disruptor.split();

    // Start the benchmark timer
    let start_time = Instant::now();

    // Create a producer thread to send messages
    let producer_thread = thread::spawn(move || {
        for i in 0..num_messages {
            let sequence = producer.claim_sequence(); // Claim a sequence in the ring buffer
            producer.publish(sequence, i as u64); // Publish the message (u64 value)
        }
    });

    // Create a consumer thread to receive messages
    let consumer_thread = thread::spawn(move || {
        for _ in 0..num_messages {
            let sequence = consumer.wait_for_sequence(); // Wait for a message
            let value = consumer.consume(sequence); // Consume the message
            let _ = value; // Use the value (can be used for validation)
        }
    });

    // Wait for both threads to finish
    producer_thread.join().unwrap();
    consumer_thread.join().unwrap();

    // Calculate elapsed time
    let elapsed_time = start_time.elapsed();
    println!("Processed {} messages in {:?}", num_messages, elapsed_time);
}
