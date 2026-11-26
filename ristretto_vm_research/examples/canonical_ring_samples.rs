//! Finite retained-heap ring measurements for benchmarks/gc/ring.
use ristretto_classfile::ClassFile;
use ristretto_gc::canonical::Configuration;
use ristretto_vm_research::canonical::{Error, Invocation, JavaThread, Limits, Program};
use sha2::{Digest, Sha256};
use std::fmt::Write;
use std::time::Instant;

const CLASSES: [(&str, &[u8]); 2] = [
    (
        "CanonicalRingWorkload.class",
        include_bytes!("../../ristretto_vm/tests/fixtures/canonical/CanonicalRingWorkload.class"),
    ),
    (
        "CanonicalRingWorkload$Node.class",
        include_bytes!(
            "../../ristretto_vm/tests/fixtures/canonical/CanonicalRingWorkload$Node.class"
        ),
    ),
];
const PAYLOAD_LIMIT: usize = 64 * 1024 * 1024;

trait Invoke {
    async fn run(&mut self, method: &str, steps: i32) -> Result<Invocation, Error>;
}
impl Invoke for JavaThread<'_, '_, '_> {
    async fn run(&mut self, method: &str, steps: i32) -> Result<Invocation, Error> {
        self.invoke("CanonicalRingWorkload", method, steps).await
    }
}
#[cfg(all(feature = "experimental-canonical-jit", not(target_family = "wasm")))]
impl Invoke for ristretto_vm_research::canonical::CompiledThread<'_, '_, '_, '_> {
    async fn run(&mut self, method: &str, steps: i32) -> Result<Invocation, Error> {
        self.invoke("CanonicalRingWorkload", method, steps).await
    }
}
struct Plan {
    method: String,
    steps: i32,
    expected: i32,
    warmup: usize,
    samples: usize,
}
struct Sample {
    elapsed_ns: u128,
    invocation: Invocation,
}

async fn sample(thread: &mut impl Invoke, plan: &Plan) -> Result<(Vec<Sample>, u128), Error> {
    let mut output = Vec::with_capacity(plan.samples);
    let mut batch_start = None;
    for iteration in 0..plan.warmup + plan.samples {
        let start = Instant::now();
        if iteration == plan.warmup {
            batch_start = Some(start);
        }
        let invocation = thread.run(&plan.method, plan.steps).await?;
        let elapsed_ns = start.elapsed().as_nanos();
        if invocation.result != plan.expected {
            return Err(Error::Invalid(format!(
                "ring checksum {}",
                invocation.result
            )));
        }
        if iteration >= plan.warmup {
            output.push(Sample {
                elapsed_ns,
                invocation,
            });
        }
    }
    let batch_ns = batch_start
        .ok_or(Error::Boundary("empty sample batch"))?
        .elapsed()
        .as_nanos();
    Ok((output, batch_ns))
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let tier = args.next().ok_or("missing tier")?;
    let plan = Plan {
        method: args.next().ok_or("missing small|medium|large")?,
        steps: args.next().ok_or("missing steps")?.parse()?,
        expected: args.next().ok_or("missing expected checksum")?.parse()?,
        warmup: args.next().ok_or("missing warmup")?.parse()?,
        samples: args.next().ok_or("missing samples")?.parse()?,
    };
    if args.next().is_some()
        || !(0..=100_000).contains(&plan.steps)
        || plan.warmup > 128
        || !(1..=128).contains(&plan.samples)
    {
        return Err("invalid finite invocation plan".into());
    }
    let retained = match plan.method.as_str() {
        "small" => 8,
        "medium" => 32,
        "large" => 128,
        _ => return Err("unknown workload".into()),
    };
    let configuration = Configuration {
        objects: retained * 4 + 8,
        roots: 128,
        mutators: 1,
        helpers: 1,
        edges_per_object: 3,
    };
    let program = Program::from_classfiles(
        CLASSES
            .iter()
            .map(|(_, bytes)| ClassFile::from_bytes(bytes))
            .collect::<Result<Vec<_>, _>>()?,
    )?;
    #[cfg(all(feature = "experimental-canonical-jit", not(target_family = "wasm")))]
    let compiled = if tier == "compiled" {
        Some(program.compile()?)
    } else {
        None
    };
    let completion = program
        .with_heap(configuration, PAYLOAD_LIMIT, 1_000_000, async |heap| {
            let samples = match tier.as_str() {
                "interpreted" => sample(&mut heap.attach(Limits::default())?, &plan).await?,
                #[cfg(all(feature = "experimental-canonical-jit", not(target_family = "wasm")))]
                "compiled" => {
                    let compiled = compiled
                        .as_ref()
                        .ok_or_else(|| Error::Invalid("missing compiled program".into()))?;
                    sample(
                        &mut compiled.attach(
                            heap,
                            ristretto_vm_research::canonical::CompiledLimits::default(),
                        )?,
                        &plan,
                    )
                    .await?
                }
                _ => return Err(Error::Unsupported(tier.clone())),
            };
            Ok((samples, heap.managed_bytes()))
        })
        .await?;
    println!(
        "revision\t{}",
        option_env!("RISTRETTO_GC_BENCH_REVISION").unwrap_or("unknown")
    );
    println!("debug_assertions\t{}", cfg!(debug_assertions));
    for (name, bytes) in CLASSES {
        let mut digest = String::with_capacity(64);
        for byte in Sha256::digest(bytes) {
            write!(&mut digest, "{byte:02x}")?;
        }
        println!("class\t{name}\t{digest}");
    }
    println!(
        "limits\t{}\t{}\t{}\t{}",
        configuration.objects, configuration.roots, configuration.edges_per_object, PAYLOAD_LIMIT
    );
    let ((rows, batch_ns), live_bytes) = completion.value;
    println!("batch_ns\t{batch_ns}");
    for row in rows {
        let invocation = row.invocation;
        println!(
            "sample\t{}\t{}\t{}\t{}\t{}",
            invocation.result,
            row.elapsed_ns,
            invocation.allocations,
            invocation.helper_polls,
            invocation.released_objects
        );
    }
    // with_heap verifies the final payload drain and catalog refund before returning.
    println!(
        "cleanup\t{}\t{}\t{}",
        live_bytes, completion.cleanup_polls, completion.released_during_cleanup
    );
    Ok(())
}
