#![allow(dead_code)]
#![deny(clippy::pedantic)]
#![deny(clippy::unwrap_in_result)]
#![deny(clippy::unwrap_used)]

mod argument;
mod logging;
mod version;

use argument::Arguments;
use clap::CommandFactory;
use ristretto_vm::Error::{InternalError, Throwable};
use ristretto_vm::{ClassPath, ConfigurationBuilder, Error, Result, VM, Value, startup_trace};
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

async fn common_main(cli: Arguments) -> Result<()> {
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
    if let Some(class_path) = cli.classpath {
        let class_paths = env::split_paths(&class_path).collect::<Vec<_>>();
        let class_path = ClassPath::from(&class_paths);
        configuration_builder = configuration_builder.class_path(class_path);
    }

    if let Ok(java_version) = env::var("JAVA_VERSION") {
        configuration_builder = configuration_builder.java_version(java_version);
    }

    if let Some(main_class) = cli.mainclass {
        configuration_builder = configuration_builder.main_class(main_class);
    } else if let Some(jar) = cli.jar {
        configuration_builder = configuration_builder.jar(PathBuf::from(jar));
    }

    if let Some(properties) = cli.properties {
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

    if cli.enable_preview {
        configuration_builder = configuration_builder.preview_features();
    }

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
    match vm.invoke_main(&parameters).await {
        Ok(_) => Ok(()),
        Err(error) => process_error(error),
    }
}

fn process_error(error: Error) -> Result<()> {
    let Throwable(ref throwable) = error else {
        eprintln!("{error}");
        return Err(error);
    };

    let throwable = throwable.clone();
    let mut first_throwable = true;

    loop {
        let throwable_object = throwable.as_object_ref()?;
        let class = throwable_object.class();
        let class_name = class.name();
        let message = throwable_object
            .value("detailMessage")
            .and_then(|value| value.as_string())?;
        let prelude = if first_throwable {
            first_throwable = false;
            "Exception"
        } else {
            "Caused by"
        };
        eprintln!("{prelude} {class_name}: {message}");
        let stack_trace: Vec<Value> = throwable_object.value("backtrace")?.try_into()?;
        for stack_trace_element in stack_trace {
            let Value::Object(Some(reference)) = stack_trace_element else {
                continue;
            };
            let stack_trace_element = reference.as_object_ref()?;
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

        let cause = throwable_object.value("cause")?;
        if cause.is_null() || throwable == cause {
            break;
        }
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
