use super::bridge::{
    BRIDGE_CLASS, BRIDGE_COMPILE_METHOD, parse_bridge_result, register_bridge_classes,
    validate_memory_arguments,
};
use super::runtime::java8_tools_jar;
use super::{CompiledClasses, CompilerError, JavaSource};
use crate::{Configuration, ConfigurationBuilder, Error, VM};
use ristretto_types::JavaObject;
use std::ffi::OsStr;
use std::fmt::{Debug, Formatter};
use std::sync::Arc;
use tokio::sync::Mutex;

const JAVAC_MAIN_CLASS: &str = "com.sun.tools.javac.Main";
const JAVAC_COMPILE_METHOD: &str = "compile([Ljava/lang/String;)I";

/// An embedded Java compiler.
pub struct Compiler {
    vm: Arc<VM>,
    compile_lock: Mutex<()>,
}

impl Compiler {
    /// Create a compiler with the supplied VM configuration.
    ///
    /// # Errors
    ///
    /// if the compiler VM or its embedded bridge classes cannot be initialized
    pub async fn new(mut configuration: Configuration) -> Result<Self, CompilerError> {
        if let Some(tools_jar) = java8_tools_jar(&configuration).await? {
            configuration.prepend_bootstrap_class_path(tools_jar);
        }
        let vm = VM::new(configuration).await?;
        register_bridge_classes(&vm).await?;
        Ok(Self {
            vm,
            compile_lock: Mutex::new(()),
        })
    }

    /// Create a compiler with the default VM configuration.
    ///
    /// # Errors
    ///
    /// if the default configuration or compiler VM cannot be initialized
    pub async fn default() -> Result<Self, CompilerError> {
        let configuration = ConfigurationBuilder::new().build()?;
        Self::new(configuration).await
    }

    /// Compile Java files using `javac` command-line arguments.
    ///
    /// Calls on one compiler are serialized and run on the caller's current Tokio runtime.
    ///
    /// # Errors
    ///
    /// returns a typed [`CompilerError`] for every nonzero `javac` status or VM failure
    pub async fn compile<S>(&self, arguments: &[S]) -> Result<(), CompilerError>
    where
        S: AsRef<OsStr> + Debug,
    {
        let _guard = self.compile_lock.lock().await;
        let thread = self.vm.primordial_thread().await?;
        let arguments = arguments.to_object(&*thread).await?;
        let result = self
            .vm
            .try_invoke(
                JAVAC_MAIN_CLASS,
                JAVAC_COMPILE_METHOD,
                std::slice::from_ref(&arguments),
            )
            .await?;
        let exit_code = result.as_i32().map_err(Error::from)?;
        CompilerError::from_exit_code(exit_code)
    }

    /// Compile one Java source from memory with default compiler options.
    ///
    /// # Errors
    ///
    /// if the source does not compile or the compiler cannot be invoked
    pub async fn compile_source<N, S>(
        &self,
        class_name: N,
        source: S,
    ) -> Result<CompiledClasses, CompilerError>
    where
        N: AsRef<str>,
        S: AsRef<str>,
    {
        self.compile_source_with_options(class_name, source, &[] as &[&str])
            .await
    }

    /// Compile one Java source from memory with compiler options.
    ///
    /// File-output options such as `-d`, `-s`, and `-h` are rejected because generated artifacts
    /// remain in memory.
    ///
    /// # Errors
    ///
    /// if the arguments are invalid, the source does not compile, or the compiler cannot be
    /// invoked
    pub async fn compile_source_with_options<N, S, O>(
        &self,
        class_name: N,
        source: S,
        options: &[O],
    ) -> Result<CompiledClasses, CompilerError>
    where
        N: AsRef<str>,
        S: AsRef<str>,
        O: AsRef<OsStr> + Debug,
    {
        let source = JavaSource::new(class_name.as_ref(), source.as_ref());
        self.compile_sources(std::slice::from_ref(&source), options)
            .await
    }

    /// Compile mutually dependent Java sources entirely in memory.
    ///
    /// File-output options such as `-d`, `-s`, and `-h` are rejected because generated artifacts
    /// remain in memory.
    ///
    /// # Errors
    ///
    /// if the arguments are invalid, a source does not compile, or the compiler cannot be invoked
    pub async fn compile_sources<O>(
        &self,
        sources: &[JavaSource],
        options: &[O],
    ) -> Result<CompiledClasses, CompilerError>
    where
        O: AsRef<OsStr> + Debug,
    {
        validate_memory_arguments(sources, options)?;
        let _guard = self.compile_lock.lock().await;

        let class_names = sources
            .iter()
            .map(|source| source.class_name.as_str())
            .collect::<Vec<_>>();
        let source_text = sources
            .iter()
            .map(|source| source.source.as_str())
            .collect::<Vec<_>>();
        let thread = self.vm.primordial_thread().await?;
        let class_names = class_names.as_slice().to_object(&*thread).await?;
        let source_text = source_text.as_slice().to_object(&*thread).await?;
        let options = options.to_object(&*thread).await?;
        let result = self
            .vm
            .try_invoke(
                BRIDGE_CLASS,
                BRIDGE_COMPILE_METHOD,
                &[class_names, source_text, options],
            )
            .await?;
        parse_bridge_result(&result)
    }
}

impl Debug for Compiler {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("Compiler").finish_non_exhaustive()
    }
}
