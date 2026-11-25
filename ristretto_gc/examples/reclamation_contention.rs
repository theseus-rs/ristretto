//! Native saturation screen: a coordinator reclaims while producers repeatedly
//! publish live edges or acquire/release fresh roots over a configurable working
//! set. Deadlines retain unfinished cycles as censored work.
#[cfg(target_family = "wasm")]
fn main() {}

#[cfg(not(target_family = "wasm"))]
fn main() -> ristretto_gc::Result<()> {
    native::run()
}

#[cfg(not(target_family = "wasm"))]
mod native {
    use ristretto_gc::{Configuration, GarbageCollector, Gc, Result, RootStorage};
    use std::hint::black_box;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, Barrier};
    use std::thread;
    use std::time::{Duration, Instant};

    #[expect(
        clippy::print_stdout,
        clippy::panic_in_result_fn,
        clippy::too_many_lines,
        reason = "standalone correctness-checked benchmark with explicit timing boundaries"
    )]
    pub fn run() -> Result<()> {
        let args: Vec<_> = std::env::args().collect();
        let policy = args.get(1).map_or("candidate", String::as_str);
        let kind = args.get(2).map_or("full", String::as_str);
        let producers = args
            .get(3)
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(4);
        let cycles = args
            .get(4)
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(64);
        let seconds = args.get(5).and_then(|v| v.parse::<u64>().ok()).unwrap_or(5);
        let targets = args
            .get(6)
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(1);
        let operation = args.get(7).map_or("barriers", String::as_str);
        assert!(["barriers", "roots"].contains(&operation));
        let root_churn = operation == "roots";
        assert!(["baseline", "counted", "candidate"].contains(&policy));
        assert!(["full", "young"].contains(&kind));
        assert!((1..=32).contains(&producers) && (1..=10000).contains(&cycles));
        assert!((1..=3600).contains(&seconds));
        assert!((1..=256).contains(&targets) && targets.is_power_of_two());
        let gc = GarbageCollector::with_config(
            Configuration::builder()
                .concurrent_publication(policy != "baseline")
                .root_storage(if policy == "candidate" {
                    RootStorage::Contexts
                } else {
                    RootStorage::Tree
                })
                .barrier_buffer_size(if policy == "candidate" { 32 } else { 0 })
                .independent_phases(true)
                .metadata_shards(16)
                .incremental_step_size(16)
                .max_pause_time_us(0)
                .allocation_threshold(usize::MAX)
                .full_collection_interval(usize::MAX)
                .promotion_age(1)
                .diagnostics(true)
                .build(),
        );
        let roots = Arc::new(
            (0..targets)
                .map(|_| Gc::try_new(&gc, 42_u64).map(ristretto_gc::GcRootGuard::into_rooted))
                .collect::<Result<Vec<_>>>()?,
        );
        gc.collect_full()?; // promote the live edge before either generation screen
        let stop = Arc::new(AtomicBool::new(false));
        let measured = Arc::new(AtomicBool::new(false));
        let start = Arc::new(Barrier::new(producers + 1));
        let workers: Vec<_> = (0..producers)
            .map(|_| {
                let (gc, roots, stop, measured, start) = (
                    gc.clone(),
                    roots.clone(),
                    stop.clone(),
                    measured.clone(),
                    start.clone(),
                );
                thread::spawn(move || {
                    gc.register_context().with(|| {
                        // Rooting an owning handle only shares its registration.
                        // Heap edges force each measured acquisition to install a
                        // fresh registration. The original roots protect them.
                        let edges: Vec<_> = if root_churn {
                            roots
                                .iter()
                                .map(|root| {
                                    let mut edge = root.clone();
                                    // SAFETY: The original roots remain live throughout all uses of these borrowed edges.
                                    #[expect(
                                        unsafe_code,
                                        reason = "explicit non-owning edge fixture"
                                    )]
                                    unsafe {
                                        edge.unroot();
                                    };
                                    edge
                                })
                                .collect()
                        } else {
                            Vec::new()
                        };
                        let inputs = if root_churn { &edges } else { &*roots };
                        start.wait();
                        let mut count = 0_u64;
                        let mut checksum = 0_u64;
                        let mut cursor = 0_usize;
                        while !stop.load(Ordering::Acquire) {
                            #[expect(
                                clippy::indexing_slicing,
                                reason = "targets is a nonzero power of two and equals inputs.len()"
                            )]
                            let root = &inputs[cursor & (targets - 1)];
                            cursor = cursor.wrapping_add(1);
                            let value = if root_churn {
                                let local = black_box(root).rooted_clone();
                                let value = black_box(*local);
                                drop(local);
                                value
                            } else {
                                gc.write_barrier(black_box(root));
                                black_box(**root)
                            };
                            if measured.load(Ordering::Acquire) {
                                count += 1;
                                checksum = checksum.wrapping_add(value);
                            }
                            if count.is_multiple_of(64) {
                                thread::yield_now();
                            }
                        }
                        (count, checksum)
                    })
                })
            })
            .collect();
        start.wait();
        thread::sleep(Duration::from_millis(100));
        let before = gc.diagnostics();
        let start = Instant::now();
        let deadline = start + Duration::from_secs(seconds);
        measured.store(true, Ordering::Release);
        let mut cycle_ns = Vec::with_capacity(cycles);
        let mut unfinished_ns = None;
        let mut allocated = 0_usize;
        // No fallible early return can strand producers; stop/join follows this block.
        let execution = (|| -> Result<()> {
            for _ in 0..cycles {
                let cycle_start = Instant::now();
                for _ in 0..16 {
                    drop(Gc::try_new(&gc, 99_u64)?);
                    allocated += 1;
                }
                if kind == "full" {
                    gc.collect();
                } else {
                    gc.collect_young();
                }
                loop {
                    if Instant::now() >= deadline {
                        unfinished_ns = Some(cycle_start.elapsed().as_nanos());
                        return Ok(());
                    }
                    if gc.step() {
                        break;
                    }
                    thread::yield_now();
                }
                cycle_ns.push(cycle_start.elapsed().as_nanos());
            }
            Ok(())
        })();
        measured.store(false, Ordering::Release);
        let elapsed_ns = start.elapsed().as_nanos();
        stop.store(true, Ordering::Release);
        let mut operations = 0_u64;
        for worker in workers {
            let (count, checksum) = worker
                .join()
                .unwrap_or_else(|error| std::panic::resume_unwind(error));
            assert_eq!(checksum, count.wrapping_mul(42));
            operations += count;
        }
        execution?;
        let after = gc.diagnostics();
        let live_before_cleanup = gc.statistics()?.live_objects;
        assert!(roots.iter().all(|root| **root == 42));
        gc.collect_full()?;
        gc.collect_full()?;
        assert_eq!(gc.statistics()?.live_objects, targets);
        drop(roots);
        gc.collect_full()?;
        gc.stop()?;
        assert_eq!(gc.managed_bytes(), 0);
        let unfinished = unfinished_ns.map_or_else(|| "null".into(), |n| n.to_string());
        let barrier_operations = if root_churn { 0 } else { operations };
        println!(
            "{{\"policy\":\"{policy}\",\"kind\":\"{kind}\",\"operation\":\"{operation}\",\"producers\":{producers},\"targets\":{targets},\"cycles_requested\":{cycles},\"cycles_completed\":{},\"cycle_ns\":{cycle_ns:?},\"unfinished_cycle_ns\":{unfinished},\"elapsed_ns\":{elapsed_ns},\"producer_operations\":{operations},\"barrier_operations\":{barrier_operations},\"allocated_objects\":{allocated},\"live_before_cleanup\":{live_before_cleanup},\"publication_retries\":{},\"publication_transitions\":{},\"teardown_live\":0,\"teardown_bytes\":0,\"checksum_ok\":true,\"diagnostics\":true}}",
            cycle_ns.len(),
            after.publication_retries - before.publication_retries,
            after.publication_transitions - before.publication_transitions
        );
        Ok(())
    }
}
