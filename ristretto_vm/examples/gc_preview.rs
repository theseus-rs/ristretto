//! Run the source preview's Java object graph on the ordinary JVM heap.
#![forbid(unsafe_code)]
use ristretto_gc::{GcWeak, ReferenceKind};
use ristretto_vm::{ClassPath, ConfigurationBuilder, VM, Value};
use std::path::PathBuf;

const GRAPH: &str = "CanonicalGraphWorkload";
const NODE: &str = "CanonicalGraphWorkload$Node";
const CHECKSUM: &str = "(LCanonicalGraphWorkload$Node;LCanonicalGraphWorkload$Node;)I";

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let (home, classes, compiled) = match args.as_slice() {
        [home, classes] => (home, classes, false),
        [home, classes, flag] if flag == "--compiled" => (home, classes, true),
        _ => return Err("expected JDK-home classes-directory [--compiled]".into()),
    };
    let vm = VM::new(
        ConfigurationBuilder::new()
            .java_home(PathBuf::from(home))
            .class_path(ClassPath::from(&[PathBuf::from(classes)]))
            .batch_compilation(false)
            .interpreted(!compiled)
            .build()?,
    )
    .await?;
    let gc = vm.garbage_collector();
    gc.stop()?;
    if compiled {
        #[cfg(all(
            not(target_family = "wasm"),
            feature = "experimental-reference-dispatch"
        ))]
        {
            vm.prepare_experimental_jit(GRAPH, "run", "(I)I").await?;
            vm.prepare_experimental_jit(NODE, "<init>", "(I)V").await?;
            vm.prepare_experimental_jit(GRAPH, "checksum", CHECKSUM)
                .await?;
        }
        #[cfg(not(all(
            not(target_family = "wasm"),
            feature = "experimental-reference-dispatch"
        )))]
        return Err("compiled preview requires native reference dispatch".into());
    }
    let checkpoint = vm.execution_profile_checkpoint("one Java graph invocation");
    let result = vm.try_invoke(GRAPH, "run(I)I", &[8]).await?.as_i32()?;
    let interval = vm.execution_profile_since(checkpoint)?;
    let mut compiled_entries = 0;
    let mut compiled_backedges = 0;
    let mut interpreted_entries = 0;
    for (class, name, descriptor, calls) in [
        (GRAPH, "run", "(I)I", 1),
        (GRAPH, "checksum", CHECKSUM, 8),
        (NODE, "<init>", "(I)V", 16),
    ] {
        let activity = interval
            .methods
            .iter()
            .map(|m| &m.activity)
            .find(|m| m.class == class && m.name == name && m.descriptor == descriptor)
            .ok_or("missing graph method profile")?;
        if activity.interpreter_entries + activity.compiled_entries != calls
            || (compiled
                && (!activity.experimental_preparation || activity.compiled_entries != calls))
        {
            return Err(format!("unexpected dispatch for {class}.{name}").into());
        }
        compiled_entries += activity.compiled_entries;
        compiled_backedges += activity.compiled_backedges;
        interpreted_entries += activity.interpreter_entries;
    }

    // The bootstrap heap stays live. Observe two exact Java objects forming an
    // otherwise unreachable cycle and require a physical refund in this full GC.
    let witnesses = {
        let a = vm.object(NODE, "I", &[1]).await?;
        let b = vm.object(NODE, "I", &[2]).await?;
        a.as_object_mut()?.set_value("next", b.clone())?;
        b.as_object_mut()?.set_value("next", a.clone())?;
        let (Value::Object(Some(a)), Value::Object(Some(b))) = (&a, &b) else {
            return Err("expected Java nodes".into());
        };
        [
            GcWeak::new(a, ReferenceKind::Weak, 0),
            GcWeak::new(b, ReferenceKind::Weak, 0),
        ]
    };
    let before = gc.statistics()?;
    gc.collect_full()?;
    let after = gc.statistics()?;
    let cleared = witnesses.iter().filter(|link| link.is_cleared()).count();
    println!(
        "{}",
        serde_json::json!({"result":result, "compiled_entries":compiled_entries,
        "compiled_backedges":compiled_backedges,"interpreted_entries":interpreted_entries,
        "cleared_witnesses":cleared, "collections_completed":after.collections_completed-before.collections_completed,
        "bytes_freed":after.bytes_freed-before.bytes_freed})
    );
    Ok(())
}
