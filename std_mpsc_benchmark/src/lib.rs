use std::sync::mpsc;
use std::thread;
use std::time::Instant;

pub fn run_benchmark(num_messages: i32) {
    let (tx, rx) = mpsc::channel();

    let start = Instant::now();

    let producer = thread::spawn(move || {
        for _ in 0..num_messages {
            tx.send(1).unwrap();
        }
    });

    let consumer = thread::spawn(move || loop {
        match rx.recv() {
            Ok(val) => {
                if val == 999_999 {
                    println!("Received last message: {}", val);
                    break;
                }
            }
            Err(e) => {
                break;
            }
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();

    let duration = start.elapsed();
    println!("std::mpsc  {} messages in {:?}", num_messages, duration);
}
