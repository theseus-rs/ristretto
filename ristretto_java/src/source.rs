//! Source-file launch using the selected JDK's compiler inside Ristretto.

use crate::argument::Arguments;
use std::ffi::OsStr;
use std::path::Path;

pub fn file(cli: &Arguments) -> Option<String> {
    cli.mainclass
        .as_ref()
        .filter(|name| {
            cli.source.is_some()
                || (Path::new(name).extension() == Some(OsStr::new("java"))
                    && Path::new(name).is_file())
        })
        .cloned()
}

/// Forward compiler-relevant options before module configuration consumes them.
pub fn compiler_arguments(cli: &Arguments) -> Vec<String> {
    let mut arguments = Vec::new();
    if let Some(classpath) = &cli.classpath {
        arguments.extend([
            "--class-path".to_string(),
            classpath.to_string_lossy().into_owned(),
        ]);
    }
    for (option, values) in [
        ("--module-path", &cli.module_path),
        ("--upgrade-module-path", &cli.upgrade_module_path),
    ] {
        if let Some(values) = values {
            let separator = OsStr::new(if cfg!(windows) { ";" } else { ":" });
            arguments.extend([
                option.to_string(),
                values.join(separator).to_string_lossy().into_owned(),
            ]);
        }
    }
    for (option, values) in [
        ("--add-modules", &cli.add_modules),
        ("--limit-modules", &cli.limit_modules),
    ] {
        if let Some(values) = values {
            arguments.extend([option.to_string(), values.join(",")]);
        }
    }
    for (option, values) in [
        ("--add-exports", &cli.add_exports),
        ("--patch-module", &cli.patch_module),
    ] {
        for value in values.iter().flatten() {
            arguments.extend([option.to_string(), value.clone()]);
        }
    }
    if cli.enable_preview {
        arguments.push("--enable-preview".to_string());
    }
    arguments
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    use ristretto_vm::ConfigurationBuilder;
    use std::path::PathBuf;

    #[test]
    fn preserve_module_paths_for_compiler_and_runtime() {
        let paths = if cfg!(windows) {
            [r"C:\module files", r"D:\other modules"]
        } else {
            ["/module files", "/other modules"]
        };
        let joined = if cfg!(windows) {
            r"C:\module files;D:\other modules"
        } else {
            "/module files:/other modules"
        };
        let mut cli = Arguments::try_parse_from([
            OsStr::new("java"),
            OsStr::new("--module-path"),
            OsStr::new(joined),
            OsStr::new("--upgrade-module-path"),
            OsStr::new(joined),
            OsStr::new("Hello.java"),
        ])
        .expect("valid module options");
        assert_eq!(
            ["--module-path", joined, "--upgrade-module-path", joined],
            compiler_arguments(&cli).as_slice()
        );
        let configuration =
            crate::module::apply_module_configuration(&mut cli, ConfigurationBuilder::new())
                .build()
                .expect("valid module configuration");
        let expected = paths.map(PathBuf::from);
        assert_eq!(expected, configuration.module_path());
        assert_eq!(expected, configuration.upgrade_module_path());
    }

    #[test]
    fn preserve_module_option_values_for_compiler_and_runtime() {
        let mut cli = Arguments::try_parse_from([
            "java",
            "--add-exports",
            "java.base/jdk.internal.misc=java.compiler,ALL-UNNAMED",
            "--patch-module",
            "app=patch,files",
            "Hello.java",
        ])
        .expect("valid module options");
        assert_eq!(
            [
                "--add-exports",
                "java.base/jdk.internal.misc=java.compiler,ALL-UNNAMED",
                "--patch-module",
                "app=patch,files",
            ],
            compiler_arguments(&cli).as_slice()
        );
        let configuration =
            crate::module::apply_module_configuration(&mut cli, ConfigurationBuilder::new())
                .build()
                .expect("valid module configuration");
        assert_eq!(
            ["java.compiler", "ALL-UNNAMED"],
            configuration
                .add_exports()
                .iter()
                .map(|export| export.target.as_str())
                .collect::<Vec<_>>()
                .as_slice()
        );
        let patch = configuration.patch_modules().first().expect("module patch");
        assert_eq!("app", patch.module);
        assert_eq!(PathBuf::from("patch,files"), patch.path);
    }
}
