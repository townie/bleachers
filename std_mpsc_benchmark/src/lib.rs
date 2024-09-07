use std::sync::mpsc;
use std::thread;
use std::time::Instant;

pub fn run_benchmark(num_messages: i32) {
    let (tx, rx) = mpsc::channel();
    // let num_messages = 1_000_000;

    let start = Instant::now();

    let producer = thread::spawn(move || {
        for _ in 0..num_messages {
            tx.send(1).unwrap();
        }
    });

    let consumer = thread::spawn(move || {
        for _ in 0..num_messages {
            let _ = rx.recv().unwrap();
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();

    let duration = start.elapsed();

    println!("std::mpsc time: {:?}", duration);
}
