use anstyle::{AnsiColor, Style};
use clap::builder::Styles;
use clap::{ArgAction, ArgGroup, Parser, ValueEnum};
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
    #[arg(help = "the main class to execute")]
    pub mainclass: Option<String>,

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
        value_delimiter = ':'
    )]
    pub module_path: Option<Vec<OsString>>,

    #[arg(
        long = "upgrade-module-path",
        help = "upgrade module path",
        value_delimiter = ':'
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

    #[arg(
        long = "add-exports",
        help = "add exports (SOURCE/PACKAGE=TARGET,...)",
        value_delimiter = ','
    )]
    pub add_exports: Option<Vec<String>>,

    #[arg(
        long = "add-opens",
        help = "add opens (SOURCE/PACKAGE=TARGET,...)",
        value_delimiter = ','
    )]
    pub add_opens: Option<Vec<String>>,

    #[arg(
        long = "patch-module",
        help = "patch module (MODULE=PATH,...)",
        value_delimiter = ','
    )]
    pub patch_module: Option<Vec<String>>,

    #[arg(short = 'D', help = "define a system property")]
    pub properties: Option<Vec<String>>,

    #[arg(help = "additional parameters to pass to the main class")]
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
        for argument in &mut arguments {
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
