use anyhow;
use eventador::Eventador;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Instant;

/// Benchmarks the `eventador` crate by sending and receiving a specified number of messages
///
/// # Arguments
///
/// * `num_messages` - Number of messages to send and receive during the benchmark
pub fn run_benchmark(num_messages: i32) -> anyhow::Result<()> {
    // Create an Eventador instance
    let eventbus = Eventador::new(2)?;
    let start_time = Instant::now();

    let subscriber = eventbus.subscribe::<i32>();
    let subscriber_thread = std::thread::spawn(move || {
        loop {
            // std::thread::sleep(std::time::Duration::from_micros(10));

            let event = subscriber.recv();
            // println!("Received event: {}", *event);
            if *event == 999_999 {
                println!("Received last message: {}", *event);
                break;
            }
        }
    });

    let mut publisher = eventbus.publisher();
    let _publisher_thread = std::thread::spawn(move || {
        let mut i: usize = 1;
        while i <= num_messages as usize {
            // std::thread::sleep(std::time::Duration::from_micros(10));
            publisher.send(i as i32);
            i += 1;
        }
    });

    subscriber_thread
        .join()
        .expect("Join of subscriber thread was unsuccessful");

    // Calculate elapsed time
    let elapsed_time = start_time.elapsed();
    println!("Processed {} messages in {:?}", num_messages, elapsed_time);
    Ok(())
}
