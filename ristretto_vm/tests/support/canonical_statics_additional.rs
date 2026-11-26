#![cfg(feature = "experimental-canonical-runtime")]
#![expect(clippy::panic_in_result_fn, reason = "integration assertions")]
use ristretto_classfile::{ClassFile, FieldAccessFlags};
use ristretto_gc::canonical::Configuration;
use ristretto_vm::canonical::{Error, Limits, Program};
type Result<T = ()> = std::result::Result<T, Box<dyn std::error::Error>>;
fn files() -> Result<Vec<ClassFile<'static>>> {
    let bytes: [&[u8]; 12] = [
        include_bytes!("../fixtures/canonical/CanonicalStaticRootsSmall.class"),
        include_bytes!("../fixtures/canonical/CanonicalStaticRootsSmall$Node.class"),
        include_bytes!("../fixtures/canonical/CanonicalStaticRootsFull.class"),
        include_bytes!("../fixtures/canonical/CanonicalStaticRootsFull$Node.class"),
        include_bytes!("../fixtures/canonical/CanonicalStaticProtocol.class"),
        include_bytes!("../fixtures/canonical/CanonicalStaticProtocol$Journal.class"),
        include_bytes!("../fixtures/canonical/CanonicalStaticProtocol$A.class"),
        include_bytes!("../fixtures/canonical/CanonicalStaticProtocol$B.class"),
        include_bytes!("../fixtures/canonical/CanonicalStaticProtocol$Lazy.class"),
        include_bytes!("../fixtures/canonical/CanonicalStaticProtocol$Fieldless.class"),
        include_bytes!("../fixtures/canonical/CanonicalStaticProtocol$Bad.class"),
        include_bytes!("../fixtures/canonical/CanonicalStaticProtocol$NestedBad.class"),
    ];
    Ok(bytes
        .into_iter()
        .map(ClassFile::from_bytes)
        .collect::<std::result::Result<Vec<_>, _>>()?)
}
fn configuration() -> Configuration {
    Configuration {
        objects: 4096,
        roots: 64,
        mutators: 1,
        helpers: 1,
        edges_per_object: 8,
    }
}
#[tokio::test]
async fn static_roots_and_initialization_match_hotspot() -> Result {
    let program = Program::from_classfiles(files()?)?;
    let mut rows = 0;
    for line in include_str!("../fixtures/canonical/static-oracle.tsv").lines() {
        let fields: Vec<_> = line.split('\t').collect();
        let [owner, method, argument, expected] = fields.as_slice() else {
            return Err("oracle fields".into());
        };
        let result = program
            .run(
                owner,
                method,
                argument.parse()?,
                Configuration {
                    objects: if owner.ends_with("Full") { 4096 } else { 64 },
                    ..configuration()
                },
                Limits {
                    cleanup_polls: 5_000_000,
                    ..Limits::default()
                },
            )
            .await;
        println!("{line}: {result:?}");
        if let Ok(expected) = expected.parse::<i32>() {
            let report = result?;
            assert_eq!(report.result, expected);
            assert_eq!(report.allocations, report.released_objects);
            assert_eq!(report.payload_bytes_after_cleanup, 0);
            assert_eq!(report.catalog_bytes_after_drop, 0);
            if owner.starts_with("CanonicalStaticRoots") {
                assert_eq!(report.static_storage_allocations, 1);
                assert_eq!(report.initialized_classes, 1);
                assert_eq!(report.class_initializer_calls, 1);
                assert_eq!(
                    report.entry_initialization.allocations,
                    if owner.ends_with("Full") { 295 } else { 5 }
                );
                assert!(report.entry_initialization.instructions > 0);
                assert_eq!(report.entry_initialization.calls, 1);
            } else {
                assert_eq!(report.entry_initialization.allocations, 0);
                if *method == "fieldless" {
                    assert_eq!(report.static_storage_allocations, 1);
                }
                if *method == "noTrigger" {
                    assert_eq!(report.class_initializer_calls, 0);
                }
            }
        } else {
            let Err(Error::Initialization { kind, cause }) = result else {
                return Err("expected initialization error".into());
            };
            assert_eq!(kind, "ExceptionInInitializerError");
            assert!(matches!(
                cause.as_deref(),
                Some(Error::ArrayFault("NegativeArraySizeException"))
            ));
        }
        rows += 1;
    }
    assert_eq!(rows, 16);
    Ok(())
}
#[tokio::test]
async fn static_capacity_rejects_before_execution() -> Result {
    let program = Program::from_classfiles(files()?)?;
    for config in [
        Configuration {
            roots: 1,
            ..configuration()
        },
        Configuration {
            edges_per_object: 1,
            ..configuration()
        },
    ] {
        assert!(
            program
                .run(
                    "CanonicalStaticRootsSmall",
                    "run",
                    0,
                    config,
                    Limits::default()
                )
                .await
                .is_err()
        );
    }
    Ok(())
}
#[test]
fn unsupported_static_final_volatile_and_opcode_kinds_reject() -> Result {
    for (field_name, flags) in [
        (
            "rewrites",
            FieldAccessFlags::STATIC | FieldAccessFlags::FINAL,
        ),
        (
            "rewrites",
            FieldAccessFlags::STATIC | FieldAccessFlags::VOLATILE,
        ),
        ("rewrites", FieldAccessFlags::empty()),
    ] {
        let mut classes = files()?;
        let file = classes.first_mut().ok_or("class")?;
        let field = file
            .fields
            .iter_mut()
            .find(|field| {
                file.constant_pool
                    .try_get_utf8(field.name_index)
                    .is_ok_and(|name| name == field_name)
            })
            .ok_or("field")?;
        field.access_flags = flags;
        assert!(Program::from_classfiles(classes).is_err());
    }
    Ok(())
}
#[cfg(all(feature = "experimental-canonical-jit", not(target_family = "wasm")))]
#[test]
fn admitted_static_program_prepares_request_only_native_methods() -> Result {
    let program = Program::from_classfiles(files()?)?;
    let compiled = program.compile()?;
    let profiles = compiled.method_profiles()?;
    assert_eq!(profiles.len(), 47);
    assert_eq!(
        profiles
            .iter()
            .filter(|method| method.name == "<clinit>")
            .count(),
        8
    );
    assert!(profiles.iter().all(|method| method.entries == 0));
    Ok(())
}

#[cfg(feature = "experimental-canonical-telemetry")]
fn telemetry() -> Result<ristretto_vm::canonical_telemetry::Telemetry> {
    use ristretto_vm::canonical_telemetry::{Binding, Telemetry, current_process_id};
    Ok(Telemetry::new(
        65_536,
        Binding {
            process_id: current_process_id(),
            run_id: "static-initialization-correctness".to_owned(),
            source_sha256: "0".repeat(64),
            binary_sha256: "0".repeat(64),
            workload_sha256: "0".repeat(64),
            configuration_sha256: "0".repeat(64),
        },
    )?)
}
#[cfg(feature = "experimental-canonical-telemetry")]
#[tokio::test]
async fn entry_initialization_is_reported_outside_body_window() -> Result {
    use ristretto_vm::canonical_telemetry::{CycleCapture, Kind, Operation, Outcome};
    let program = Program::from_classfiles(files()?)?;
    let telemetry = telemetry()?;
    let report = program
        .run_recorded(
            "CanonicalStaticRootsSmall",
            "prepared",
            0,
            Configuration {
                objects: 64,
                ..configuration()
            },
            Limits::default(),
            &telemetry,
        )
        .await?;
    let capture = telemetry.finish().map_err(|_| "recorder owner escaped")?;
    assert!(
        capture.snapshot.instrumented_capture_complete,
        "{:?}",
        capture.snapshot.issues
    );
    assert!(!capture.snapshot.global_capture_complete);
    assert!(matches!(capture.cycles, CycleCapture::Complete(_)));
    let events = &capture.snapshot.events;
    let window = events
        .iter()
        .find(|event| event.kind == Kind::Measurement)
        .ok_or("window")?;
    let allocations = events.iter().filter(|event| {
        event.kind == Kind::Request
            && event.operation == Operation::Allocation
            && event.outcome == Some(Outcome::Complete)
    });
    assert_eq!(allocations.clone().count(), report.allocations);
    assert_eq!(
        allocations
            .filter(|event| event.end_ns.is_some_and(|end| end <= window.start_ns))
            .count(),
        report.entry_initialization.allocations
    );
    assert_eq!(
        events
            .iter()
            .filter(|event| event.kind == Kind::HelperPoll
                && event.end_ns.is_some_and(|end| end <= window.start_ns))
            .count(),
        report.entry_initialization.helper_polls
    );
    assert_eq!(report.entry_initialization.allocations, 5);
    assert_eq!(report.allocations, 5);
    Ok(())
}
#[cfg(feature = "experimental-canonical-telemetry")]
#[tokio::test]
async fn failed_or_cancelled_entry_initialization_has_no_body_window() -> Result {
    use ristretto_vm::canonical_telemetry::{CycleCapture, Kind};
    use std::future::Future;
    use std::task::{Context, Waker};
    let program = Program::from_classfiles(files()?)?;
    for cancel in [false, true] {
        let telemetry = telemetry()?;
        if cancel {
            let mut pending = Box::pin(program.run_recorded(
                "CanonicalStaticRootsSmall",
                "prepared",
                0,
                Configuration {
                    objects: 64,
                    ..configuration()
                },
                Limits::default(),
                &telemetry,
            ));
            assert!(
                pending
                    .as_mut()
                    .poll(&mut Context::from_waker(Waker::noop()))
                    .is_pending()
            );
            drop(pending);
        } else {
            assert!(matches!(
                program
                    .run_recorded(
                        "CanonicalStaticProtocol$Bad",
                        "run",
                        0,
                        Configuration {
                            objects: 64,
                            ..configuration()
                        },
                        Limits::default(),
                        &telemetry
                    )
                    .await,
                Err(Error::Initialization {
                    kind: "ExceptionInInitializerError",
                    ..
                })
            ));
        }
        let capture = telemetry.finish().map_err(|_| "recorder owner escaped")?;
        assert!(!capture.snapshot.instrumented_capture_complete);
        assert_eq!(capture.cycles, CycleCapture::Unobserved);
        assert!(
            !capture
                .snapshot
                .events
                .iter()
                .any(|event| event.kind == Kind::Measurement)
        );
        assert_eq!(capture.snapshot.open_events, 0);
    }
    Ok(())
}

#[test]
fn unsupported_scalar_and_static_reference_quotas_reject() -> Result {
    use ristretto_classfile::{Field, FieldType};
    for (descriptor, count) in [("J", 1), ("Ljava/lang/Object;", 9)] {
        let mut classes = files()?;
        let file = classes.first_mut().ok_or("class")?;
        for index in 0..count {
            file.fields.push(Field {
                access_flags: FieldAccessFlags::STATIC,
                name_index: file.constant_pool.add_utf8(format!("extraStatic{index}"))?,
                descriptor_index: file.constant_pool.add_utf8(descriptor)?,
                field_type: FieldType::parse(descriptor)?,
                attributes: vec![],
            });
        }
        assert!(matches!(
            Program::from_classfiles(classes),
            Err(Error::Unsupported(_))
        ));
    }
    Ok(())
}

#[tokio::test]
async fn constant_values_are_visible_to_recursive_initializer_reads() -> Result {
    use ristretto_classfile::attributes::{Attribute, Instruction};
    for mutable in [false, true] {
        let mut classes = files()?;
        let file = classes.first_mut().ok_or("class")?;
        if mutable {
            let field = file
                .fields
                .iter_mut()
                .find(|field| {
                    file.constant_pool
                        .try_get_utf8(field.name_index)
                        .is_ok_and(|name| name == "SLOTS")
                })
                .ok_or("constant field")?;
            field.access_flags.remove(FieldAccessFlags::FINAL);
        }
        let field = file
            .constant_pool
            .add_field_ref(file.this_class, "SLOTS", "I")?;
        let initializer = file
            .methods
            .iter_mut()
            .find(|method| {
                file.constant_pool
                    .try_get_utf8(method.name_index)
                    .is_ok_and(|name| name == "<clinit>")
            })
            .ok_or("initializer")?;
        let (code, metadata) = initializer
            .attributes
            .iter_mut()
            .find_map(|attribute| {
                if let Attribute::Code {
                    code,
                    attributes,
                    exception_table,
                    ..
                } = attribute
                {
                    assert_eq!(exception_table.as_slice(), []);
                    Some((code, attributes))
                } else {
                    None
                }
            })
            .ok_or("initializer code")?;
        assert!(matches!(code.first(), Some(Instruction::Bipush(9))));
        // Valid bytecode reads ConstantValue through the recursive initialization
        // gate instead of javac's inlined literal, for both final and mutable int.
        *code.first_mut().ok_or("first instruction")? = Instruction::Getstatic(field);
        // This straight-line initializer has only line metadata. Getstatic is
        // one byte wider than bipush, so re-emit every later source-line offset.
        for attribute in metadata {
            let Attribute::LineNumberTable { line_numbers, .. } = attribute else {
                return Err("unexpected initializer metadata".into());
            };
            for line in line_numbers {
                if line.start_pc > 0 {
                    line.start_pc = line.start_pc.checked_add(1).ok_or("line offset")?;
                }
            }
        }
        let mut encoded = Vec::new();
        file.to_bytes(&mut encoded)?;
        *file = ClassFile::from_bytes(&encoded)?.into_owned();
        let program = Program::from_classfiles(classes)?;
        let report = program
            .run(
                "CanonicalStaticRootsSmall",
                "prepared",
                0,
                Configuration {
                    objects: 64,
                    ..configuration()
                },
                Limits::default(),
            )
            .await?;
        assert_eq!(report.result, 19);
        assert_eq!(report.class_initializer_calls, 1);
        assert_eq!(report.allocations, report.released_objects);
    }
    Ok(())
}
