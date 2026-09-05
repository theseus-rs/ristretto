#![cfg(not(target_family = "wasm"))]
#![expect(
    clippy::panic_in_result_fn,
    reason = "integration tests assert observable process behavior"
)]

use serde_json::{Value, json};
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process::Command;

struct Response {
    code: Option<i32>,
    events: Vec<Value>,
}

fn invoke(workspace: &Path, java_home: &Path) -> Result<Response, Box<dyn Error>> {
    let output = Command::new(env!("CARGO_BIN_EXE_ristretto_playground"))
        .env("RISTRETTO_PLAYGROUND_JAVA_HOME", java_home)
        .env("RISTRETTO_PLAYGROUND_WORKSPACE", workspace)
        .output()?;
    assert!(
        output.stderr.is_empty(),
        "runner wrote outside the JSON protocol: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let events = String::from_utf8(output.stdout)?
        .lines()
        .map(serde_json::from_str)
        .collect::<Result<Vec<Value>, _>>()?;
    assert!(!events.is_empty(), "runner emitted no events");
    Ok(Response {
        code: output.status.code(),
        events,
    })
}

fn output(events: &[Value], stream: &str) -> Result<String, Box<dyn Error>> {
    let mut bytes = Vec::new();
    for event in events {
        if event["type"] == "output" && event["stream"] == stream {
            bytes.extend(serde_json::from_value::<Vec<u8>>(event["bytes"].clone())?);
        }
    }
    Ok(String::from_utf8(bytes)?)
}

fn request(
    workspace: &Path,
    java_home: &Path,
    version: u16,
    action: &str,
    source: &str,
) -> Result<Response, Box<dyn Error>> {
    fs::write(
        workspace.join("request.json"),
        serde_json::to_vec(&json!({
            "id": 42,
            "action": action,
            "javaVersion": version,
            "className": "example.Main",
            "source": source,
        }))?,
    )?;
    invoke(workspace, java_home)
}

fn request_errors(workspace: &Path, java_home: &Path) -> Result<(), Box<dyn Error>> {
    // Missing and malformed requests must still produce a usable protocol error.
    for malformed in [None, Some("{")] {
        if let Some(text) = malformed {
            fs::write(workspace.join("request.json"), text)?;
        }
        let response = invoke(workspace, java_home)?;
        assert_eq!(Some(0), response.code);
        assert_eq!(1, response.events.len());
        let event = response.events.first().ok_or("missing request error")?;
        assert_eq!(0, event["id"]);
        assert_eq!("error", event["type"]);
    }

    let response = request(workspace, java_home, 9, "compile", "")?;
    assert_eq!(1, response.events.len());
    let event = response.events.first().ok_or("missing version error")?;
    assert_eq!(42, event["id"]);
    assert_eq!("Unsupported Java version", event["message"]);
    Ok(())
}

#[tokio::test(flavor = "current_thread")]
async fn runner_protocol_covers_requests_compilation_execution_and_limits()
-> Result<(), Box<dyn Error>> {
    let (java_home, _, _) = ristretto_classloader::runtime::default_class_loader().await?;
    let directory = tempfile::tempdir()?;
    let workspace = directory.path();
    request_errors(workspace, &java_home)?;

    let source = r#"package example;
public class Main {
    static class Nested {}
    public static void main(String[] args) throws Exception {
        System.out.print("☕😀 args=" + args.length + " stdin=" + System.in.read());
        System.err.print("diagnostic");
        System.out.flush();
    }
}"#;
    for action in ["compile", "run"] {
        let response = request(workspace, &java_home, 25, action, source)?;
        assert_eq!(Some(0), response.code);
        assert!(response.events.iter().all(|event| event["id"] == 42));
        assert_eq!(
            "compiling",
            response.events.first().ok_or("missing compilation phase")?["phase"]
        );
        assert!(
            response
                .events
                .iter()
                .any(|event| { event["type"] == "compiled" && event["classes"] == 2 })
        );
        assert_eq!(
            "done",
            response.events.last().ok_or("missing result")?["type"]
        );
        if action == "run" {
            assert!(
                response
                    .events
                    .iter()
                    .any(|event| event["phase"] == "running")
            );
            assert_eq!("☕😀 args=0 stdin=-1", output(&response.events, "stdout")?);
            assert_eq!("diagnostic", output(&response.events, "stderr")?);
        } else {
            assert_eq!(3, response.events.len());
        }
    }

    // These paths previously risked formatting a throwable after its VM heap was dropped.
    for (source, diagnostic) in [
        (
            "package example; public class Main { invalid Java; }",
            "Main.java:1:",
        ),
        (
            "package example; public class Main { public static void main(String[] args) { throw new RuntimeException(\"boom\"); } }",
            "example.Main.main(Main.java:1)",
        ),
    ] {
        let response = request(workspace, &java_home, 25, "run", source)?;
        assert_eq!(Some(0), response.code);
        let result = response.events.last().ok_or("missing error")?;
        assert_eq!("error", result["type"]);
        assert!(
            !result["message"]
                .as_str()
                .ok_or("missing message")?
                .is_empty()
        );
        assert!(output(&response.events, "stderr")?.contains(diagnostic));
    }

    let response = request(
        workspace,
        &java_home,
        25,
        "run",
        "package example; public class Main { public static void main(String[] args) { System.out.print(\"x\".repeat(1048577)); } }",
    )?;
    assert_eq!(Some(1), response.code);
    let result = response.events.last().ok_or("missing limit error")?;
    assert_eq!("error", result["type"]);
    assert_eq!(
        "Output exceeded 1 MiB; execution stopped.",
        result["message"]
    );
    assert!(output(&response.events, "stdout")?.len() <= 1024 * 1024);
    Ok(())
}
