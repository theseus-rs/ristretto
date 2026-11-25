//! Real javac bytecode through the opt-in canonical Java tier, on native and WASI.
#![cfg(feature = "experimental-canonical-runtime")]
#![expect(clippy::panic_in_result_fn, reason = "test assertions")]

use ristretto_classfile::attributes::{Attribute, Instruction};
use ristretto_classfile::{ClassFile, Field, FieldAccessFlags, FieldType};
use ristretto_gc::canonical::Configuration;
use ristretto_vm::canonical::{Error, Limits, Program};

fn classes() -> Result<Vec<ClassFile<'static>>, Box<dyn std::error::Error>> {
    Ok(vec![
        ClassFile::from_bytes(include_bytes!(
            "fixtures/canonical/CanonicalGraphWorkload.class"
        ))?,
        ClassFile::from_bytes(include_bytes!(
            "fixtures/canonical/CanonicalGraphWorkload$Node.class"
        ))?,
    ])
}

fn configuration() -> Configuration {
    Configuration {
        objects: 4,
        roots: 12,
        mutators: 1,
        helpers: 1,
        edges_per_object: 2,
    }
}

#[tokio::test]
async fn reference_return_remains_rooted_after_unlink() -> Result<(), Box<dyn std::error::Error>> {
    let program = Program::from_classfiles([
        ClassFile::from_bytes(include_bytes!(
            "fixtures/canonical/CanonicalReturnWorkload.class"
        ))?,
        ClassFile::from_bytes(include_bytes!(
            "fixtures/canonical/CanonicalReturnWorkload$Node.class"
        ))?,
    ])?;
    for value in [7, -13] {
        let report = program
            .run(
                "CanonicalReturnWorkload",
                "run",
                value,
                configuration(),
                Limits::default(),
            )
            .await?;
        assert_eq!(report.result, value);
        assert!(report.instructions > 0);
        assert_eq!(report.allocations, 1);
        assert_eq!(report.released_objects, 1);
        assert_eq!(report.payload_bytes_after_cleanup, 0);
        assert_eq!(report.catalog_bytes_after_drop, 0);
    }
    Ok(())
}

#[tokio::test]
async fn javac_cycles_calls_identity_and_complete_cleanup() -> Result<(), Box<dyn std::error::Error>>
{
    let program = Program::from_classfiles(classes()?)?;
    for rounds in [0, 1, 3, 8] {
        let report = program
            .run(
                "CanonicalGraphWorkload",
                "run",
                rounds,
                configuration(),
                Limits::default(),
            )
            .await?;
        assert_eq!(report.result, rounds * rounds);
        let rounds = usize::try_from(rounds)?;
        assert_eq!(report.allocations, 2 * rounds);
        assert_eq!(report.reference_reads, 3 * rounds);
        assert_eq!(report.reference_writes, 4 * rounds);
        assert_eq!(report.calls, 1 + 3 * rounds);
        assert_eq!(report.payload_bytes_after_cleanup, 0);
        assert_eq!(report.catalog_bytes_after_drop, 0);
    }
    Ok(())
}

#[tokio::test]
async fn reference_capacity_rejects_before_execution() -> Result<(), Box<dyn std::error::Error>> {
    let program = Program::from_classfiles(classes()?)?;
    let mut config = configuration();
    config.edges_per_object = 1;
    assert!(matches!(
        program
            .run(
                "CanonicalGraphWorkload",
                "run",
                1,
                config,
                Limits::default()
            )
            .await,
        Err(Error::Unsupported(_))
    ));
    Ok(())
}

#[tokio::test]
async fn full_root_quota_reports_request_limit_and_cleans_up()
-> Result<(), Box<dyn std::error::Error>> {
    let program = Program::from_classfiles(classes()?)?;
    let mut config = configuration();
    config.roots = 2;
    let limits = Limits {
        request_polls: 2_000,
        ..Limits::default()
    };
    assert!(matches!(
        program
            .run("CanonicalGraphWorkload", "run", 1, config, limits)
            .await,
        Err(Error::Limit("request poll"))
    ));
    Ok(())
}

#[tokio::test]
async fn instruction_limit_drops_frame_roots_and_cleans_up()
-> Result<(), Box<dyn std::error::Error>> {
    let program = Program::from_classfiles(classes()?)?;
    let limits = Limits {
        instructions: 40,
        ..Limits::default()
    };
    assert!(matches!(
        program
            .run("CanonicalGraphWorkload", "run", 3, configuration(), limits)
            .await,
        Err(Error::Limit("instruction"))
    ));
    Ok(())
}

#[tokio::test]
async fn frame_limit_drops_call_arguments_and_cleans_up() -> Result<(), Box<dyn std::error::Error>>
{
    let program = Program::from_classfiles(classes()?)?;
    let limits = Limits {
        frames: 1,
        ..Limits::default()
    };
    assert!(matches!(
        program
            .run("CanonicalGraphWorkload", "run", 1, configuration(), limits)
            .await,
        Err(Error::Limit("frame"))
    ));
    Ok(())
}

#[test]
fn missing_referenced_class_rejects() -> Result<(), Box<dyn std::error::Error>> {
    let mut files = classes()?;
    files.pop();
    assert!(matches!(
        Program::from_classfiles(files),
        Err(Error::Unsupported(_))
    ));
    Ok(())
}

#[test]
fn duplicate_classes_reject() -> Result<(), Box<dyn std::error::Error>> {
    let mut files = classes()?;
    files.push(files.first().ok_or("missing class")?.clone());
    assert!(matches!(
        Program::from_classfiles(files),
        Err(Error::Invalid(_))
    ));
    Ok(())
}

fn add_field(
    file: &mut ClassFile<'static>,
    name: &str,
    descriptor: &str,
    flags: FieldAccessFlags,
) -> Result<(), Box<dyn std::error::Error>> {
    let name_index = file.constant_pool.add_utf8(name)?;
    let descriptor_index = file.constant_pool.add_utf8(descriptor)?;
    file.fields.push(Field {
        name_index,
        descriptor_index,
        access_flags: flags,
        field_type: FieldType::parse(descriptor)?,
        attributes: Vec::new(),
    });
    Ok(())
}

#[test]
fn ninth_reference_field_rejects() -> Result<(), Box<dyn std::error::Error>> {
    let mut files = classes()?;
    let node = files.last_mut().ok_or("missing node")?;
    for index in 2..9 {
        add_field(
            node,
            &format!("edge{index}"),
            "LCanonicalGraphWorkload$Node;",
            FieldAccessFlags::empty(),
        )?;
    }
    assert!(matches!(
        Program::from_classfiles(files),
        Err(Error::Unsupported(_))
    ));
    Ok(())
}

#[test]
fn array_payload_rejects() -> Result<(), Box<dyn std::error::Error>> {
    let mut files = classes()?;
    add_field(
        files.last_mut().ok_or("missing node")?,
        "array",
        "[I",
        FieldAccessFlags::empty(),
    )?;
    assert!(matches!(
        Program::from_classfiles(files),
        Err(Error::Unsupported(_))
    ));
    Ok(())
}

#[test]
fn volatile_static_storage_rejects() -> Result<(), Box<dyn std::error::Error>> {
    let mut files = classes()?;
    add_field(
        files.last_mut().ok_or("missing node")?,
        "global",
        "I",
        FieldAccessFlags::STATIC | FieldAccessFlags::VOLATILE,
    )?;
    assert!(matches!(
        Program::from_classfiles(files),
        Err(Error::Unsupported(_))
    ));
    Ok(())
}

fn replace_first_run_instruction(
    files: &mut [ClassFile<'static>],
    instruction: Instruction,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = files.first_mut().ok_or("missing workload")?;
    let method = file
        .methods
        .iter_mut()
        .find(|method| {
            file.constant_pool
                .try_get_utf8(method.name_index)
                .is_ok_and(|name| name == "run")
        })
        .ok_or("missing run")?;
    let code = method
        .attributes
        .iter_mut()
        .find_map(|attribute| {
            if let Attribute::Code { code, .. } = attribute {
                Some(code)
            } else {
                None
            }
        })
        .ok_or("missing code")?;
    *code.first_mut().ok_or("empty code")? = instruction;
    Ok(())
}

#[test]
fn unsupported_opcode_rejects_before_execution() -> Result<(), Box<dyn std::error::Error>> {
    let mut files = classes()?;
    replace_first_run_instruction(&mut files, Instruction::Fconst_0)?;
    assert!(matches!(
        Program::from_classfiles(files),
        Err(Error::Unsupported(_))
    ));
    Ok(())
}

#[test]
fn invalid_stack_type_rejects_during_verification() -> Result<(), Box<dyn std::error::Error>> {
    let mut files = classes()?;
    replace_first_run_instruction(&mut files, Instruction::Aconst_null)?;
    assert!(matches!(
        Program::from_classfiles(files),
        Err(Error::Invalid(_))
    ));
    Ok(())
}
