//! Compile Java source and run the resulting in-memory class using Ristretto's Rust APIs.

#![forbid(unsafe_code)]

use ristretto_vm::{
    ClassPath, ClassPathEntry, CompiledClasses, Compiler, ConfigurationBuilder, Memory, VM, Value,
};

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

const CLASS_NAME: &str = "InMemoryHello";

#[cfg(target_family = "wasm")]
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    common_main().await
}

#[cfg(not(target_family = "wasm"))]
#[tokio::main]
async fn main() -> Result<()> {
    common_main().await
}

async fn common_main() -> Result<()> {
    let _result = compile_and_run(ConfigurationBuilder::new()).await?;
    Ok(())
}

fn source() -> &'static str {
    r#"
        public class InMemoryHello {
            public static void main(String[] arguments) {
                System.out.println("Hello from an in-memory class!");
            }
        }
    "#
}

async fn compile_classes() -> Result<CompiledClasses> {
    let compiler = Compiler::default().await?;
    Ok(compiler.compile_source(CLASS_NAME, source()).await?)
}

async fn compile_and_run(configuration_builder: ConfigurationBuilder) -> Result<Option<Value>> {
    let compiled_classes = compile_classes().await?;
    let memory = Memory::new("in-memory-compiler");
    compiled_classes.load_into(&memory).await?;
    let class_path = ClassPath::new(vec![ClassPathEntry::Memory(memory)]);
    let configuration = configuration_builder
        .main_class(CLASS_NAME)
        .class_path(class_path)
        .build()?;
    let vm = VM::new(configuration).await?;
    let parameters = Vec::<&str>::new();
    Ok(vm.invoke_main(&parameters).await?)
}

#[cfg(all(test, not(target_family = "wasm")))]
mod test {
    use super::*;
    use std::io::Cursor;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    #[tokio::test(flavor = "current_thread")]
    async fn test_compile_and_run() {
        let stdout = Arc::new(Mutex::new(Cursor::new(Vec::<u8>::new())));
        let configuration_builder = ConfigurationBuilder::new().stdout(stdout.clone());

        let result = match compile_and_run(configuration_builder).await {
            Ok(result) => result,
            Err(error) => panic!("failed to compile and run example: {error}"),
        };

        assert!(result.is_none());
        let stdout = stdout.lock().await;
        let output = String::from_utf8_lossy(stdout.get_ref());
        assert_eq!("Hello from an in-memory class!", output.trim());
    }
}
