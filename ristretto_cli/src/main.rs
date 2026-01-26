//! Ristretto CLI entry point.

#![forbid(unsafe_code)]

mod argument;
mod logging;
mod module;
mod version;

use argument::{Arguments, VerifyMode as CliVerifyMode};
use clap::CommandFactory;
use ristretto_gc::{ConfigurationBuilder as GcConfigurationBuilder, GarbageCollector};
use ristretto_vm::Error::{InternalError, Throwable};
use ristretto_vm::{
    ClassPath, ConfigurationBuilder, Error, Result, VM, Value, VerifyMode, startup_trace,
};
use std::env;
use std::env::consts::{ARCH, OS};
use std::path::PathBuf;
use tracing::debug;

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(target_family = "wasm")]
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    startup_trace!("[cli] entry point");
    let cli = Arguments::parse();
    logging::initialize()?;
    startup_trace!("[cli] initialization");
    if common_main(cli).await.is_err() {
        std::process::exit(1);
    }
    startup_trace!("[cli] main executed");
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[tokio::main]
async fn main() -> Result<()> {
    startup_trace!("[cli] entry point");
    let cli = Arguments::parse();
    startup_trace!("[cli] argument parse");
    logging::initialize()?;
    startup_trace!("[cli] logging initialize");
    if common_main(cli).await.is_err() {
        std::process::exit(1);
    }
    startup_trace!("[cli] main executed");
    Ok(())
}

async fn common_main(mut cli: Arguments) -> Result<()> {
    if cli.help {
        Arguments::command().print_help()?;
        return Ok(());
    }

    if cli.x_options.x_help {
        argument::print_x_help();
        return Ok(());
    }

    if cli.version {
        let version = version::full();
        println!("{version}");
        return Ok(());
    }

    debug!("ristretto/{VERSION}/{OS}/{ARCH}");
    let mut configuration_builder = ConfigurationBuilder::new();
    if let Some(ref class_path) = cli.classpath {
        let class_paths = env::split_paths(class_path).collect::<Vec<_>>();
        let class_path = ClassPath::from(&class_paths);
        configuration_builder = configuration_builder.class_path(class_path);
    }

    if let Ok(java_version) = env::var("JAVA_VERSION") {
        configuration_builder = configuration_builder.java_version(java_version);
    }

    if let Some(ref main_class) = cli.mainclass {
        configuration_builder = configuration_builder.main_class(main_class);
    } else if let Some(ref jar) = cli.jar {
        configuration_builder = configuration_builder.jar(PathBuf::from(jar));
    }

    let gc_config = GcConfigurationBuilder::new().build();
    let garbage_collector = GarbageCollector::with_config(gc_config);
    configuration_builder = configuration_builder.garbage_collector(garbage_collector);

    if let Some(ref properties) = cli.properties {
        for property in properties {
            let mut parts = property.splitn(2, '=');
            let key = parts.next().ok_or(InternalError(format!(
                "Invalid system property key: {property}"
            )))?;
            let value = parts.next().ok_or(InternalError(format!(
                "Invalid system property value: {property}"
            )))?;
            configuration_builder = configuration_builder.add_system_property(key, value);
        }
    }

    configuration_builder =
        configuration_builder.batch_compilation(cli.x_options.batch_compilation);
    configuration_builder = configuration_builder.interpreted(cli.x_options.interpreted);

    let verify_mode = match cli.x_options.verify {
        CliVerifyMode::All => VerifyMode::All,
        CliVerifyMode::Remote => VerifyMode::Remote,
        CliVerifyMode::None => VerifyMode::None,
    };
    configuration_builder = configuration_builder.verify_mode(verify_mode);

    if cli.enable_preview {
        configuration_builder = configuration_builder.preview_features();
    }

    configuration_builder = module::apply_module_configuration(&mut cli, configuration_builder);

    let configuration = configuration_builder.build()?;
    startup_trace!("[cli] vm configuration");

    let vm = match VM::new(configuration).await {
        Ok(vm) => vm,
        Err(error) => {
            return process_error(error);
        }
    };
    startup_trace!("[cli] vm created");

    let parameters = cli.parameters.unwrap_or_default();
    let result = match vm.invoke_main(&parameters).await {
        Ok(_) => Ok(()),
        Err(error) => process_error(error),
    };

    // Wait for all non-daemon threads to complete before exiting
    // This matches JVM behavior where the VM waits for all non-daemon threads
    if let Err(error) = vm.wait_for_non_daemon_threads().await {
        eprintln!("Error waiting for threads: {error}");
    }

    result
}

fn process_error(error: Error) -> Result<()> {
    let Throwable(ref throwable) = error else {
        eprintln!("{error}");
        return Err(error);
    };

    let mut current_throwable = throwable.clone();
    let mut first_throwable = true;

    loop {
        let (class_name, message, stack_trace, cause) = {
            let throwable_object = current_throwable.as_object_ref()?;
            let class = throwable_object.class();
            let class_name = class.name().to_string();
            let message = throwable_object
                .value("detailMessage")
                .and_then(|value| value.as_string())?;
            let stack_trace: Vec<Value> = throwable_object.value("backtrace")?.try_into()?;
            let cause = throwable_object.value("cause")?;
            (class_name, message, stack_trace, cause)
        };

        let prelude = if first_throwable {
            first_throwable = false;
            "Exception"
        } else {
            "Caused by"
        };
        eprintln!("{prelude} {class_name}: {message}");

        for stack_trace_element in stack_trace {
            let Value::Object(Some(reference)) = stack_trace_element else {
                continue;
            };
            let guard = reference.read();
            let stack_trace_element = guard.as_object_ref()?;
            let class = stack_trace_element.value("declaringClass")?.as_string()?;
            let method = stack_trace_element.value("methodName")?.as_string()?;

            let mut source = String::new();
            let file = stack_trace_element.value("fileName")?;
            if let Value::Object(Some(ref _file_object)) = file {
                source = file.as_string()?;
            }
            let line = stack_trace_element.value("lineNumber")?.as_i32()?;
            if line > 0 {
                if source.is_empty() {
                    source = format!("{line}");
                } else {
                    source = format!("{source}:{line}");
                }
            }
            if source.is_empty() {
                eprintln!("    at {class}.{method}");
            } else {
                eprintln!("    at {class}.{method}({source})");
            }
        }

        if cause.is_null() {
            break;
        }

        let Value::Object(Some(current_gc)) = &current_throwable else {
            break;
        };
        let Value::Object(Some(cause_gc)) = &cause else {
            break;
        };
        if std::ptr::eq(cause_gc.as_ptr(), current_gc.as_ptr()) {
            break;
        }
        current_throwable = cause;
    }
    Err(error)
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[tokio::test]
    async fn test_common_main_no_parameters_error() -> Result<()> {
        let parameters: Vec<String> = Vec::new();
        let cli = Arguments::parse_from(parameters);
        let result = common_main(cli).await;
        assert!(result.is_err());
        Ok(())
    }
}
