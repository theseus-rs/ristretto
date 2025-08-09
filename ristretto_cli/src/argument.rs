use anstyle::{AnsiColor, Style};
use clap::builder::Styles;
use clap::{ArgAction, ArgGroup, Parser};
use std::env;
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
        .args(&["mainclass", "jar"])
))]
#[clap(styles=STYLES)]
pub struct Cli {
    #[arg(help = "the main class to execute")]
    pub mainclass: Option<String>,

    #[arg(
        long = "jar",
        help = "execute a jar file",
        conflicts_with = "mainclass"
    )]
    pub jar: Option<String>,

    #[arg(
        long = "classpath",
        visible_aliases = ["cp", "class-path"],
        help = "class search path of directories and zip/jar files"
    )]
    pub classpath: Option<String>,

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

impl Cli {
    pub fn parse() -> Self {
        let args = Self::preprocess_args(env::args().collect());
        Self::parse_from(args)
    }

    /// Preprocesses the command line arguments to replace short options with long options.
    fn preprocess_args(mut arguments: Vec<String>) -> Vec<String> {
        for argument in &mut arguments {
            match argument.as_str() {
                "-help" => {
                    *argument = "-h".to_string();
                }
                "-cp" => {
                    *argument = "--cp".to_string();
                }
                "-version" => {
                    *argument = "--version".to_string();
                }
                "-Xbatch" => {
                    *argument = "--Xbatch".to_string();
                }
                "-Xcomp" => {
                    *argument = "--Xcomp".to_string();
                }
                "-Xdebug" => {
                    *argument = "--Xdebug".to_string();
                }
                "-Xint" => {
                    *argument = "--Xint".to_string();
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
}
