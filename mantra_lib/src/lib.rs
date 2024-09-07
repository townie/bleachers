use std::cell::UnsafeCell;
use std::sync::atomic::{compiler_fence, AtomicI32, AtomicUsize, Ordering};
use std::time::Instant;

use core_affinity::CoreId;

#[derive(Default)]
#[repr(align(64))]
pub struct Seqlock<T> {
    version: AtomicUsize,
    data: UnsafeCell<T>,
}
unsafe impl<T: Send> Send for Seqlock<T> {}
unsafe impl<T: Sync> Sync for Seqlock<T> {}

impl<T: Copy> Seqlock<T> {
    pub fn new(data: T) -> Self {
        Self {
            version: AtomicUsize::new(0),
            data: UnsafeCell::new(data),
        }
    }
    #[inline(never)]
    pub fn read(&self, result: &mut T) {
        loop {
            let v1 = self.version.load(Ordering::Acquire);
            compiler_fence(Ordering::AcqRel);
            *result = unsafe { *self.data.get() };
            compiler_fence(Ordering::AcqRel);
            let v2 = self.version.load(Ordering::Acquire);
            if v1 == v2 && v1 & 1 == 0 {
                return;
            }
        }
    }

    #[inline(never)]
    pub fn write(&self, val: &T) {
        let v = self.version.fetch_add(1, Ordering::Release);
        compiler_fence(Ordering::AcqRel);
        unsafe { *self.data.get() = *val };
        compiler_fence(Ordering::AcqRel);
        self.version.store(v.wrapping_add(2), Ordering::Release);
    }
}

#[repr(align(64))]
struct Test(AtomicI32);

// fn one_way_2_lines(n_samples: usize) {}

pub fn run_benchmark(num_messages: i32) {
    // let (tx, rx) = mpsc::channel();

    let start = Instant::now();
    let seq1 = Test(AtomicI32::new(-1));
    let seq2 = Test(AtomicI32::new(-1));
    std::thread::scope(|s| {
        s.spawn(|| {
            core_affinity::set_for_current(CoreId { id: 2 });
            for n in 0..num_messages {
                // while seq1.0.load(Ordering::Acquire) != n {}
                seq2.0.store(n, Ordering::Release);
            }
        });
        s.spawn(|| {
            core_affinity::set_for_current(CoreId { id: 3 });
            // loop {
            //     // seq1.0.store(n, Ordering::Release);
            //     loop {
            //         // seq2.0.load(Ordering::Acquire)
            //         // println!("{} {}", seq2.0.load(Ordering::Acquire), n);
            //     }
            // }
        });
    });
    // let producer = thread::spawn(move || {
    //     for i in 0..num_messages {
    //         tx.send(i).unwrap();
    //     }
    // });

    // let consumer = thread::spawn(move || loop {
    //     match rx.recv() {
    //         Ok(val) => {
    //             if val == num_messages {
    //                 break;
    //             }
    //         }
    //         Err(e) => {
    //             break;
    //         }
    //     }
    // });

    // producer.join().unwrap();
    // consumer.join().unwrap();

    let duration = start.elapsed();
    println!("mantra::  {} messages in {:?}", num_messages, duration);
}

#[derive(Clone, Copy)]
struct TimingMessage {
    rdtscp: Instant,
    data: [u8; 1],
}

fn contender(lock: &Seqlock<TimingMessage>) {
    let mut m = TimingMessage {
        rdtscp: Instant::now(),
        data: [0],
    };
    while m.data[0] == 0 {
        lock.read(&mut m);
    }
}

// fn timed_consumer(lock: &Seqlock<TimingMessage>) {
//     core_affinity::set_for_current(CoreId { id: 1 });
//     let mut m = TimingMessage {
//         rdtscp: Instant::now(),
//         data: [0],
//     };
//     let mut last = m.rdtscp;
//     while m.data[0] == 0 {
//         timer.start();
//         lock.read(&mut m);
//         if m.rdtscp != last {
//             timer.stop();
//             timer.latency_till_stop(m.rdtscp);
//         }
//         last = m.rdtscp;
//     }
// }

// fn producer(lock: &Seqlock<TimingMessage>) {
//     // let mut timer = Timer::new("write");

//     core_affinity::set_for_current(CoreId { id: 2 });
//     let mut m = TimingMessage {
//         rdtscp: Instant::now(),
//         data: [0],
//     };
//     // let curt = Instant::now();
//     // while curt.elapsed() < Nanos::from_secs(5) {
//     //     // timer.start();
//     //     m.rdtscp = Instant::now();
//     //     lock.write(&m);
//     //     // timer.stop();
//     //     let curt = Instant::now();
//     //     while Instant::now() - curt < Nanos::from_micros(2) {}
//     // }
//     m.data[0] = 1;
//     lock.write(&m);
// }

// fn consumer_latency(n_contenders: usize) {
//     let lock = Seqlock::default();
//     std::thread::scope(|s| {
//         for i in 1..(n_contenders + 1) {
//             let lck = &lock;
//             s.spawn(move || {
//                 core_affinity::set_for_current(CoreId { id: i + 2 });
//                 contender(lck);
//             });
//         }
//         s.spawn(|| timed_consumer(&lock));
//         s.spawn(|| producer(&lock));
//     })
// }
