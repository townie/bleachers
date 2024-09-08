use std::time::Instant;

pub mod queue;
pub mod seqlock;
pub mod vector;

fn multithread(n_writers: usize, n_readers: usize, tot_messages: usize) {
    let q = queue::Queue::new(16, queue::QueueType::MPMC).unwrap();

    let mut readhandles = Vec::new();
    for n in 0..n_readers {
        let mut c1 = queue::Consumer::from(q);
        let cons = std::thread::spawn(move || {
            let mut c = 0;
            let mut m = 0;
            while c < tot_messages {
                while c1.try_consume(&mut m).is_err() {}
                c += m;
            }
            // assert_eq!(c, (0..tot_messages).sum::<usize>());
        });
        readhandles.push(cons)
    }
    let mut writehandles = Vec::new();
    for n in 0..n_writers {
        let mut p1 = queue::Producer::from(q);
        let prod1 = std::thread::spawn(move || {
            // std::thread::sleep(std::time::Duration::from_millis(20));
            let mut c = n;
            while c < tot_messages {
                p1.produce(&c);
                c += n_writers;
                std::thread::yield_now();
            }
        });
        writehandles.push(prod1);
    }

    for h in readhandles {
        h.join();
    }
    for h in writehandles {
        h.join();
    }
}

fn basic(num_messages: i32) {
    // for typ in [queue::QueueType::SPMC, queue::QueueType::MPMC] {
    let q = queue::Queue::new(100000, queue::QueueType::MPMC).unwrap();
    let mut p = queue::Producer::from(q);
    let mut c = queue::Consumer::from(q);
    let start = Instant::now();

    let prod1 = std::thread::spawn(move || {
        for i in 0..num_messages {
            // println!("prod {} ", i);
            p.produce(&i);
            // std::thread::sleep(std::time::Duration::from_nanos(1));
        }
    });
    // let mut final_message = 0;
    let cons1 = std::thread::spawn(move || {
        let mut m = 0;

        loop {
            while c.try_consume(&mut m).is_err() {
                // println!("cons {} ", m);
                if m == num_messages - 1 {
                    // final_message = m;
                    return;
                }
            }
        }
    });
    cons1.join();
    prod1.join();
    let duration = start.elapsed();
    println!("mantra::  {} messages in {:?}", num_messages, duration);
    // p.produce(&1);
    // let mut m = 0;

    // assert_eq!(c.try_consume(&mut m), Ok(()));
    // assert_eq!(m, 1);
    // assert!(matches!(
    //     c.try_consume(&mut m),
    //     Err(seqlock::ReadError::Empty)
    // ));
    // for i in 0..16 {
    //     p.produce(&i);
    // }
    // for i in 0..16 {
    //     c.try_consume(&mut m).unwrap();
    //     assert_eq!(m, i);
    // }

    // assert!(matches!(
    //     c.try_consume(&mut m),
    //     Err(seqlock::ReadError::Empty)
    // ));

    // for i in 0..20 {
    //     p.produce(&1);
    // }

    // assert!(matches!(
    //     c.try_consume(&mut m),
    //     Err(seqlock::ReadError::SpedPast)
    // ));
    // }
}

pub fn run_benchmark(num_messages: i32) {
    basic(num_messages);
    // multithread(1, 1, 1_000_000);
}
