//! Execute source programs with the selected JDK's in-memory source launcher.

use super::run_local;
use crate::{Error, Result, VM};
use ristretto_classfile::{JAVA_11, JAVA_22};
use ristretto_types::JavaObject;
use std::ffi::OsStr;
use std::path::Path;

impl VM {
    /// Compile a Java source file in memory and invoke its entry point.
    ///
    /// Uses the selected runtime's source launcher (Java 11+). Compact source
    /// files and instance entry points are supported without preview flags on
    /// Java 25+. `compiler_arguments` contains launcher options such as
    /// `--class-path` and `--enable-preview`, not application arguments.
    ///
    /// # Errors
    ///
    /// Returns an error for unsupported runtimes, compilation or launch failures,
    /// or an exception thrown by the application.
    pub async fn invoke_source<S, O>(
        &self,
        file: &Path,
        parameters: &[S],
        compiler_arguments: &[O],
    ) -> Result<()>
    where
        S: AsRef<OsStr>,
        O: AsRef<OsStr>,
    {
        run_local(async {
            if self.java_major_version() < JAVA_11.java() {
                return Err(Error::InternalError(format!(
                    "Source-file launch requires Java {} or later", JAVA_11.java(),
                )));
            }
            let (class, method) = if self.java_major_version() >= JAVA_22.java() {
                (
                    "com.sun.tools.javac.launcher.SourceLauncher",
                    "run([Ljava/lang/String;[Ljava/lang/String;)Lcom/sun/tools/javac/launcher/Result;",
                )
            } else {
                (
                    "com.sun.tools.javac.launcher.Main",
                    "run([Ljava/lang/String;[Ljava/lang/String;)V",
                )
            };
            let stderr = self.class("java.lang.System").await?.static_value("err")?;
            let launcher = self.object(class, "Ljava/io/PrintStream;", &[stderr]).await?;
            let thread = self.primordial_thread().await?;
            let runtime_arguments = compiler_arguments.to_object(&*thread).await?;
            let arguments = std::iter::once(file.as_os_str())
                .chain(parameters.iter().map(AsRef::as_ref))
                .collect::<Vec<_>>();
            let arguments = arguments.as_slice().to_object(&*thread).await?;

            // Calling run directly supplies the actual CLI options without depending on
            // HotSpot's VM.getRuntimeArguments native or letting the launcher exit the process.
            match self
                .invoke(class, method, &[launcher, runtime_arguments, arguments])
                .await
            {
                Ok(_) => Ok(()),
                Err(Error::Throwable(throwable)) => {
                    let object = throwable.as_object_ref()?;
                    match object.class().name() {
                        "com/sun/tools/javac/launcher/Fault"
                        | "com/sun/tools/javac/launcher/Main$Fault" => Err(Error::InternalError(
                            object.value("detailMessage")?.as_string()?,
                        )),
                        "java/lang/reflect/InvocationTargetException" => {
                            Err(Error::Throwable(object.value("target")?))
                        }
                        _ => {
                            drop(object);
                            Err(Error::Throwable(throwable))
                        }
                    }
                }
                Err(error) => Err(error),
            }
        }).await
    }
}
