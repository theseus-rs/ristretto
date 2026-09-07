#![expect(
    clippy::expect_used,
    reason = "benchmarks must fail instead of timing failed compiler operations"
)]

use criterion::{BenchmarkId, Criterion, SamplingMode, criterion_group, criterion_main};
use ristretto_classfile::ClassFile;
use ristretto_classloader::runtime;
use ristretto_vm::{Compiler, ConfigurationBuilder};
use std::error::Error;
use std::ffi::OsString;
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use tempfile::TempDir;

type Result<T = ()> = std::result::Result<T, Box<dyn Error>>;

// Use the file-compilation example, isolated from checked-in .class files and CLASSPATH.
const HELLO_WORLD: &str = include_str!("../../examples/compiler/HelloWorld.java");

struct Fixture {
    _directory: TempDir,
    arguments: Vec<OsString>,
    class_file: PathBuf,
}

impl Fixture {
    fn new() -> Result<Self> {
        let directory = tempfile::tempdir()?;
        let source_file = directory.path().join("HelloWorld.java");
        let classes = directory.path().join("classes");
        fs::write(&source_file, HELLO_WORLD)?;
        fs::create_dir(&classes)?;
        let arguments = vec![
            OsString::from("-classpath"),
            classes.as_os_str().to_owned(),
            OsString::from("-d"),
            classes.as_os_str().to_owned(),
            source_file.into_os_string(),
        ];
        Ok(Self {
            class_file: classes.join("HelloWorld.class"),
            arguments,
            _directory: directory,
        })
    }

    fn remove_output(&self) -> Result {
        match fs::remove_file(&self.class_file) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(error.into()),
        }
    }

    fn verify_output(&self) -> Result {
        let bytes = fs::read(&self.class_file)?;
        let class = ClassFile::from_bytes(&bytes)?;
        class.verify()?;
        if class.class_name()?.to_rust_string() != "HelloWorld" {
            return Err("compiler produced the wrong class".into());
        }
        Ok(())
    }

    // Include the compiler's input/output I/O, but exclude fixture cleanup and validation.
    fn measure(&self, iterations: u64, mut compile: impl FnMut()) -> Duration {
        let mut elapsed = Duration::ZERO;
        for _ in 0..iterations {
            self.remove_output()
                .expect("remove previous compiler output");
            let start = Instant::now();
            compile();
            elapsed += start.elapsed();
            self.verify_output()
                .expect("verify freshly generated class");
        }
        elapsed
    }
}

async fn new_compiler(interpreted: bool) -> Result<Compiler> {
    let configuration = ConfigurationBuilder::new()
        .interpreted(interpreted)
        .build()?;
    Ok(Compiler::new(configuration).await?)
}

fn benchmarks(criterion: &mut Criterion) {
    bench_compiler(criterion).expect("compiler benchmark setup must succeed");
}

fn bench_initialization(criterion: &mut Criterion, executor: &tokio::runtime::Runtime) {
    let mut group = criterion.benchmark_group("compiler/init");
    group
        .sampling_mode(SamplingMode::Flat)
        .warm_up_time(Duration::from_millis(500))
        .measurement_time(Duration::from_secs(1));
    for (mode, interpreted) in [("int", true), ("jit", false)] {
        group.bench_with_input(mode, &interpreted, |bencher, &interpreted| {
            bencher.iter(|| {
                executor.block_on(async {
                    let _compiler = new_compiler(interpreted)
                        .await
                        .expect("compiler initialization must succeed");
                });
            });
        });
    }
}

fn bench_compiler(criterion: &mut Criterion) -> Result {
    let mut builder = {
        #[cfg(target_family = "wasm")]
        {
            tokio::runtime::Builder::new_current_thread()
        }
        #[cfg(not(target_family = "wasm"))]
        {
            tokio::runtime::Builder::new_multi_thread()
        }
    };
    let executor = builder.enable_all().build()?;

    // Resolve/download the pinned default JDK before any timed iterations. Do not retain its
    // class loader: cold samples must still initialize their own compiler VM and class loaders.
    let (java_home, java_version, _) = executor.block_on(runtime::default_class_loader())?;
    eprintln!(
        "Compiler benchmark JDK: {java_version} ({})",
        java_home.display()
    );

    bench_initialization(criterion, &executor);
    let mut group = criterion.benchmark_group("compiler");
    group.sampling_mode(SamplingMode::Flat);

    for (mode, interpreted) in [("int", true), ("jit", false)] {
        let fixture = Fixture::new()?;

        group.bench_with_input(
            BenchmarkId::new("hello_world", mode),
            &interpreted,
            |bencher, &interpreted| {
                bencher.iter_custom(|iterations| {
                    fixture.measure(iterations, || {
                        executor.block_on(async {
                            let compiler = new_compiler(interpreted)
                                .await
                                .expect("compiler initialization must succeed");
                            compiler
                                .compile(&fixture.arguments)
                                .await
                                .expect("HelloWorld.java must compile");
                        });
                    })
                });
            },
        );

        // Warm one compiler per Criterion batch, outside the timer. Reuse it for that batch's
        // iterations to exclude VM startup and first-use javac loading, while bounding the
        // lifetime of retained Java allocations. javac still recompiles the file each time.
        group.bench_function(BenchmarkId::new("hello_world_warm", mode), |bencher| {
            let compiler = executor
                .block_on(new_compiler(interpreted))
                .expect("compiler initialization must succeed");
            fixture
                .remove_output()
                .expect("remove previous compiler output");
            executor
                .block_on(compiler.compile(&fixture.arguments))
                .expect("HelloWorld.java warmup must compile");
            fixture.verify_output().expect("verify warmup output");
            bencher.iter_custom(|iterations| {
                fixture.measure(iterations, || {
                    executor
                        .block_on(compiler.compile(&fixture.arguments))
                        .expect("HelloWorld.java must compile");
                })
            });
        });
    }
    Ok(())
}

criterion_group!(
    name = benches;
    config = Criterion::default()
        .sample_size(10)
        .measurement_time(Duration::from_secs(10));
    targets = benchmarks
);
criterion_main!(benches);
