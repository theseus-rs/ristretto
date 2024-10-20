#![forbid(unsafe_code)]

mod logging;
mod version;

use clap::{ArgGroup, Parser};
use ristretto_vm::Error::InternalError;
use ristretto_vm::{ClassPath, ConfigurationBuilder, Error, Result, VM};
use std::env;
use std::env::consts::{ARCH, OS};
use std::path::PathBuf;
use std::process::exit;
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

    #[arg(help = "Additional arguments to pass to the main class")]
    arguments: Option<Vec<String>>,

    /// Display the version of this tool
    #[arg(long)]
    version: bool,
}

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(target_arch = "wasm32")]
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    logging::initialize();
    let cli = Cli::parse();
    common_main(cli).await
}

#[cfg(not(target_arch = "wasm32"))]
#[tokio::main]
async fn main() -> Result<()> {
    logging::initialize();
    let cli = Cli::parse();
    common_main(cli).await
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

    let configuration = configuration_builder.build()?;
    let vm = match VM::new(configuration).await {
        Ok(vm) => vm,
        Err(error) => {
            process_error(error);
            exit(1);
        }
    };
    let Some(main_class_name) = vm.main_class() else {
        return Err(InternalError("No main class specified".into()));
    };
    let main_class = vm.class(main_class_name).await?;
    let Some(main_method) = main_class.main_method() else {
        return Err(InternalError("No main method found".into()));
    };

    let mut arguments = Vec::new();
    for argument in cli.arguments.unwrap_or_default() {
        arguments.push(vm.string(argument.as_str()).await?);
    }

    if let Err(error) = vm.invoke(&main_class, &main_method, arguments).await {
        process_error(error);
        exit(1);
    }
    Ok(())
}

fn process_error(error: Error) {
    let Error::Throwable(throwable) = error else {
        eprintln!("{error}");
        return;
    };
    let class = throwable.class();
    let class_name = class.name();
    let message = throwable
        .field("detailMessage")
        .and_then(|value| value.value())
        .and_then(|value| value.as_string())
        .unwrap_or_default();
    eprintln!("Exception {class_name}: {message}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_common_main_no_arguments_error() -> Result<()> {
        let arguments: Vec<String> = Vec::new();
        let cli = Cli::parse_from(arguments);
        let result = common_main(cli).await;
        assert!(result.is_err());
        Ok(())
    }
}
