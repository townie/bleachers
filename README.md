# Benchmark mspc channels

 cargo run --release -p benchmark_runner


```
disruptor:: 1000000 messages in 13.079042ms
std::mpsc time: 23.957333ms
concurrent_queue:: Processed 1000000 messages in 16.174375ms
tokio::mpsc  1000000 messages in 46.308333ms
crossbeam::channel  1000000 messages in 44.431542ms
kanal:: Processed 1000000 messages in 33.131625ms
flume:: Processed 1000000 messages in 36.869291ms
async_channel Processed 1000000 messages in 37.270375ms
loole:: Processed 1000000 messages in 34.598042ms
```
