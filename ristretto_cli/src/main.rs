#![forbid(unsafe_code)]

mod logging;
mod version;

use clap::{ArgGroup, Parser};
use ristretto_vm::Error::{InternalError, Throwable};
use ristretto_vm::{ClassPath, ConfigurationBuilder, Error, Reference, Result, VM, Value};
use std::env;
use std::env::consts::{ARCH, OS};
use std::path::PathBuf;
use tracing::debug;

#[derive(Debug, Parser)]
#[command(
    name = "java",
    about = "Ristretto CLI",
    help_expected = true,
    trailing_var_arg = true
)]
#[command(group(
    ArgGroup::new("execution")
    .args(&["mainclass", "jar"])
))]
struct Cli {
    #[arg(help = "The main class to execute")]
    mainclass: Option<String>,

    #[arg(
        long = "jar",
        help = "Execute a jar file",
        conflicts_with = "mainclass"
    )]
    jar: Option<String>,

    #[arg(
        long = "classpath",
        help = "Class search path of directories and zip/jar files"
    )]
    classpath: Option<String>,

    #[arg(short = 'D', help = "Define a system property")]
    properties: Option<Vec<String>>,

    #[arg(help = "Additional parameters to pass to the main class")]
    parameters: Option<Vec<String>>,

    #[arg(long = "int", help = "Disable JIT compilation")]
    interpreted: bool,

    #[arg(
        long = "enable-preview",
        help = "Allow classes to depend on preview features of this release"
    )]
    enable_preview: bool,

    /// Display the version of this tool
    #[arg(long)]
    version: bool,
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(target_family = "wasm")]
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    logging::initialize()?;
    let cli = Cli::parse();
    if common_main(cli).await.is_err() {
        std::process::exit(1);
    }
    Ok(())
}

#[cfg(not(target_family = "wasm"))]
#[tokio::main]
async fn main() -> Result<()> {
    logging::initialize()?;
    let cli = Cli::parse();
    if common_main(cli).await.is_err() {
        std::process::exit(1);
    }
    Ok(())
}

async fn common_main(cli: Cli) -> Result<()> {
    if cli.version {
        let version = version::full();
        println!("{version}");
        return Ok(());
    }

    debug!("ristretto/{VERSION}/{OS}/{ARCH}");
    let mut configuration_builder = ConfigurationBuilder::new();
    if let Some(class_path) = cli.classpath {
        let class_path = ClassPath::from(class_path.as_str());
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

    if cli.interpreted {
        configuration_builder = configuration_builder.interpreted();
    }

    if cli.enable_preview {
        configuration_builder = configuration_builder.preview_features();
    }

    let configuration = configuration_builder.build()?;
    let vm = match VM::new(configuration).await {
        Ok(vm) => vm,
        Err(error) => {
            return process_error(error);
        }
    };
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

    let mut throwable = throwable.clone();
    let mut first_throwable = true;

    loop {
        let class = throwable.class();
        let class_name = class.name();
        let message: String = throwable
            .value("detailMessage")
            .and_then(|value| value.try_into())?;
        let prelude = if first_throwable {
            first_throwable = false;
            "Exception"
        } else {
            "Caused by"
        };
        eprintln!("{prelude} {class_name}: {message}");
        let Value::Object(Some(Reference::Array(stack_trace_array))) =
            throwable.value("backtrace")?
        else {
            return Err(error);
        };
        let stack_trace = stack_trace_array.elements.to_vec()?;
        for stack_trace_element in stack_trace {
            let Some(Reference::Object(stack_trace_element)) = stack_trace_element else {
                continue;
            };
            let class: String = stack_trace_element.value("declaringClass")?.try_into()?;
            let method: String = stack_trace_element.value("methodName")?.try_into()?;

            let mut source = String::new();
            let file = stack_trace_element.value("fileName")?;
            if let Value::Object(Some(ref _file_object)) = file {
                source = file.try_into()?;
            };
            let line = stack_trace_element.value("lineNumber")?.to_int()?;
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

        let cause = throwable.value("cause")?;
        let Value::Object(Some(Reference::Object(cause))) = cause else {
            break;
        };
        if throwable == cause {
            break;
        } else {
            throwable = cause;
        }
    }
    Err(error)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_common_main_no_parameters_error() -> Result<()> {
        let parameters: Vec<String> = Vec::new();
        let cli = Cli::parse_from(parameters);
        let result = common_main(cli).await;
        assert!(result.is_err());
        Ok(())
    }
}
