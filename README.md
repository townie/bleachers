# Benchmark mspc channels

 cargo run --release -p benchmark_runner

# 1,000,000 messages

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


# 1,000 messages
```
disruptor:: 1000 messages in 46.917µs
mantra::  1000 messages in 97.792µs
std::mpsc time: 114.667µs
concurrent_queue:: Processed 1000 messages in 69.208µs
tokio::mpsc  1000 messages in 146µs
crossbeam::channel  1000 messages in 101.291µs
kanal:: Processed 1000 messages in 1.120584ms
flume:: Processed 1000 messages in 67.333µs
async_channel Processed 1000 messages in 75.875µs
loole:: Processed 1000 messages in 63.291µs
```
