use anstyle::{AnsiColor, Style};
use clap::builder::Styles;
use clap::{ArgAction, ArgGroup, CommandFactory, Parser, ValueEnum};
use std::env;
use std::ffi::{OsStr, OsString};
use std::io::{self, Write};

const CYAN: Style = AnsiColor::Cyan.on_default();
const GREEN: Style = AnsiColor::Green.on_default();
const GREEN_BOLD: Style = AnsiColor::Green.on_default().bold();
const RED_BOLD: Style = AnsiColor::Red.on_default().bold();
const YELLOW: Style = AnsiColor::Yellow.on_default();
const STYLES: Styles = Styles::styled()
    .header(GREEN_BOLD)
    .usage(GREEN_BOLD)
    .literal(CYAN)
    .placeholder(CYAN)
    .error(RED_BOLD)
    .valid(GREEN)
    .invalid(YELLOW);

/// Verification mode for class files.
///
/// This corresponds to the `-Xverify` JVM options:
/// - `-Xverify:all` - Verify all classes
/// - `-Xverify:remote` - Verify only classes loaded from network (default)
/// - `-Xverify:none` - Skip verification entirely
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, ValueEnum)]
pub enum VerifyMode {
    /// Verify all classes.
    All,
    /// Verify only remote/untrusted classes (default).
    #[default]
    Remote,
    /// Skip verification entirely.
    None,
}

#[expect(clippy::struct_excessive_bools)]
#[derive(Debug, Parser)]
pub struct XOptions {
    #[arg(short = 'X', help = "print help on extra options to the error stream")]
    pub x_help: bool,

    #[arg(long = "Xbatch", hide = true, action = ArgAction::SetFalse)]
    pub batch_compilation: bool,

    #[arg(long = "Xcomp", hide = true)]
    pub compilation: bool,

    #[arg(long = "Xdebug", hide = true)]
    pub debug: bool,

    #[arg(long = "Xint", hide = true)]
    pub interpreted: bool,

    #[arg(long = "Xverify", hide = true, default_value = "remote", value_enum)]
    pub verify: VerifyMode,
}

#[derive(Debug, Parser)]
#[command(
    name = "java",
    about = "Ristretto JVM",
    help_expected = false,
    trailing_var_arg = true,
    disable_help_flag = true
)]
#[command(group(
    ArgGroup::new("execution")
        .args(&["mainclass", "jar", "module"])
))]
#[clap(styles=STYLES)]
pub struct Arguments {
    #[arg(help = "the main class or Java source file to execute")]
    pub mainclass: Option<String>,

    #[arg(
        long = "source",
        help = "launch a source file using the specified Java release",
        requires = "mainclass",
        conflicts_with_all = ["jar", "module"]
    )]
    pub source: Option<String>,

    #[arg(
        long = "jar",
        help = "execute a jar file",
        conflicts_with = "mainclass"
    )]
    pub jar: Option<OsString>,

    #[arg(
        short = 'm',
        long = "module",
        help = "execute module[/mainclass]",
        conflicts_with_all = ["mainclass", "jar"]
    )]
    pub module: Option<String>,

    #[arg(
        long = "classpath",
        visible_aliases = ["cp", "class-path"],
        help = "class search path of directories and zip/jar files"
    )]
    pub classpath: Option<OsString>,

    #[arg(
        short = 'p',
        long = "module-path",
        help = "module path for modular JARs",
        value_delimiter = if cfg!(windows) { ';' } else { ':' }
    )]
    pub module_path: Option<Vec<OsString>>,

    #[arg(
        long = "upgrade-module-path",
        help = "upgrade module path",
        value_delimiter = if cfg!(windows) { ';' } else { ':' }
    )]
    pub upgrade_module_path: Option<Vec<OsString>>,

    #[arg(
        long = "add-modules",
        help = "additional root modules (e.g., ALL-DEFAULT, ALL-SYSTEM, or module names)",
        value_delimiter = ','
    )]
    pub add_modules: Option<Vec<String>>,

    #[arg(
        long = "limit-modules",
        help = "limit observable modules",
        value_delimiter = ','
    )]
    pub limit_modules: Option<Vec<String>>,

    #[arg(
        long = "add-reads",
        help = "add read edges (SOURCE=TARGET,...)",
        value_delimiter = ','
    )]
    pub add_reads: Option<Vec<String>>,

    #[arg(long = "add-exports", help = "add exports (SOURCE/PACKAGE=TARGET,...)")]
    pub add_exports: Option<Vec<String>>,

    #[arg(
        long = "add-opens",
        help = "add opens (SOURCE/PACKAGE=TARGET,...)",
        value_delimiter = ','
    )]
    pub add_opens: Option<Vec<String>>,

    #[arg(long = "patch-module", help = "patch module (MODULE=PATH)")]
    pub patch_module: Option<Vec<String>>,

    #[arg(short = 'D', help = "define a system property")]
    pub properties: Option<Vec<String>>,

    #[arg(
        help = "additional parameters to pass to the main class",
        allow_hyphen_values = true
    )]
    pub parameters: Option<Vec<String>>,

    #[arg(
        long = "enable-preview",
        help = "allow classes to depend on preview features of this release"
    )]
    pub enable_preview: bool,

    #[arg(long = "version", help = "display the version of this tool")]
    pub version: bool,

    #[arg(short = 'h', long = "help", help = "print help information")]
    pub help: bool,

    #[command(flatten)]
    pub x_options: XOptions,
}

impl Arguments {
    pub fn parse() -> Self {
        let args = Self::preprocess_args(env::args_os().collect());
        Self::parse_from(args)
    }

    /// Preprocesses the command line arguments to replace short options with long options.
    fn preprocess_args(mut arguments: Vec<OsString>) -> Vec<OsString> {
        let command = Self::command();
        let mut option_value = false;
        let mut target_index = None;
        for (index, argument) in arguments.iter_mut().enumerate().skip(1) {
            if option_value {
                option_value = false;
                continue;
            }
            if argument == "--" {
                break;
            }
            if !argument.to_string_lossy().starts_with('-') {
                target_index = Some(index);
                break;
            }
            match argument.as_os_str() {
                s if s == OsStr::new("-help") => *argument = OsString::from("-h"),
                s if s == OsStr::new("-cp") => *argument = OsString::from("--cp"),
                s if s == OsStr::new("-version") => *argument = OsString::from("--version"),
                s if s == OsStr::new("-Xbatch") => *argument = OsString::from("--Xbatch"),
                s if s == OsStr::new("-Xcomp") => *argument = OsString::from("--Xcomp"),
                s if s == OsStr::new("-Xdebug") => *argument = OsString::from("--Xdebug"),
                s if s == OsStr::new("-Xint") => *argument = OsString::from("--Xint"),
                s if s == OsStr::new("-Xverify") || s == OsStr::new("-Xverify:remote") => {
                    *argument = OsString::from("--Xverify=remote");
                }
                s if s == OsStr::new("-Xverify:all") => *argument = OsString::from("--Xverify=all"),
                s if s == OsStr::new("-Xverify:none") => {
                    *argument = OsString::from("--Xverify=none");
                }
                _ => {}
            }
            // Everything after the launch target belongs to the Java application.
            if matches!(argument.to_str(), Some("--jar" | "-m" | "--module")) {
                break;
            }
            let Some(option) = argument.to_str() else {
                continue;
            };
            option_value = command.get_arguments().any(|arg| {
                let matches = if let Some(long) = option.strip_prefix("--") {
                    arg.get_long() == Some(long)
                        || arg
                            .get_all_aliases()
                            .is_some_and(|aliases| aliases.contains(&long))
                } else {
                    arg.get_short()
                        .is_some_and(|short| option == format!("-{short}"))
                };
                matches && arg.get_action().takes_values()
            });
        }
        if let Some(index) = target_index {
            // Stop clap from consuming recognized options belonging to the program.
            arguments.insert(index, OsString::from("--"));
        }
        arguments
    }
}

/// Prints help for the X options.
pub fn print_x_help() {
    let literal_style = STYLES.get_literal();
    let mut stderr = io::stderr();

    // Use literal style for option names to match clap's styling
    let _ = writeln!(
        stderr,
        "    {}-Xbatch{}           disable background compilation",
        literal_style.render(),
        literal_style.render_reset()
    );
    let _ = writeln!(
        stderr,
        "    {}-Xcomp{}            forces compilation of methods on first invocation",
        literal_style.render(),
        literal_style.render_reset()
    );
    let _ = writeln!(
        stderr,
        "    {}-Xdebug{}           does nothing. provided for backward compatibility.",
        literal_style.render(),
        literal_style.render_reset()
    );
    let _ = writeln!(
        stderr,
        "    {}-Xint{}             interpreted mode execution only",
        literal_style.render(),
        literal_style.render_reset()
    );
    let _ = writeln!(
        stderr,
        "    {}-Xverify{}          sets the mode of the bytecode verifier",
        literal_style.render(),
        literal_style.render_reset()
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use ristretto_classfile::JAVA_25;

    #[test]
    fn preserve_source_program_arguments() -> Result<(), clap::Error> {
        let source_version = JAVA_25.java().to_string();
        let arguments = [
            "java",
            "-cp",
            "libs",
            "-Xint",
            "--source",
            source_version.as_str(),
            "Hello.java",
            "--version",
            "-version",
            "-cp",
            "a b",
        ]
        .map(OsString::from)
        .to_vec();
        let cli = Arguments::try_parse_from(Arguments::preprocess_args(arguments))?;
        assert_eq!(Some("Hello.java"), cli.mainclass.as_deref());
        assert_eq!(Some(source_version.as_str()), cli.source.as_deref());
        assert!(cli.x_options.interpreted);
        assert_eq!(
            Some(vec![
                "--version".into(),
                "-version".into(),
                "-cp".into(),
                "a b".into()
            ]),
            cli.parameters
        );
        Ok(())
    }
}
