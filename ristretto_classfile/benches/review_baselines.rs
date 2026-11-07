//! Reproducible microbenchmarks for classfile review follow-ups.
#![expect(
    clippy::expect_used,
    reason = "benchmark fixtures must fail loudly if invalid"
)]
use criterion::{BatchSize, Criterion, criterion_group, criterion_main};
use ristretto_classfile::attributes::{
    Attribute, ExceptionTableEntry, Instruction, StackFrame, VerificationType,
};
use ristretto_classfile::verifiers::bytecode::{
    VerificationCache, VerifierConfig, verify_class_cached, verify_method,
};
use ristretto_classfile::verifiers::context::VerificationContext;
use ristretto_classfile::verifiers::{error, signature};
use ristretto_classfile::{
    ClassFile, Constant, JAVA_5, JAVA_6, JAVA_8, JavaStr, JavaString, Method, MethodAccessFlags,
};
use std::hint::black_box;

// Fixtures use only Object and Throwable; no application hierarchy is resolved.
struct Context;
impl VerificationContext for Context {
    fn verify_member_access(
        &self,
        _access: &ristretto_classfile::verifiers::context::MemberAccess<'_>,
    ) -> error::Result<()> {
        // This fixture models accessible members; access-policy failures have dedicated cases.
        Ok(())
    }

    fn cache_token(&self) -> Option<u64> {
        Some(1)
    }

    fn is_subclass(&self, child: &JavaStr, parent: &JavaStr) -> error::Result<bool> {
        Ok(child == parent || parent == "java/lang/Object" || parent == "java/lang/Throwable")
    }

    fn is_assignable(&self, target: &JavaStr, source: &JavaStr) -> error::Result<bool> {
        self.is_subclass(source, target)
    }

    fn common_superclass(&self, _: &JavaStr, _: &JavaStr) -> error::Result<JavaString> {
        Ok("java/lang/Object".into())
    }
}

fn fixture(method_count: usize, operations: usize, handlers: bool) -> ClassFile<'static> {
    let mut class = ClassFile {
        version: if handlers { JAVA_8 } else { JAVA_5 },
        ..ClassFile::default()
    };
    class.this_class = class.constant_pool.add_class("Benchmark").expect("class");
    class.super_class = class
        .constant_pool
        .add_class("java/lang/Object")
        .expect("superclass");
    let descriptor_index = class.constant_pool.add_utf8("()V").expect("descriptor");
    let code_name = class.constant_pool.add_utf8("Code").expect("Code");
    let stackmap_name = class
        .constant_pool
        .add_utf8("StackMapTable")
        .expect("StackMapTable");
    let throwable = class
        .constant_pool
        .add_class("java/lang/Throwable")
        .expect("Throwable");
    let constant = class
        .constant_pool
        .add(Constant::Integer(42))
        .expect("constant");
    for n in 0..method_count {
        let mut code = Vec::new();
        for _ in 0..operations {
            code.extend([Instruction::Ldc_w(constant), Instruction::Pop]);
        }
        code.push(Instruction::Return);
        let mut attributes = Vec::new();
        let mut exception_table = Vec::new();
        if handlers {
            let handler_pc = u16::try_from(operations * 4 + 1).expect("code size");
            code.extend([Instruction::Pop, Instruction::Return]);
            exception_table.push(ExceptionTableEntry {
                range_pc: 0..handler_pc - 1,
                handler_pc,
                catch_type: 0,
            });
            attributes.push(Attribute::StackMapTable {
                name_index: stackmap_name,
                frames: vec![StackFrame::FullFrame {
                    frame_type: 255,
                    offset_delta: handler_pc,
                    locals: vec![],
                    stack: vec![VerificationType::Object {
                        cpool_index: throwable,
                    }],
                }],
            });
        }
        class.methods.push(Method {
            access_flags: MethodAccessFlags::PUBLIC | MethodAccessFlags::STATIC,
            name_index: class
                .constant_pool
                .add_utf8(["method", &n.to_string()].concat())
                .expect("method name"),
            descriptor_index,
            attributes: vec![Attribute::Code {
                name_index: code_name,
                max_stack: 1,
                max_locals: if handlers { 64 } else { 0 },
                code,
                exception_table,
                attributes,
            }],
        });
    }
    class.verify().expect("valid class fixture");
    class
}

fn benchmarks(c: &mut Criterion) {
    let class = fixture(32, 8, false);
    let mut bytes = Vec::new();
    class.to_bytes(&mut bytes).expect("serialize fixture");
    let config = VerifierConfig::default();
    let cache = VerificationCache::new(true);
    verify_class_cached(&class, &Context, &config, &cache).expect("valid cached fixture");
    c.bench_function("review/parse-borrowed", |b| {
        b.iter(|| ClassFile::from_slice(black_box(&bytes)).expect("parse"));
    });
    c.bench_function("review/parse-owned", |b| {
        b.iter(|| ClassFile::from_bytes(black_box(&bytes)).expect("parse"));
    });
    c.bench_function("review/serialize", |b| {
        b.iter(|| {
            let mut output = Vec::new();
            class.to_bytes(black_box(&mut output)).expect("serialize");
            output
        });
    });
    c.bench_function("review/cache-hit", |b| {
        b.iter(|| {
            verify_class_cached(black_box(&class), &Context, &config, &cache).expect("cache")
        });
    });
    for (name, with_handlers) in [
        ("review/verify-code", false),
        ("review/verify-handlers", true),
    ] {
        let class = fixture(1, 128, with_handlers);
        let method = class.methods.first().expect("method");
        verify_method(&class, method, &Context, &config).expect("valid method fixture");
        c.bench_function(name, |b| {
            b.iter(|| verify_method(black_box(&class), method, &Context, &config).expect("verify"));
        });
    }
    let mut fallback = fixture(1, 128, false);
    fallback.version = JAVA_6;
    if let Some(Attribute::Code { code, .. }) = fallback
        .methods
        .first_mut()
        .and_then(|m| m.attributes.first_mut())
    {
        code.insert(0, Instruction::Goto(3));
    }
    let fallback_method = fallback.methods.first().expect("method");
    let permissive = VerifierConfig::permissive();
    let result =
        verify_method(&fallback, fallback_method, &Context, &permissive).expect("fallback fixture");
    assert_eq!(
        result.path_used,
        ristretto_classfile::verifiers::bytecode::VerificationPath::Inference
    );
    c.bench_function("review/verify-fallback", |b| {
        b.iter(|| {
            verify_method(black_box(&fallback), fallback_method, &Context, &permissive)
                .expect("fallback")
        });
    });
    let signature = "<T:Ljava/lang/Object;>(Ljava/util/Map<Ljava/lang/String;Ljava/util/List<TT;>;>;)Ljava/util/List<TT;>;";
    signature::verify_method_signature(signature).expect("valid signature");
    c.bench_function("review/signature", |b| {
        b.iter(|| signature::verify_method_signature(black_box(signature)).expect("signature"));
    });
    let source = "aβ".repeat(256);
    c.bench_function("review/string-owned", |b| {
        b.iter_batched(
            || source.clone(),
            |s| JavaString::from(black_box(s)),
            BatchSize::SmallInput,
        );
    });
    for (name, source) in [
        ("review/format-ascii", "a".repeat(256)),
        ("review/format-mutf8", "a\0😀".repeat(64)),
    ] {
        let string = JavaString::from(source);
        c.bench_function(name, |b| b.iter(|| format!("{}", black_box(&string))));
    }
}
criterion_group! { name = benches; config = Criterion::default(); targets = benchmarks }
criterion_main!(benches);
