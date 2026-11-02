//! Bounded subprocess execution with output capture that cannot block on full pipes.

use ristretto_vm::Error::InternalError;
use ristretto_vm::Result;
use serde::Serialize;
use std::io::{Read, Seek, SeekFrom};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

#[derive(Debug, Serialize)]
pub struct Output {
    pub command: Vec<String>,
    pub exit_code: Option<i32>,
    pub timed_out: bool,
    pub stdout: String,
    pub stderr: String,
    pub seconds: f64,
}

impl Output {
    pub fn success(&self) -> bool {
        !self.timed_out && self.exit_code == Some(0)
    }

    pub fn require_success(&self) -> Result<()> {
        if self.success() {
            return Ok(());
        }
        Err(InternalError(format!(
            "Command {:?} failed (exit {:?}, timed out: {})\n{}{}",
            self.command, self.exit_code, self.timed_out, self.stdout, self.stderr
        )))
    }
}

/// Always kill and reap the child, including when polling or reading fails.
struct ReapOnDrop(Child);

impl Drop for ReapOnDrop {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

pub fn run(command: &mut Command, timeout: Duration) -> Result<Output> {
    let arguments = std::iter::once(command.get_program())
        .chain(command.get_args())
        .map(|argument| argument.to_string_lossy().into_owned())
        .collect();
    // Files avoid both pipe-buffer deadlocks and unbounded reader-thread joins. The fixtures
    // do not spawn descendants; kill + wait contains every timed-out invocation.
    let mut stdout = tempfile::tempfile()?;
    let mut stderr = tempfile::tempfile()?;
    command
        .stdin(Stdio::null())
        .stdout(Stdio::from(stdout.try_clone()?))
        .stderr(Stdio::from(stderr.try_clone()?));
    let start = Instant::now();
    let mut child = ReapOnDrop(command.spawn()?);
    let (status, timed_out) = loop {
        if let Some(status) = child.0.try_wait()? {
            break (status, false);
        }
        if start.elapsed() >= timeout {
            child.0.kill()?;
            break (child.0.wait()?, true);
        }
        std::thread::sleep(Duration::from_millis(10));
    };
    let seconds = start.elapsed().as_secs_f64();
    stdout.seek(SeekFrom::Start(0))?;
    stderr.seek(SeekFrom::Start(0))?;
    let mut stdout_bytes = Vec::new();
    let mut stderr_bytes = Vec::new();
    stdout.read_to_end(&mut stdout_bytes)?;
    stderr.read_to_end(&mut stderr_bytes)?;
    Ok(Output {
        command: arguments,
        exit_code: status.code(),
        timed_out,
        stdout: String::from_utf8_lossy(&stdout_bytes).into_owned(),
        stderr: String::from_utf8_lossy(&stderr_bytes).into_owned(),
        seconds,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    const CHILD_MODE: &str = "RISTRETTO_HARNESS_CHILD_MODE";

    #[test]
    #[ignore = "subprocess fixture for process helper tests"]
    fn child_process() -> std::io::Result<()> {
        match std::env::var(CHILD_MODE).as_deref() {
            Ok("sleep") => {
                println!("started");
                std::io::stdout().flush()?;
                std::thread::sleep(Duration::from_secs(30));
            }
            Ok("output") => {
                let bytes = vec![b'x'; 256 * 1024];
                std::io::stdout().write_all(&bytes)?;
                std::io::stderr().write_all(&bytes)?;
                std::process::exit(7);
            }
            _ => {}
        }
        Ok(())
    }

    fn child(mode: &str) -> Result<Command> {
        let mut command = Command::new(std::env::current_exe()?);
        command
            .args([
                "--exact",
                "common::process::tests::child_process",
                "--ignored",
                "--nocapture",
            ])
            .env(CHILD_MODE, mode);
        Ok(command)
    }

    #[test]
    fn timeout_kills_and_reaps_child() -> Result<()> {
        let output = run(&mut child("sleep")?, Duration::from_millis(200))?;
        assert!(output.timed_out);
        assert!(!output.success());
        assert!(output.seconds < 10.0);
        assert!(output.require_success().is_err());
        Ok(())
    }

    #[test]
    fn captures_both_streams_and_rejects_failed_exit() -> Result<()> {
        let output = run(&mut child("output")?, Duration::from_secs(10))?;
        assert!(!output.timed_out);
        assert_eq!(output.exit_code, Some(7));
        assert!(output.stdout.len() >= 256 * 1024);
        assert!(output.stderr.len() >= 256 * 1024);
        assert!(output.require_success().is_err());
        Ok(())
    }
}
