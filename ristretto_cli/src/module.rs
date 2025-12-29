//! Module configuration parsing for JPMS CLI arguments.
//!
//! This module contains parsing logic for converting CLI string arguments
//! into module configuration settings applied to `ConfigurationBuilder`.

use crate::argument::Arguments;
use ristretto_vm::{
    ConfigurationBuilder, MainModule, ModuleExport, ModuleOpens, ModulePatch, ModuleRead,
};
use std::collections::HashSet;
use std::path::PathBuf;

/// Applies module configuration from CLI arguments to a `ConfigurationBuilder`.
pub fn apply_module_configuration(
    cli: &mut Arguments,
    mut builder: ConfigurationBuilder,
) -> ConfigurationBuilder {
    if let Some(paths) = cli.module_path.take() {
        let module_path = paths.into_iter().map(PathBuf::from).collect();
        builder = builder.module_path(module_path);
    }

    if let Some(paths) = cli.upgrade_module_path.take() {
        let upgrade_module_path = paths.into_iter().map(PathBuf::from).collect();
        builder = builder.upgrade_module_path(upgrade_module_path);
    }

    if let Some(ref module_spec) = cli.module {
        let main_module = MainModule::parse(module_spec);
        builder = builder.main_module(main_module);
    }

    if let Some(modules) = cli.add_modules.take() {
        builder = builder.set_add_modules(modules);
    }

    if let Some(modules) = cli.limit_modules.take() {
        let limit_modules: HashSet<String> = modules.into_iter().collect();
        builder = builder.set_limit_modules(limit_modules);
    }

    if let Some(read_arguments) = cli.add_reads.take() {
        for read_argument in read_arguments {
            if let Some(read) = parse_read(&read_argument) {
                builder = builder.add_read(read);
            }
        }
    }

    if let Some(export_arguments) = cli.add_exports.take() {
        for export_argument in export_arguments {
            if let Some(export) = parse_export(&export_argument) {
                builder = builder.add_export(export);
            }
        }
    }

    if let Some(opens_arguments) = cli.add_opens.take() {
        for opens_argument in opens_arguments {
            if let Some(opens) = parse_opens(&opens_argument) {
                builder = builder.add_opens(opens);
            }
        }
    }

    if let Some(patch_arguments) = cli.patch_module.take() {
        for patch_argument in patch_arguments {
            if let Some(patch) = parse_patch(&patch_argument) {
                builder = builder.add_patch(patch);
            }
        }
    }

    builder
}

/// Parses a `SOURCE=TARGET` specification into a `ModuleRead`.
///
/// Returns `None` if the format is invalid.
#[must_use]
fn parse_read(spec: &str) -> Option<ModuleRead> {
    let parts: Vec<&str> = spec.splitn(2, '=').collect();
    if parts.len() == 2 {
        Some(ModuleRead::new(parts[0], parts[1]))
    } else {
        None
    }
}

/// Parses a `SOURCE/PACKAGE=TARGET` specification into a `ModuleExport`.
///
/// Returns `None` if the format is invalid.
#[must_use]
fn parse_export(spec: &str) -> Option<ModuleExport> {
    let parts: Vec<&str> = spec.splitn(2, '=').collect();
    if parts.len() != 2 {
        return None;
    }
    let target = parts[1];
    let source_parts: Vec<&str> = parts[0].splitn(2, '/').collect();
    if source_parts.len() != 2 {
        return None;
    }
    Some(ModuleExport::new(source_parts[0], source_parts[1], target))
}

/// Parses a `SOURCE/PACKAGE=TARGET` specification into a `ModuleOpens`.
///
/// Returns `None` if the format is invalid.
#[must_use]
fn parse_opens(spec: &str) -> Option<ModuleOpens> {
    let parts: Vec<&str> = spec.splitn(2, '=').collect();
    if parts.len() != 2 {
        return None;
    }
    let target = parts[1];
    let source_parts: Vec<&str> = parts[0].splitn(2, '/').collect();
    if source_parts.len() != 2 {
        return None;
    }
    Some(ModuleOpens::new(source_parts[0], source_parts[1], target))
}

/// Parses a `MODULE=PATH` specification into a `ModulePatch`.
///
/// Returns `None` if the format is invalid.
#[must_use]
fn parse_patch(spec: &str) -> Option<ModulePatch> {
    let parts: Vec<&str> = spec.splitn(2, '=').collect();
    if parts.len() == 2 {
        Some(ModulePatch::new(parts[0], parts[1]))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn test_parse_read_valid() {
        let result = parse_read("my.module=java.sql");
        assert!(result.is_some());
        let read = result.unwrap();
        assert_eq!("my.module", read.source);
        assert_eq!("java.sql", read.target);
    }

    #[test]
    fn test_parse_read_invalid() {
        assert!(parse_read("invalid").is_none());
    }

    #[test]
    fn test_parse_export_valid() {
        let result = parse_export("java.base/java.lang=ALL-UNNAMED");
        assert!(result.is_some());
        let export = result.unwrap();
        assert_eq!("java.base", export.source);
        assert_eq!("java.lang", export.package);
        assert_eq!("ALL-UNNAMED", export.target);
    }

    #[test]
    fn test_parse_export_missing_equals() {
        assert!(parse_export("java.base/java.lang").is_none());
    }

    #[test]
    fn test_parse_export_missing_slash() {
        assert!(parse_export("java.base=ALL-UNNAMED").is_none());
    }

    #[test]
    fn test_parse_opens_valid() {
        let result = parse_opens("java.base/java.lang.reflect=my.module");
        assert!(result.is_some());
        let opens = result.unwrap();
        assert_eq!("java.base", opens.source);
        assert_eq!("java.lang.reflect", opens.package);
        assert_eq!("my.module", opens.target);
    }

    #[test]
    fn test_parse_patch_valid() {
        let result = parse_patch("java.base=/path/to/patch");
        assert!(result.is_some());
        let patch = result.unwrap();
        assert_eq!("java.base", patch.module);
        assert_eq!(PathBuf::from("/path/to/patch"), patch.path);
    }

    #[test]
    fn test_parse_patch_invalid() {
        assert!(parse_patch("invalid").is_none());
    }

    #[test]
    fn test_build_module_configuration_full() {
        let args = vec![
            "java",
            "--module-path",
            "/mods",
            "--upgrade-module-path",
            "/upgrade",
            "--module",
            "my.module/com.example.Main",
            "--add-modules",
            "java.sql",
            "--limit-modules",
            "java.base",
            "--add-reads",
            "my.module=java.sql",
            "--add-exports",
            "java.base/java.lang=ALL-UNNAMED",
            "--add-opens",
            "java.base/java.lang.reflect=ALL-UNNAMED",
            "--patch-module",
            "java.base=/patch",
        ];
        let mut cli = Arguments::parse_from(args);
        let builder = ConfigurationBuilder::new();
        let builder = apply_module_configuration(&mut cli, builder);
        let config = builder.build().unwrap();

        assert_eq!(1, config.module_path().len());
        assert_eq!(PathBuf::from("/mods"), config.module_path()[0]);

        assert_eq!(1, config.upgrade_module_path().len());
        assert_eq!(PathBuf::from("/upgrade"), config.upgrade_module_path()[0]);

        assert!(config.main_module().is_some());
        assert_eq!("my.module", config.main_module_name().unwrap());
        assert_eq!("com.example.Main", config.main_module_class().unwrap());

        assert_eq!(1, config.add_modules().len());
        assert_eq!("java.sql", config.add_modules()[0]);

        assert_eq!(1, config.limit_modules().len());
        assert!(config.limit_modules().contains("java.base"));

        assert_eq!(1, config.add_reads().len());
        let read = &config.add_reads()[0];
        assert_eq!("my.module", read.source);
        assert_eq!("java.sql", read.target);

        assert_eq!(1, config.add_exports().len());
        let export = &config.add_exports()[0];
        assert_eq!("java.base", export.source);
        assert_eq!("java.lang", export.package);
        assert_eq!("ALL-UNNAMED", export.target);

        assert_eq!(1, config.add_opens().len());
        let opens = &config.add_opens()[0];
        assert_eq!("java.base", opens.source);
        assert_eq!("java.lang.reflect", opens.package);
        assert_eq!("ALL-UNNAMED", opens.target);

        assert_eq!(1, config.patch_modules().len());
        let patch = &config.patch_modules()[0];
        assert_eq!("java.base", patch.module);
        assert_eq!(PathBuf::from("/patch"), patch.path);
    }

    #[test]
    fn test_build_module_configuration_filters_invalid_entries() {
        let args = vec![
            "java",
            "--add-reads",
            "valid=read,invalid",
            "--add-exports",
            "valid/pkg=target,invalid,also/invalid",
            "--patch-module",
            "valid=/path,invalid",
        ];
        let mut cli = Arguments::parse_from(args);
        let builder = ConfigurationBuilder::new();
        let builder = apply_module_configuration(&mut cli, builder);
        let config = builder.build().unwrap();

        assert_eq!(1, config.add_reads().len());
        let read = &config.add_reads()[0];
        assert_eq!("valid", read.source);
        assert_eq!("read", read.target);

        assert_eq!(1, config.add_exports().len());
        let export = &config.add_exports()[0];
        assert_eq!("valid", export.source);
        assert_eq!("pkg", export.package);
        assert_eq!("target", export.target);

        assert_eq!(1, config.patch_modules().len());
        let patch = &config.patch_modules()[0];
        assert_eq!("valid", patch.module);
        assert_eq!(PathBuf::from("/path"), patch.path);
    }
}
