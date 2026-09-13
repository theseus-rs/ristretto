//! Deadline-bounded process I/O and per-child resource measurements.

use anyhow::{Context, Result, bail, ensure};
use nix::sys::signal::{Signal, killpg};
use nix::unistd::Pid;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::File;
use std::io::{Read, Seek};
use std::path::Path;
use std::process::{ExitStatus, Stdio};
use std::time::{Duration, Instant};
use tokio::io::{AsyncBufReadExt, AsyncRead, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::process::{Child, Command};
use tokio::time::timeout;

pub type Environment = BTreeMap<String, String>;

#[derive(Debug, Deserialize, Serialize)]
pub struct Sample {
    pub elapsed_ns: u64,
    pub checksum: i64,
}

#[derive(Debug, Serialize)]
pub struct Fork {
    pub samples: Vec<Sample>,
    pub stderr: String,
    pub process_wall_ns: u64,
    pub peak_rss_bytes: u64,
    pub user_cpu_s: f64,
    pub system_cpu_s: f64,
    pub resource_command: Vec<String>,
    pub resource_usage_stderr: String,
}

/// A process group guard also kills the VM below the time utility on error/cancellation.
#[derive(Debug)]
struct ManagedChild {
    child: Child,
    group: Pid,
    finished: bool,
}

impl ManagedChild {
    fn spawn(command: &mut Command) -> Result<Self> {
        command.process_group(0).kill_on_drop(true);
        let child = command.spawn()?;
        let group = Pid::from_raw(i32::try_from(child.id().context("Missing child PID")?)?);
        Ok(Self {
            child,
            group,
            finished: false,
        })
    }

    async fn wait(&mut self, limit: Duration) -> Result<ExitStatus> {
        let status = timeout(limit, self.child.wait())
            .await
            .context("VM shutdown timed out")??;
        self.finished = true;
        Ok(status)
    }

    async fn stop(&mut self) {
        let _ = killpg(self.group, Signal::SIGKILL);
        let _ = self.child.wait().await;
        self.finished = true;
    }
}

impl Drop for ManagedChild {
    fn drop(&mut self) {
        if !self.finished {
            let _ = killpg(self.group, Signal::SIGKILL);
        }
    }
}

fn command(args: &[String], env: &Environment, cwd: &Path) -> Result<Command> {
    let (program, arguments) = args.split_first().context("Empty command")?;
    let mut command = Command::new(program);
    command
        .args(arguments)
        .env_clear()
        .envs(env)
        .current_dir(cwd);
    Ok(command)
}

fn read_file(file: &mut File) -> Result<String> {
    file.rewind()?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    Ok(text)
}

pub fn elapsed_ns(start: Instant) -> Result<u64> {
    Ok(u64::try_from(start.elapsed().as_nanos())?)
}

pub async fn capture(
    args: &[String],
    env: &Environment,
    cwd: &Path,
    limit: Duration,
) -> Result<(ExitStatus, String, String)> {
    let mut stdout = tempfile::tempfile()?;
    let mut stderr = tempfile::tempfile()?;
    let mut cmd = command(args, env, cwd)?;
    cmd.stdin(Stdio::null())
        .stdout(stdout.try_clone()?)
        .stderr(stderr.try_clone()?);
    let mut child = ManagedChild::spawn(&mut cmd)?;
    match child.wait(limit).await {
        Ok(status) => Ok((status, read_file(&mut stdout)?, read_file(&mut stderr)?)),
        Err(error) => {
            child.stop().await;
            Err(error.context(format!(
                "command {args:?}; stderr: {}",
                read_file(&mut stderr)?
            )))
        }
    }
}

pub async fn output(args: &[String], env: &Environment, cwd: &Path) -> Result<String> {
    let (status, stdout, stderr) = capture(args, env, cwd, Duration::from_secs(120)).await?;
    ensure!(
        status.success(),
        "Command {args:?} failed: {stdout}{stderr}"
    );
    Ok(format!("{stdout}{stderr}").trim().to_owned())
}

async fn line<R: AsyncRead + Unpin>(reader: &mut BufReader<R>, limit: Duration) -> Result<String> {
    let mut bytes = Vec::new();
    let count = timeout(limit, reader.take(102).read_until(b'\n', &mut bytes))
        .await
        .context("VM output timed out")??;
    ensure!(count > 0, "VM closed stdout before completing the protocol");
    ensure!(
        count <= 101 && bytes.last() == Some(&b'\n'),
        "Oversized or unterminated protocol line"
    );
    bytes.pop();
    Ok(String::from_utf8(bytes)?)
}

async fn expect<R: AsyncRead + Unpin>(
    reader: &mut BufReader<R>,
    marker: &str,
    limit: Duration,
) -> Result<()> {
    let actual = line(reader, limit).await?;
    ensure!(
        actual == marker,
        "Expected protocol marker {marker:?}, got {actual:?}"
    );
    Ok(())
}

pub fn verify(samples: &[Sample], expected: Option<i64>) -> Result<i64> {
    let checksum = samples.first().context("No samples returned")?.checksum;
    ensure!(
        samples.iter().all(|s| s.checksum == checksum),
        "Checksum changed across samples"
    );
    if let Some(expected) = expected {
        ensure!(
            expected == checksum,
            "Checksum mismatch: expected {expected}, got {checksum}"
        );
    }
    Ok(checksum)
}

pub fn resource_usage(text: &str, macos: bool) -> Result<(u64, f64, f64)> {
    let (mut rss, mut user, mut system) = (None, None, None);
    for line in text.lines().map(str::trim) {
        if macos {
            if line.ends_with("maximum resident set size") {
                rss = Some(
                    line.split_whitespace()
                        .next()
                        .context("Missing RSS")?
                        .parse::<u64>()?,
                );
            } else if let Some(value) = line.strip_prefix("user ") {
                user = Some(value.parse()?);
            } else if let Some(value) = line.strip_prefix("sys ") {
                system = Some(value.parse()?);
            }
        } else if let Some(value) = line.strip_prefix("Maximum resident set size (kbytes):") {
            rss = Some(
                value
                    .trim()
                    .parse::<u64>()?
                    .checked_mul(1024)
                    .context("RSS overflow")?,
            );
        } else if let Some(value) = line.strip_prefix("User time (seconds):") {
            user = Some(value.trim().parse()?);
        } else if let Some(value) = line.strip_prefix("System time (seconds):") {
            system = Some(value.trim().parse()?);
        }
    }
    Ok((
        rss.context("Missing peak RSS from /usr/bin/time")?,
        user.context("Missing user CPU time")?,
        system.context("Missing system CPU time")?,
    ))
}

pub async fn run_fork(
    args: &[String],
    env: &Environment,
    cwd: &Path,
    limit: Duration,
    samples: usize,
) -> Result<Fork> {
    let start = Instant::now();
    let usage = tempfile::NamedTempFile::new()?;
    let mut errors = tempfile::tempfile()?;
    let mut timed = vec!["/usr/bin/time".to_owned()];
    if cfg!(target_os = "macos") {
        timed.extend(["-p".to_owned(), "-l".to_owned()]);
    } else {
        timed.push("-v".to_owned());
    }
    timed.extend(["-o".to_owned(), usage.path().to_string_lossy().into_owned()]);
    timed.extend_from_slice(args);
    let mut cmd = command(&timed, env, cwd)?;
    cmd.stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(errors.try_clone()?);
    let mut child = ManagedChild::spawn(&mut cmd)?;
    let mut input = child.child.stdin.take().context("Missing stdin")?;
    let mut reader = BufReader::new(child.child.stdout.take().context("Missing stdout")?);
    let measured = async {
        expect(&mut reader, "H", limit).await?;
        input.write_all(b"G\n").await?;
        let mut measurements = Vec::with_capacity(samples);
        for _ in 0..samples {
            expect(&mut reader, "R", limit).await?;
            let begin = Instant::now();
            timeout(limit, async {
                input.write_all(b"G\n").await?;
                expect(&mut reader, "D", limit).await
            })
            .await
            .context("VM sample timed out")??;
            let elapsed_ns = elapsed_ns(begin)?;
            let checksum = line(&mut reader, limit).await?.parse()?;
            measurements.push(Sample {
                elapsed_ns,
                checksum,
            });
        }
        drop(input);
        let status = child.wait(limit).await?;
        ensure!(status.success(), "VM exited with status {status}");
        let mut tail = Vec::new();
        timeout(limit, reader.take(1).read_to_end(&mut tail))
            .await
            .context("VM stdout did not close")??;
        ensure!(tail.is_empty(), "Unexpected trailing VM output");
        Ok::<_, anyhow::Error>(measurements)
    }
    .await;
    let measurements = match measured {
        Ok(measurements) => measurements,
        Err(error) => {
            child.stop().await;
            bail!("{error:#}; stderr: {}", read_file(&mut errors)?);
        }
    };
    let resource_text = std::fs::read_to_string(usage.path())?;
    let (peak_rss_bytes, user_cpu_s, system_cpu_s) =
        resource_usage(&resource_text, cfg!(target_os = "macos"))?;
    Ok(Fork {
        samples: measurements,
        stderr: read_file(&mut errors)?,
        process_wall_ns: elapsed_ns(start)?,
        peak_rss_bytes,
        user_cpu_s,
        system_cpu_s,
        resource_command: timed,
        resource_usage_stderr: resource_text,
    })
}

#[cfg(test)]
#[expect(
    clippy::panic_in_result_fn,
    reason = "Tests use assertions and propagate setup errors"
)]
mod tests {
    use super::*;

    fn shell(script: &str) -> Vec<String> {
        vec!["/bin/sh".into(), "-c".into(), script.into()]
    }

    #[test]
    fn rejects_invalid_checksums() {
        assert!(verify(&[], None).is_err());
        let samples = [Sample {
            elapsed_ns: 10,
            checksum: 7,
        }];
        assert!(verify(&samples, Some(8)).is_err());
        let samples = [
            Sample {
                elapsed_ns: 10,
                checksum: 7,
            },
            Sample {
                elapsed_ns: 10,
                checksum: 8,
            },
        ];
        assert!(verify(&samples, None).is_err());
    }

    #[test]
    fn parses_resource_units_on_both_platforms() -> Result<()> {
        assert_eq!(
            resource_usage(
                "user 0.25\nsys 0.1\n 2048 maximum resident set size\n",
                true
            )?,
            (2048, 0.25, 0.1)
        );
        assert_eq!(
            resource_usage(
                "User time (seconds): 0.25\nSystem time (seconds): 0.1\nMaximum resident set size (kbytes): 2\n",
                false
            )?,
            (2048, 0.25, 0.1)
        );
        assert!(resource_usage("", true).is_err());
        Ok(())
    }

    #[tokio::test]
    async fn protocol_and_resource_usage() -> Result<()> {
        let script = "printf 'H\\n'; read -r start; test \"$start\" = G || exit 1; for i in 1 2; do printf 'R\\n'; read -r start; test \"$start\" = G || exit 1; printf 'D\\n%s\\n' -1234567890123; done";
        let record = run_fork(
            &shell(script),
            &Environment::new(),
            Path::new("."),
            Duration::from_secs(5),
            2,
        )
        .await?;
        assert_eq!(verify(&record.samples, None)?, -1_234_567_890_123);
        assert_eq!(record.samples.len(), 2);
        assert!(record.peak_rss_bytes > 0);
        Ok(())
    }

    #[tokio::test]
    async fn timeout_kills_the_vm_below_the_time_wrapper() -> Result<()> {
        let pid_file = tempfile::NamedTempFile::new()?;
        let script = format!(
            "echo $$ > '{}'; exec /bin/sleep 60",
            pid_file.path().display()
        );
        let result = run_fork(
            &shell(&script),
            &Environment::new(),
            Path::new("."),
            Duration::from_millis(150),
            1,
        )
        .await;
        assert!(result.is_err());
        let pid = std::fs::read_to_string(pid_file.path())?
            .trim()
            .parse::<i32>()?;
        // A killed descendant can briefly remain a zombie until reaped by init.
        tokio::time::sleep(Duration::from_millis(100)).await;
        let output = std::process::Command::new("/bin/ps")
            .args(["-o", "stat=", "-p", &pid.to_string()])
            .output()?;
        let state = String::from_utf8_lossy(&output.stdout);
        assert!(
            state.trim().is_empty() || state.trim().starts_with('Z'),
            "Child survived: {state}"
        );
        Ok(())
    }

    #[tokio::test]
    async fn preserves_errors_and_rejects_bad_protocol() {
        for (script, message) in [
            ("echo 'example failure' >&2; exit 1", "example failure"),
            ("echo wrong", "protocol marker"),
        ] {
            let result = run_fork(
                &shell(script),
                &Environment::new(),
                Path::new("."),
                Duration::from_secs(5),
                1,
            )
            .await;
            assert!(result.is_err_and(|e| e.to_string().contains(message)));
        }
    }
}
