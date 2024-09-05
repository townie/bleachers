// use disruptor::{Consumer, Disruptor, Producer};
use core::num;
use disruptor::*;
use std::thread;
use std::time::Instant;

// The event on the ring buffer.
struct Event {
    price: i32,
}

/// * `num_messages` - Number of messages to send and receive during the benchmark
pub fn run_benchmark(num_messages: i32) {
    const BUFFER_SIZE: usize = 1024; // Size of the disruptor ring buffer
    let factory = || Event { price: 0 };

    // Closure for processing events.
    let processor = |e: &Event, sequence: Sequence, end_of_batch: bool| {
        // Your processing logic here.
        // let inbound = e.price;
        if e.price == 999999 {
            println!("Processing event: {} ", e.price,);
        }
        // println!("Processing event: {} ", inbound,);
    };

    let size = 64;
    let mut producer = disruptor::build_single_producer(size, factory, BusySpin)
        .handle_events_with(processor)
        .build();
    let start_time = Instant::now();

    // Publish single events into the Disruptor via the `Producer` handle.
    for i in 0..num_messages {
        producer.publish(|e| {
            e.price = i;
        });
    }

    // Publish a batch of events into the Disruptor.
    // producer.batch_publish(5, |iter| {
    //     for e in iter {
    //         // `iter` is guaranteed to yield 5 events.
    //         e.price = 42.0;
    //     }
    // });

    // // Calculate elapsed time
    let elapsed_time = start_time.elapsed();
    println!("disruptor {} messages in {:?}", num_messages, elapsed_time);
}
