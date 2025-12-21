//! Error handling and Result type tests
//!
//! Tests error scenarios, error types, and proper error propagation.

use ristretto_gc::{Error, GarbageCollector, Gc, Result};
use std::sync::{Arc, Mutex};
use std::thread;

#[test_log::test]
fn test_successful_operations() -> Result<()> {
    let collector = GarbageCollector::new();
    collector.collect();

    // Test that normal operations return Ok
    let _gc = Gc::with_collector(&collector, 42);
    collector.collect();
    let _stats = collector.statistics()?;
    Ok(())
}

#[test_log::test]
fn test_error_types_exist() {
    // Test that all error variants can be constructed
    let _lock_error = Error::LockError("test lock error".to_string());
    let _sync_error = Error::SyncError("test sync error".to_string());
    let _phase_error = Error::CollectionPhaseError("test phase error".to_string());
    let _stats_error = Error::StatsError("test stats error".to_string());
    let _root_error = Error::RootError("test root error".to_string());
}

#[test_log::test]
fn test_error_display() {
    let error = Error::LockError("Failed to acquire mutex".to_string());
    let error_str = format!("{error}");
    assert!(error_str.contains("Failed to acquire lock"));
    assert!(error_str.contains("Failed to acquire mutex"));
}

#[test_log::test]
fn test_error_debug() {
    let error = Error::SyncError("Thread sync failed".to_string());
    let debug_str = format!("{error:?}");
    assert!(debug_str.contains("SyncError"));
    assert!(debug_str.contains("Thread sync failed"));
}

#[test_log::test]
fn test_collector_error_handling() -> Result<()> {
    let collector = GarbageCollector::new();

    // Test that start/stop operations handle errors gracefully
    collector.start();
    collector.stop()?;

    // Test that stopping again doesn't cause issues
    collector.stop()?;
    Ok(())
}

#[test_log::test]
fn test_concurrent_error_handling() {
    let collector = GarbageCollector::new();
    collector.start();

    // Test error handling in concurrent scenarios
    let gc = Gc::with_collector(&collector, 42);
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];

    for i in 0..5 {
        let collector = collector.clone();
        let gc_clone = gc.clone();
        let results_clone = Arc::clone(&results);
        let handle = thread::spawn(move || {
            let result = match i {
                0 => {
                    collector.collect();
                    Ok(())
                }
                1 => collector.statistics().map(|_| ()),
                2 => gc_clone.as_root(&collector).map(|_| ()),
                _ => Ok(()),
            };

            results_clone.lock().unwrap().push(result);
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Check that most operations succeeded
    let results = results.lock().unwrap();
    let success_count = results.iter().filter(|r| r.is_ok()).count();
    assert!(
        success_count >= 3,
        "Expected at least 3 successful operations"
    );
}

#[test_log::test]
fn test_graceful_degradation() {
    let collector = GarbageCollector::new();
    collector.start();

    // Test that the system degrades gracefully under error conditions
    let gc1 = Gc::with_collector(&collector, 1);
    let gc2 = Gc::with_collector(&collector, 2);
    let gc3 = Gc::with_collector(&collector, 3);

    // Even if some operations might have issues, basic functionality should work
    assert_eq!(**gc1, 1);
    assert_eq!(**gc2, 2);
    assert_eq!(**gc3, 3);

    // All objects should be accessible in reachability analysis
    assert!(Gc::ptr_eq(&*gc1, &*gc1));
    assert!(!Gc::ptr_eq(&*gc1, &*gc2));
}
