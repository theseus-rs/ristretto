//! Real publication guards, including unsuccessful acquisition and post-unlock time.
#![expect(
    clippy::panic_in_result_fn,
    reason = "correctness assertions with fallible clock setup"
)]
use super::*;
use crate::pause_recorder::PauseRecorder;

#[test]
fn publication_pause_records_only_successful_writer_and_counts_contention() -> std::io::Result<()> {
    let recorder = Arc::new(PauseRecorder::new(8)?);
    let gc = GarbageCollector::with_config(Configuration {
        pause_recorder: Some(Arc::clone(&recorder)),
        ..Configuration::default()
    });
    let reader = gc.allocation_guard();
    assert!(gc.try_publication_write().is_none());
    drop(reader);
    let writer = gc.try_publication_write();
    assert!(writer.is_some());
    assert!(gc.publication.try_read().is_none());
    drop(writer);
    let after_release = recorder.timestamp();
    assert!(gc.publication.try_read().is_some());
    drop(gc);
    let snapshot = Arc::try_unwrap(recorder)
        .map_err(|_| std::io::Error::other("recorder owner leaked"))?
        .finish();
    assert_eq!(snapshot.publication_attempts, 2);
    assert_eq!(snapshot.publication_failed_attempts, 1);
    assert_eq!(snapshot.publication_acquisitions, 1);
    assert_eq!(snapshot.events.len(), 1);
    assert!(snapshot.events[0].released_ns <= Some(after_release));
    assert_eq!(snapshot.open_events, 0);
    Ok(())
}

#[test]
fn concurrent_publication_bypass_does_not_fabricate_global_stop() -> std::io::Result<()> {
    let recorder = Arc::new(PauseRecorder::new(8)?);
    let gc = GarbageCollector::with_config(Configuration {
        pause_recorder: Some(Arc::clone(&recorder)),
        concurrent_publication: true,
        ..Configuration::default()
    });
    let writer = gc.try_publication_write();
    assert!(writer.is_some());
    assert!(gc.publication.try_read().is_some());
    drop(writer);
    drop(gc);
    let snapshot = Arc::try_unwrap(recorder)
        .map_err(|_| std::io::Error::other("recorder owner leaked"))?
        .finish();
    assert_eq!(snapshot.publication_attempts, 0);
    assert!(snapshot.events.is_empty());
    // This only establishes writer bypass, never the absence of other GC stalls.
    Ok(())
}

#[test]
fn measured_gc_counter_tracks_real_completed_collections()
-> std::result::Result<(), Box<dyn std::error::Error>> {
    let recorder = Arc::new(
        PauseRecorder::new(1024).map_err(|error| crate::Error::SyncError(error.to_string()))?,
    );
    let gc = GarbageCollector::with_config(Configuration {
        pause_recorder: Some(Arc::clone(&recorder)),
        ..Configuration::default()
    });
    gc.collect_full()?;
    let before = u64::try_from(gc.statistics()?.collections_completed)?;
    assert!(recorder.begin_measurement().is_some());
    gc.collect_full()?;
    let after = u64::try_from(gc.statistics()?.collections_completed)?;
    assert!(recorder.end_measurement().is_some());
    gc.collect_full()?;
    drop(gc);
    let snapshot = Arc::try_unwrap(recorder)
        .map_err(|_| crate::Error::SyncError("recorder owner leaked".to_owned()))?
        .finish();
    let begin = snapshot
        .measurement_begin
        .ok_or_else(|| crate::Error::SyncError("begin missing".to_owned()))?;
    let end = snapshot
        .measurement_end
        .ok_or_else(|| crate::Error::SyncError("end missing".to_owned()))?;
    assert_eq!(begin.collections_before, before);
    assert_eq!(begin.collections_after, before);
    assert_eq!(end.collections_before, after);
    assert_eq!(end.collections_after, after);
    assert_eq!(after - before, 1);
    Ok(())
}

#[test]
fn idle_steps_do_not_acquire_publication() -> std::io::Result<()> {
    const STEPS: usize = 2048;
    let recorder = Arc::new(PauseRecorder::new(STEPS)?);
    let gc = GarbageCollector::with_config(Configuration {
        pause_recorder: Some(Arc::clone(&recorder)),
        ..Configuration::default()
    });
    for _ in 0..STEPS {
        assert!(gc.step());
    }
    drop(gc);
    let snapshot = Arc::try_unwrap(recorder)
        .map_err(|_| std::io::Error::other("recorder owner leaked"))?
        .finish();
    assert_eq!(snapshot.publication_acquisitions, 0);
    assert_eq!(snapshot.publication_attempts, 0);
    Ok(())
}
