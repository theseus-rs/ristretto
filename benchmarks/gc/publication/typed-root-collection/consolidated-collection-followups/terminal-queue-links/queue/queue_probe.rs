#![allow(dead_code)]
mod publication_queue;
use publication_queue::Queue;
use std::hint::black_box;
use std::time::Instant;

fn round(shape: &str, count: usize) {
    let queue = Queue::new();
    match shape {
        "alternating" => {
            for n in 0..count {
                queue.push(black_box(n));
                assert_eq!(queue.pop(), Some(n));
            }
        }
        "burst" => {
            for base in (0..count).step_by(64) {
                for n in base..base + 64 { queue.push(black_box(n)); }
                for n in base..base + 64 { assert_eq!(queue.pop(), Some(n)); }
            }
        }
        "retained" => {
            let owner = queue.prepare(usize::MAX);
            for n in 0..count { queue.push(black_box(n)); }
            drop(queue);
            drop(owner);
            return;
        }
        _ => panic!("unknown shape"),
    }
    assert!(queue.is_empty());
    assert_eq!(queue.pending(), 0);
    assert_eq!(queue.retained_nodes(), 2);
}

fn main() {
    let shape = std::env::args().nth(1).unwrap();
    let count = 8192;
    for _ in 0..2 { round(&shape, count); }
    let mut samples = Vec::with_capacity(7);
    for _ in 0..7 {
        let start = Instant::now();
        round(&shape, count);
        samples.push(start.elapsed().as_nanos());
    }
    println!("{{\"times_ns\":{samples:?},\"node_bytes\":{:?}}}", publication_queue::node_sizes());
}
