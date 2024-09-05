use crossbeam::channel;
use std::thread;
use std::time::Instant;

pub fn run_benchmark(num_messages: i32) {
    let (tx, rx) = channel::bounded(100);

    let start = Instant::now();

    let producer = thread::spawn(move || {
        for i in 0..num_messages {
            tx.send(i).unwrap();
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
            Err(e) => {}
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();

    let duration = start.elapsed();

    println!(
        "crossbeam::channel  {} messages in {:?}",
        num_messages, duration
    );
}
