//! Host-side interpreter benchmarks, reports and charts.

#[cfg(any(target_os = "macos", target_os = "linux"))]
mod chart;
#[cfg(any(target_os = "macos", target_os = "linux"))]
mod process;
#[cfg(any(target_os = "macos", target_os = "linux"))]
mod report;
#[cfg(any(target_os = "macos", target_os = "linux"))]
mod runner;
#[cfg(any(target_os = "macos", target_os = "linux"))]
mod statistics;
#[cfg(all(test, any(target_os = "macos", target_os = "linux")))]
mod tests;
#[cfg(any(target_os = "macos", target_os = "linux"))]
mod workloads;

#[cfg(any(target_os = "macos", target_os = "linux"))]
use anyhow::Result;
#[cfg(any(target_os = "macos", target_os = "linux"))]
use clap::{Parser, Subcommand};
#[cfg(any(target_os = "macos", target_os = "linux"))]
use std::path::PathBuf;

#[cfg(any(target_os = "macos", target_os = "linux"))]
#[derive(Debug, Parser)]
#[command(about = "Compare identical Java workloads on Ristretto and OpenJDK with -Xint")]
struct Cli {
    #[command(subcommand)]
    command: Action,
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
#[derive(Debug, Subcommand)]
enum Action {
    /// Build, validate, calibrate and measure both interpreters.
    Run(runner::Options),
    /// Generate SVG and PNG charts from an existing summary.json.
    Plot {
        results: PathBuf,
        /// Write charts elsewhere, preserving the original artifacts.
        #[arg(long)]
        output: Option<PathBuf>,
    },
    /// Recompute a report from saved metadata.json and raw.jsonl.
    Report {
        results: PathBuf,
        /// Must be empty; prevents overwriting historical reports.
        #[arg(long)]
        output: PathBuf,
    },
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    Cli::parse().execute().await
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
impl Cli {
    async fn execute(self) -> Result<()> {
        match self.command {
            Action::Run(options) => runner::run(options).await,
            Action::Plot { results, output } => {
                chart::plot(&results, output.as_deref().unwrap_or(&results))
            }
            Action::Report { results, output } => report::replay(&results, &output),
        }
    }
}

#[cfg(not(any(target_os = "macos", target_os = "linux")))]
fn main() -> std::process::ExitCode {
    eprintln!("This benchmark tool supports macOS and Linux");
    std::process::ExitCode::FAILURE
}
