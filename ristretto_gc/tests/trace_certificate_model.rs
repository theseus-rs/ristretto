//! The same certificate transitions as the production cell, with Loom atomics.
#![cfg(not(target_family = "wasm"))]
use loom::sync::Arc;
use loom::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use loom::thread;
#[path = "../src/trace_certificate.rs"]
mod state;
use state::Certificate;

#[test]
fn publication_exposes_the_completed_empty_projection() {
    loom::model(|| {
        let certificate = Arc::new(Certificate::default());
        let graph = Arc::new(AtomicUsize::new(1));
        let writer = {
            let (certificate, graph) = (certificate.clone(), graph.clone());
            thread::spawn(move || {
                graph.store(0, Ordering::Relaxed);
                certificate.publish();
            })
        };
        if certificate.certify() {
            assert_eq!(graph.load(Ordering::Relaxed), 0);
        }
        writer.join().unwrap();
        assert!(certificate.certify());
    });
}

#[test]
fn revocation_precedes_a_later_reference_bearing_writer() {
    loom::model(|| {
        let certificate = Arc::new(Certificate::default());
        let graph = Arc::new(AtomicUsize::new(0));
        certificate.publish();
        let writer = {
            let (certificate, graph) = (certificate.clone(), graph.clone());
            thread::spawn(move || {
                certificate.revoke();
                graph.store(1, Ordering::Release);
            })
        };
        if graph.load(Ordering::Acquire) == 1 {
            assert!(!certificate.certify());
        }
        writer.join().unwrap();
        assert!(!certificate.certify());
    });
}

#[test]
fn an_old_success_cannot_authorize_a_later_scan() {
    loom::model(|| {
        let certificate = Arc::new(Certificate::default());
        certificate.publish();
        assert!(certificate.certify());
        let writer = {
            let certificate = certificate.clone();
            thread::spawn(move || certificate.revoke())
        };
        writer.join().unwrap();
        // Every scan performs a fresh RMW; it cannot reuse the old success.
        assert!(!certificate.certify());
    });
}

#[test]
fn successive_guard_lifetimes_publish_fresh_empty_graphs() {
    loom::model(|| {
        let certificate = Arc::new(Certificate::default());
        let graph = Arc::new(AtomicUsize::new(0));
        certificate.publish();
        certificate.revoke();
        graph.store(1, Ordering::Relaxed);
        let writer = {
            let (certificate, graph) = (certificate.clone(), graph.clone());
            thread::spawn(move || {
                graph.store(0, Ordering::Relaxed);
                certificate.publish();
            })
        };
        if certificate.certify() {
            assert_eq!(graph.load(Ordering::Relaxed), 0);
        }
        writer.join().unwrap();
        certificate.revoke();
        assert!(!certificate.certify());
    });
}

#[test]
fn coverage_publication_exposes_all_completed_root_registrations() {
    loom::model(|| {
        let certificate = Arc::new(Certificate::default());
        let roots = Arc::new(AtomicUsize::new(0));
        let writer = {
            let (certificate, roots) = (certificate.clone(), roots.clone());
            thread::spawn(move || {
                roots.store(3, Ordering::Relaxed);
                certificate.publish();
            })
        };
        if certificate.certify() {
            assert_eq!(roots.load(Ordering::Relaxed), 3);
        }
        writer.join().unwrap();
    });
}

#[test]
#[ignore = "Loom 0.7.2 RMW/store atomicity bug #254; run benchmarks/gc/trace_certificate_check.py"]
fn a_covered_scan_retains_root_or_deletion_barrier_protection() {
    // This model assumes the root protocol publishes deletion shading before
    // releasing the root, in one active epoch. It checks certificate composition,
    // not the collector's admission/phase protocol or root-storage implementation.
    loom::model(|| {
        const ROOT: usize = 1;
        const SHADED: usize = 2;
        let certificate = Arc::new(Certificate::default());
        let protection = Arc::new(AtomicUsize::new(ROOT));
        certificate.publish();
        let writer = {
            let (certificate, protection) = (certificate.clone(), protection.clone());
            thread::spawn(move || {
                certificate.revoke();
                protection.fetch_or(SHADED, Ordering::Release);
                protection.fetch_and(!ROOT, Ordering::Release);
            })
        };
        if certificate.certify() {
            assert_ne!(protection.load(Ordering::Acquire) & (ROOT | SHADED), 0);
        }
        writer.join().unwrap();
        assert!(!certificate.certify());
    });
}
