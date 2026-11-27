//! Root registration must belong to the allocation's actual collector.
#![expect(clippy::panic_in_result_fn)]
#[path = "support/ownership.rs"]
mod ownership;
use ownership::collectors;
use ristretto_gc::{Error, Gc, GcWeak, ReferenceKind, Result};

#[test]
fn fallible_rooting_rejects_a_foreign_collector_without_registering() -> Result<()> {
    for (source, destination) in collectors().zip(collectors()) {
        let child = Gc::try_new(&source, 42_u64)?.into_rooted();
        let weak = GcWeak::new(&child, ReferenceKind::Weak, 0);
        let before_source = source.diagnostics().root_registrations;
        let before_destination = destination.diagnostics().root_registrations;
        assert!(matches!(
            child.as_root(&destination),
            Err(Error::InvalidPointer(_))
        ));
        assert_eq!(source.diagnostics().root_registrations, before_source);
        assert_eq!(
            destination.diagnostics().root_registrations,
            before_destination
        );
        source.collect_full()?;
        assert!(weak.upgrade().is_some());
        assert_eq!(*child, 42);
        drop(child);
        source.stop()?;
        destination.stop()?;
        assert_eq!(source.managed_bytes(), 0);
        assert_eq!(destination.managed_bytes(), 0);
    }
    Ok(())
}

#[cfg(panic = "unwind")]
#[test]
fn infallible_foreign_rooting_rejects_before_registration_and_cleans_up() -> Result<()> {
    for (source, destination) in collectors().zip(collectors()) {
        let child = Gc::try_new(&source, 42_u64)?.into_rooted();
        let weak = GcWeak::new(&child, ReferenceKind::Weak, 0);
        let before = destination.diagnostics().root_registrations;
        assert!(
            std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| destination.add_root(&child)))
                .is_err()
        );
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            destination.create_root_guard(child)
        }));
        assert!(result.is_err());
        assert_eq!(destination.diagnostics().root_registrations, before);
        assert_eq!(destination.root_statistics().compatibility_roots, 0);
        assert_eq!(destination.root_statistics().reserved_slots, 0);
        source.stop()?;
        destination.stop()?;
        assert!(weak.upgrade().is_none());
        assert_eq!(source.managed_bytes(), 0);
        assert_eq!(destination.managed_bytes(), 0);
    }
    Ok(())
}

#[test]
fn same_heap_guard_conversions_keep_owning_protection() -> Result<()> {
    for source in collectors() {
        for operation in 0..5 {
            let child = Gc::try_new(&source, 42_u64)?.into_rooted();
            let weak = GcWeak::new(&child, ReferenceKind::Weak, 0);
            let guard = if operation == 0 {
                child.as_root(&source)?
            } else {
                source.create_root_guard(child.clone())
            };
            drop(child);
            let escaped = match operation {
                0 | 1 => guard.into_rooted(),
                2 => guard.into_retained(),
                3 => {
                    let escaped = guard.clone_gc();
                    drop(guard);
                    escaped
                }
                _ => {
                    let cloned = guard.clone();
                    drop(guard);
                    cloned.into_rooted()
                }
            };
            source.collect_full()?;
            assert!(weak.upgrade().is_some());
            assert_eq!(*escaped, 42);
            drop(escaped);
        }
        source.stop()?;
        assert_eq!(source.managed_bytes(), 0);
    }
    Ok(())
}

#[test]
fn guarding_a_borrowed_field_registers_once_and_survives_unlink() -> Result<()> {
    for gc in collectors() {
        let context = gc.register_context();
        context.with(|| -> Result<()> {
            let leaf = Gc::try_new(&gc, 42_u64)?.into_rooted();
            let weak = GcWeak::new(&leaf, ReferenceKind::Weak, 0);
            let parent = Gc::try_new(&gc, vec![leaf])?;
            let before = gc.diagnostics().root_registrations;
            let guard = parent.first().expect("managed field").as_root(&gc)?;
            assert_eq!(gc.diagnostics().root_registrations - before, 1);
            let alias = guard.clone_gc();
            drop(parent);
            drop(guard);
            gc.collect_full()?;
            assert_eq!(*alias, 42);
            assert!(weak.upgrade().is_some());
            drop(alias);
            gc.collect_full()?;
            assert!(weak.upgrade().is_none());
            assert_eq!(gc.managed_bytes(), 0);
            Ok(())
        })?;
    }
    Ok(())
}

#[test]
fn a_replaced_guard_retains_the_foreign_replacements_own_registration() -> Result<()> {
    for (source, destination) in collectors().zip(collectors()) {
        let mut guard = Gc::try_new(&source, 7_u64)?;
        let replacement = Gc::try_new(&destination, 42_u64)?.into_rooted();
        let weak = GcWeak::new(&replacement, ReferenceKind::Weak, 0);
        let old = std::mem::replace(&mut *guard, replacement);
        drop(old);
        let escaped = guard.into_rooted();
        source.collect_full()?;
        destination.collect_full()?;
        assert!(weak.upgrade().is_some());
        assert_eq!(*escaped, 42);
        drop(escaped);
        source.stop()?;
        destination.stop()?;
        assert_eq!(source.managed_bytes(), 0);
        assert_eq!(destination.managed_bytes(), 0);
    }
    Ok(())
}
