use crossbeam_mpsc_benchmark::run_benchmark as crossbeam_benchmark;
use disruptor_benchmark::run_benchmark as disruptor_benchmark;
use std_mpsc_benchmark::run_benchmark as std_benchmark;
use tokio_mpsc_benchmark::run_benchmark as tokio_benchmark;

#[tokio::main]
async fn main() {
    let num_messages = 1_000_000;
    println!("Running std::mpsc benchmark...");
    std_benchmark(num_messages);

    println!("\nRunning tokio::mpsc benchmark...");
    tokio_benchmark(num_messages).await;

    println!("\nRunning crossbeam::channel benchmark...");
    crossbeam_benchmark(num_messages);

    println!("\nRunning disruptor benchmark...");
    disruptor_benchmark(num_messages);
}
