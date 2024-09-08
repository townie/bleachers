use async_channel_benchmark_lib::run_benchmark as async_channel_benchmark;
use concurrent_queue_benchmark::run_benchmark as concurrent_queue_benchmark;
use crossbeam_mpsc_benchmark::run_benchmark as crossbeam_benchmark;
use disruptor_benchmark::run_benchmark as disruptor_benchmark;
use eventador_benchmark_lib::run_benchmark as eventador_benchmark;
use flume_benchmark_lib::run_benchmark as flume_benchmark;
use kanal_benchmark_lib::run_benchmark as kanal_benchmark;
use loole_benchmark_lib::run_benchmark as loole_benchmark;
use mantra_lib::run_benchmark as mantra_benchmark;
use std_mpsc_benchmark::run_benchmark as std_benchmark;
use tokio_mpsc_benchmark::run_benchmark as tokio_benchmark;

#[tokio::main]
async fn main() {
    let num_messages = 1_000;

    disruptor_benchmark(num_messages.clone());

    mantra_benchmark(num_messages.clone());

    std_benchmark(num_messages.clone());

    concurrent_queue_benchmark(num_messages.clone());

    tokio_benchmark(num_messages.clone()).await;

    crossbeam_benchmark(num_messages.clone());

    kanal_benchmark(num_messages.clone());

    flume_benchmark(num_messages.clone());

    async_channel_benchmark(num_messages.clone()).await;

    loole_benchmark(num_messages.clone());

    if false {
        // Disabled because so slow
        println!("\nRunning eventador benchmark...");
        let _ = eventador_benchmark(num_messages.clone());
    }
}
