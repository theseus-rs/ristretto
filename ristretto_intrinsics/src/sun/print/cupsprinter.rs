use ristretto_classfile::VersionSpecification::{Any, GreaterThanOrEqual};
use ristretto_classfile::{JAVA_11, JAVA_25};
use ristretto_classloader::{Reference, Value};
use ristretto_macros::async_method;
use ristretto_macros::intrinsic_method;
use ristretto_types::{JavaObject, Parameters, Result, Thread, VM};
use std::env;
use std::fs;
use std::net::{SocketAddr, TcpStream, ToSocketAddrs};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

const DEFAULT_CUPS_PORT: i32 = 631;
const DEFAULT_CUPS_SERVER: &str = "localhost";
const CONNECT_TIMEOUT: Duration = Duration::from_millis(1500);

/// Returns the CUPS server hostname, honoring the `CUPS_SERVER` environment variable used by
/// libcups.  When the variable contains a `host:port` pair only the host portion is returned to
/// match `cupsServer()` semantics.  Returns `None` when the variable is unset or empty so the
/// caller can fall back to the default value.
fn cups_server_from_env() -> Option<String> {
    let raw = env::var("CUPS_SERVER").ok()?;
    let raw = raw.trim();
    if raw.is_empty() {
        return None;
    }
    let host = match raw.rfind(':') {
        Some(idx) if !raw.starts_with('[') => &raw[..idx],
        _ => raw,
    };
    Some(host.trim_matches(|c| c == '[' || c == ']').to_string())
}

/// Returns the IPP/CUPS port number, honoring the `IPP_PORT` environment variable and the
/// optional `:port` suffix on `CUPS_SERVER`.  Falls back to the IANA CUPS port (631) when neither
/// is set.
fn cups_port_from_env() -> i32 {
    if let Ok(value) = env::var("IPP_PORT")
        && let Ok(port) = value.trim().parse::<i32>()
        && (1..=65_535).contains(&port)
    {
        return port;
    }
    if let Ok(value) = env::var("CUPS_SERVER") {
        let value = value.trim();
        if !value.starts_with('[')
            && let Some(idx) = value.rfind(':')
            && let Ok(port) = value[idx + 1..].parse::<i32>()
            && (1..=65_535).contains(&port)
        {
            return port;
        }
    }
    DEFAULT_CUPS_PORT
}

/// Returns the candidate `lpoptions` files in the order CUPS itself searches them: per-user
/// overrides first (`~/.cups/lpoptions`, `~/.lpoptions`) followed by the system-wide
/// `/etc/cups/lpoptions`.
fn lpoptions_paths() -> Vec<PathBuf> {
    let mut paths = Vec::new();
    if let Ok(home) = env::var("HOME") {
        let home = PathBuf::from(home);
        paths.push(home.join(".cups").join("lpoptions"));
        paths.push(home.join(".lpoptions"));
    }
    paths.push(PathBuf::from("/etc/cups/lpoptions"));
    paths
}

/// Reads the first available `lpoptions` file and returns its contents.
fn read_first_lpoptions() -> Option<String> {
    for path in lpoptions_paths() {
        if let Ok(contents) = fs::read_to_string(&path) {
            return Some(contents);
        }
    }
    None
}

/// Parses an `lpoptions` file and extracts the destination name from each `Default <name>` or
/// `Dest <name>` entry.  CUPS treats the first `Default` line as the active default printer.
fn parse_lpoptions_default(contents: &str) -> Option<String> {
    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut parts = line.split_whitespace();
        let kind = parts.next()?;
        if kind.eq_ignore_ascii_case("Default") {
            return parts.next().map(str::to_string);
        }
    }
    None
}

/// Parses every destination entry from an `lpoptions` file, returning `(name, instance)` pairs.
/// A destination written as `printer/instance` is split into its two components; entries without
/// an explicit instance use `None`.
fn parse_lpoptions_destinations(contents: &str) -> Vec<(String, Option<String>)> {
    let mut destinations = Vec::new();
    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut parts = line.split_whitespace();
        let Some(kind) = parts.next() else {
            continue;
        };
        if !kind.eq_ignore_ascii_case("Default") && !kind.eq_ignore_ascii_case("Dest") {
            continue;
        }
        let Some(dest) = parts.next() else {
            continue;
        };
        let (name, instance) = match dest.split_once('/') {
            Some((n, i)) => (n.to_string(), Some(i.to_string())),
            None => (dest.to_string(), None),
        };
        destinations.push((name, instance));
    }
    destinations
}

/// Resolves the CUPS default printer name using the same priority order as libcups: the
/// `LPDEST` and `PRINTER` environment variables take precedence over `lpoptions` files.
fn detect_default_printer() -> Option<String> {
    for var in ["LPDEST", "PRINTER"] {
        if let Ok(value) = env::var(var) {
            let value = value.trim();
            if !value.is_empty() {
                let name = value.split('/').next().unwrap_or(value);
                return Some(name.to_string());
            }
        }
    }
    let contents = read_first_lpoptions()?;
    parse_lpoptions_default(&contents)
}

/// Returns the path to the PPD file CUPS would use for the given printer, sanitizing the name to
/// avoid path traversal.
fn ppd_path_for(printer: &str) -> Option<PathBuf> {
    if printer.is_empty() || printer.contains(['/', '\\', '\0']) {
        return None;
    }
    Some(PathBuf::from(format!("/etc/cups/ppd/{printer}.ppd")))
}

/// Reads the PPD file associated with a printer.
fn read_ppd(printer: &str) -> Option<String> {
    let path = ppd_path_for(printer)?;
    fs::read_to_string(path).ok()
}

/// Splits a PPD `key/text` pair on the first unescaped `/` character.  Per the PPD spec the key
/// is what CUPS uses programmatically and the text is the human-readable label shown in UIs.
fn split_ppd_key(raw: &str) -> (String, String) {
    if let Some(idx) = raw.find('/') {
        (raw[..idx].to_string(), raw[idx + 1..].to_string())
    } else {
        (raw.to_string(), raw.to_string())
    }
}

/// Parses a PPD file and returns the choice keys and human-readable labels for the requested
/// option (e.g. `PageSize`, `OutputBin`, `Resolution`).
fn parse_ppd_option_choices(ppd: &str, option: &str) -> Vec<(String, String)> {
    let prefix = format!("*{option} ");
    let mut choices = Vec::new();
    for line in ppd.lines() {
        let line = line.trim_start();
        let Some(rest) = line.strip_prefix(&prefix) else {
            continue;
        };
        let Some(colon) = rest.find(':') else {
            continue;
        };
        let raw_key = rest[..colon].trim();
        let (key, label) = split_ppd_key(raw_key);
        if !key.is_empty() {
            choices.push((key, label));
        }
    }
    choices
}

/// Parses a `*Default<Option>` line from a PPD, returning the choice key it points at.
fn parse_ppd_default(ppd: &str, option: &str) -> Option<String> {
    let prefix = format!("*Default{option}:");
    for line in ppd.lines() {
        let line = line.trim_start();
        if let Some(rest) = line.strip_prefix(&prefix) {
            let value = rest.trim();
            if value.is_empty() {
                return None;
            }
            return Some(split_ppd_key(value).0);
        }
    }
    None
}

/// Parses a `*PaperDimension` entry (`width height` in PostScript points) for the given page
/// size key.
fn parse_paper_dimension(ppd: &str, key: &str) -> Option<(f32, f32)> {
    parse_ppd_pair(ppd, "PaperDimension", key)
}

/// Parses an `*ImageableArea` entry (`llx lly urx ury` in PostScript points) for the given page
/// size key.
fn parse_imageable_area(ppd: &str, key: &str) -> Option<(f32, f32, f32, f32)> {
    let prefix = format!("*ImageableArea {key}:");
    for line in ppd.lines() {
        let line = line.trim_start();
        if let Some(rest) = line.strip_prefix(&prefix) {
            let values = extract_quoted_values(rest);
            if values.len() == 4 {
                return Some((values[0], values[1], values[2], values[3]));
            }
        }
    }
    None
}

/// Helper for two-number PPD entries such as `*PaperDimension`.
fn parse_ppd_pair(ppd: &str, option: &str, key: &str) -> Option<(f32, f32)> {
    let prefix = format!("*{option} {key}:");
    for line in ppd.lines() {
        let line = line.trim_start();
        if let Some(rest) = line.strip_prefix(&prefix) {
            let values = extract_quoted_values(rest);
            if values.len() == 2 {
                return Some((values[0], values[1]));
            }
        }
    }
    None
}

/// Extracts whitespace-separated `f32` numbers from a PPD value, stripping surrounding double
/// quotes if they exist (PPD scalar values are typically quoted).
fn extract_quoted_values(raw: &str) -> Vec<f32> {
    let trimmed = raw.trim().trim_matches('"');
    trimmed
        .split_whitespace()
        .filter_map(|token| token.parse::<f32>().ok())
        .collect()
}

/// Parses a CUPS resolution choice key such as `300dpi` or `600x1200dpi` into a `(x_dpi, y_dpi)`
/// pair.  Single-value resolutions are treated as square (x == y) which matches the behavior of
/// libcups when feeding the `ArrayList<Integer>` consumed by `CUPSPrinter`.
fn parse_resolution(key: &str) -> Option<(i32, i32)> {
    let lower = key.to_ascii_lowercase();
    let stripped = lower.strip_suffix("dpi").unwrap_or(&lower);
    if let Some((x, y)) = stripped.split_once('x') {
        let x = x.parse::<i32>().ok()?;
        let y = y.parse::<i32>().ok()?;
        Some((x, y))
    } else {
        let dpi = stripped.parse::<i32>().ok()?;
        Some((dpi, dpi))
    }
}

/// Attempts to resolve `host:port` and open a TCP connection within `CONNECT_TIMEOUT`.  Returns
/// `true` on the first successful connect, mirroring `httpConnect2()` in libcups.
fn try_tcp_connect(host: &str, port: u16) -> bool {
    let addrs: Vec<SocketAddr> = match (host, port).to_socket_addrs() {
        Ok(iter) => iter.collect(),
        Err(_) => return false,
    };
    for addr in addrs {
        if TcpStream::connect_timeout(&addr, CONNECT_TIMEOUT).is_ok() {
            return true;
        }
    }
    false
}

/// Returns `true` when the host is configured for CUPS.  The check is deliberately conservative:
/// we report success when either the standard CUPS configuration directory exists or a CUPS
/// environment override has been set.  This matches the role `initIDs()` plays in `OpenJDK`,
/// which is to gate the remaining CUPS native methods on whether libcups is available at runtime.
fn cups_available() -> bool {
    if env::var("CUPS_SERVER").is_ok() || env::var("IPP_PORT").is_ok() {
        return true;
    }
    PathBuf::from("/etc/cups").is_dir()
}

#[intrinsic_method("sun/print/CUPSPrinter.canConnect(Ljava/lang/String;I)Z", Any)]
#[async_method]
pub async fn can_connect<T: Thread + 'static>(
    _thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let port = parameters.pop_int()?;
    let server = parameters.pop()?;
    let server = server.as_string()?;
    let connected = if let Ok(port) = u16::try_from(port) {
        try_tcp_connect(&server, port)
    } else {
        false
    };
    Ok(Some(Value::from(connected)))
}

#[intrinsic_method("sun/print/CUPSPrinter.getCupsDefaultPrinter()Ljava/lang/String;", Any)]
#[async_method]
pub async fn get_cups_default_printer<T: Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    match detect_default_printer() {
        Some(name) => {
            let value = name.to_object(&*thread).await?;
            Ok(Some(value))
        }
        None => Ok(Some(Value::Object(None))),
    }
}

#[intrinsic_method(
    "sun/print/CUPSPrinter.getCupsDefaultPrinters()[Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_cups_default_printers<T: Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let Some(contents) = read_first_lpoptions() else {
        return Ok(Some(Value::Object(None)));
    };
    let destinations = parse_lpoptions_destinations(&contents);
    if destinations.is_empty() {
        return Ok(Some(Value::Object(None)));
    }
    let vm = thread.vm()?;
    let mut entries: Vec<Value> = Vec::with_capacity(destinations.len() * 2);
    for (name, instance) in destinations {
        entries.push(name.to_object(&*thread).await?);
        match instance {
            Some(instance) => entries.push(instance.to_object(&*thread).await?),
            None => entries.push(Value::Object(None)),
        }
    }
    let class = thread.class("[Ljava/lang/String;").await?;
    let reference = Reference::try_from((class, entries))?;
    Ok(Some(Value::new_object(vm.garbage_collector(), reference)))
}

#[intrinsic_method("sun/print/CUPSPrinter.getCupsPort()I", Any)]
#[async_method]
pub async fn get_cups_port<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::Int(cups_port_from_env())))
}

#[intrinsic_method("sun/print/CUPSPrinter.getCupsServer()Ljava/lang/String;", Any)]
#[async_method]
pub async fn get_cups_server<T: Thread + 'static>(
    thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    let server = cups_server_from_env().unwrap_or_else(|| DEFAULT_CUPS_SERVER.to_string());
    let value = server.to_object(&*thread).await?;
    Ok(Some(value))
}

#[intrinsic_method(
    "sun/print/CUPSPrinter.getMedia(Ljava/lang/String;)[Ljava/lang/String;",
    Any
)]
#[async_method]
pub async fn get_media<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let printer = parameters.pop()?;
    let printer = printer.as_string()?;
    let Some(ppd) = read_ppd(&printer) else {
        return Ok(Some(Value::Object(None)));
    };
    let choices = parse_ppd_option_choices(&ppd, "PageSize");
    if choices.is_empty() {
        return Ok(Some(Value::Object(None)));
    }
    let vm = thread.vm()?;
    let mut entries: Vec<Value> = Vec::with_capacity(choices.len() * 2);
    for (key, label) in choices {
        let display = if label.is_empty() { key.clone() } else { label };
        entries.push(display.to_object(&*thread).await?);
        entries.push(key.to_object(&*thread).await?);
    }
    let class = thread.class("[Ljava/lang/String;").await?;
    let reference = Reference::try_from((class, entries))?;
    Ok(Some(Value::new_object(vm.garbage_collector(), reference)))
}

#[intrinsic_method(
    "sun/print/CUPSPrinter.getOutputBins(Ljava/lang/String;)[Ljava/lang/String;",
    GreaterThanOrEqual(JAVA_25)
)]
#[async_method]
pub async fn get_output_bins<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let printer = parameters.pop()?;
    let printer = printer.as_string()?;
    let Some(ppd) = read_ppd(&printer) else {
        return Ok(Some(Value::Object(None)));
    };
    let choices = parse_ppd_option_choices(&ppd, "OutputBin");
    if choices.is_empty() {
        return Ok(Some(Value::Object(None)));
    }
    let vm = thread.vm()?;
    let mut entries: Vec<Value> = Vec::with_capacity(choices.len() * 2);
    for (key, label) in choices {
        let display = if label.is_empty() { key.clone() } else { label };
        entries.push(display.to_object(&*thread).await?);
        entries.push(key.to_object(&*thread).await?);
    }
    let class = thread.class("[Ljava/lang/String;").await?;
    let reference = Reference::try_from((class, entries))?;
    Ok(Some(Value::new_object(vm.garbage_collector(), reference)))
}

#[intrinsic_method("sun/print/CUPSPrinter.getPageSizes(Ljava/lang/String;)[F", Any)]
#[async_method]
pub async fn get_page_sizes<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let printer = parameters.pop()?;
    let printer = printer.as_string()?;
    let Some(ppd) = read_ppd(&printer) else {
        return Ok(Some(Value::Object(None)));
    };
    let choices = parse_ppd_option_choices(&ppd, "PageSize");
    if choices.is_empty() {
        return Ok(Some(Value::Object(None)));
    }
    // OpenJDK's CUPSPrinter.getPageSizes lays each page size out as six floats:
    //   [width, height, imageable_x, imageable_y, imageable_width, imageable_height]
    // followed by two trailing floats for the default size's width and height.
    let mut floats: Vec<f32> = Vec::with_capacity(choices.len() * 6 + 2);
    for (key, _) in &choices {
        let (width, height) = parse_paper_dimension(&ppd, key).unwrap_or((0.0, 0.0));
        let (llx, lly, urx, ury) = parse_imageable_area(&ppd, key).unwrap_or((0.0, 0.0, 0.0, 0.0));
        let imageable_width = (urx - llx).max(0.0);
        let imageable_height = (ury - lly).max(0.0);
        floats.extend_from_slice(&[width, height, llx, lly, imageable_width, imageable_height]);
    }
    let default_key =
        parse_ppd_default(&ppd, "PageSize").or_else(|| choices.first().map(|c| c.0.clone()));
    let (default_width, default_height) = match default_key.as_deref() {
        Some(key) => parse_paper_dimension(&ppd, key).unwrap_or((0.0, 0.0)),
        None => (0.0, 0.0),
    };
    floats.push(default_width);
    floats.push(default_height);
    let vm = thread.vm()?;
    Ok(Some(Value::new_object(
        vm.garbage_collector(),
        Reference::from(floats),
    )))
}

#[intrinsic_method(
    "sun/print/CUPSPrinter.getResolutions(Ljava/lang/String;Ljava/util/ArrayList;)V",
    GreaterThanOrEqual(JAVA_11)
)]
#[async_method]
pub async fn get_resolutions<T: Thread + 'static>(
    thread: Arc<T>,
    mut parameters: Parameters,
) -> Result<Option<Value>> {
    let array_list = parameters.pop()?;
    let printer = parameters.pop()?;
    let printer = printer.as_string()?;
    let Some(ppd) = read_ppd(&printer) else {
        return Ok(None);
    };
    let choices = parse_ppd_option_choices(&ppd, "Resolution");
    for (key, _) in choices {
        let Some((x_dpi, y_dpi)) = parse_resolution(&key) else {
            continue;
        };
        let x_value = thread
            .try_invoke(
                "java.lang.Integer",
                "valueOf(I)Ljava/lang/Integer;",
                &[Value::Int(x_dpi)],
            )
            .await?;
        thread
            .invoke(
                "java.util.ArrayList",
                "add(Ljava/lang/Object;)Z",
                &[array_list.clone(), x_value],
            )
            .await?;
        let y_value = thread
            .try_invoke(
                "java.lang.Integer",
                "valueOf(I)Ljava/lang/Integer;",
                &[Value::Int(y_dpi)],
            )
            .await?;
        thread
            .invoke(
                "java.util.ArrayList",
                "add(Ljava/lang/Object;)Z",
                &[array_list.clone(), y_value],
            )
            .await?;
    }
    Ok(None)
}

#[intrinsic_method("sun/print/CUPSPrinter.initIDs()Z", Any)]
#[async_method]
pub async fn init_ids<T: Thread + 'static>(
    _thread: Arc<T>,
    _parameters: Parameters,
) -> Result<Option<Value>> {
    Ok(Some(Value::from(cups_available())))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_PPD: &str = r#"*PPD-Adobe: "4.3"
*FormatVersion: "4.3"
*FileVersion: "1.0"
*LanguageVersion: English
*ModelName: "Test Printer"
*OpenUI *PageSize/Page Size: PickOne
*DefaultPageSize: Letter
*PageSize Letter/US Letter: "<</PageSize[612 792]>>setpagedevice"
*PageSize A4/A4: "<</PageSize[595 842]>>setpagedevice"
*CloseUI: *PageSize
*PaperDimension Letter: "612 792"
*PaperDimension A4: "595 842"
*ImageableArea Letter/US Letter: "12 12 600 780"
*ImageableArea A4: "10 10 585 832"
*OpenUI *OutputBin/Output Bin: PickOne
*DefaultOutputBin: Upper
*OutputBin Upper/Top Tray: ""
*OutputBin Lower/Bottom Tray: ""
*CloseUI: *OutputBin
*OpenUI *Resolution/Resolution: PickOne
*DefaultResolution: 600dpi
*Resolution 300dpi/300 DPI: "<</HWResolution[300 300]>>setpagedevice"
*Resolution 600dpi/600 DPI: "<</HWResolution[600 600]>>setpagedevice"
*Resolution 600x1200dpi/600x1200 DPI: ""
*CloseUI: *Resolution
"#;

    #[test]
    fn test_parse_ppd_option_choices() {
        let choices = parse_ppd_option_choices(SAMPLE_PPD, "PageSize");
        assert_eq!(
            choices,
            vec![
                ("Letter".to_string(), "US Letter".to_string()),
                ("A4".to_string(), "A4".to_string()),
            ]
        );
        let bins = parse_ppd_option_choices(SAMPLE_PPD, "OutputBin");
        assert_eq!(
            bins,
            vec![
                ("Upper".to_string(), "Top Tray".to_string()),
                ("Lower".to_string(), "Bottom Tray".to_string()),
            ]
        );
    }

    #[test]
    fn test_parse_ppd_default() {
        assert_eq!(
            parse_ppd_default(SAMPLE_PPD, "PageSize"),
            Some("Letter".to_string())
        );
        assert_eq!(parse_ppd_default(SAMPLE_PPD, "Missing"), None);
    }

    #[test]
    fn test_parse_paper_dimension_and_imageable_area() {
        assert_eq!(
            parse_paper_dimension(SAMPLE_PPD, "Letter"),
            Some((612.0, 792.0))
        );
        assert_eq!(
            parse_imageable_area(SAMPLE_PPD, "A4"),
            Some((10.0, 10.0, 585.0, 832.0))
        );
        assert_eq!(parse_paper_dimension(SAMPLE_PPD, "Missing"), None);
    }

    #[test]
    fn test_parse_resolution() {
        assert_eq!(parse_resolution("300dpi"), Some((300, 300)));
        assert_eq!(parse_resolution("600x1200dpi"), Some((600, 1200)));
        assert_eq!(parse_resolution("invalid"), None);
    }

    #[test]
    fn test_parse_lpoptions_default_and_destinations() {
        let contents = "# comment\nDefault HP_LaserJet/draft option=value\nDest Photo\n";
        assert_eq!(
            parse_lpoptions_default(contents),
            Some("HP_LaserJet/draft".to_string())
        );
        assert_eq!(
            parse_lpoptions_destinations(contents),
            vec![
                ("HP_LaserJet".to_string(), Some("draft".to_string())),
                ("Photo".to_string(), None),
            ]
        );
    }

    #[test]
    fn test_split_ppd_key() {
        assert_eq!(
            split_ppd_key("Letter/US Letter"),
            ("Letter".to_string(), "US Letter".to_string())
        );
        assert_eq!(
            split_ppd_key("Plain"),
            ("Plain".to_string(), "Plain".to_string())
        );
    }

    #[test]
    fn test_extract_quoted_values() {
        assert_eq!(extract_quoted_values(r#""612 792""#), vec![612.0, 792.0]);
        assert_eq!(
            extract_quoted_values("9.36 12.36 593.76 780.0"),
            vec![9.36, 12.36, 593.76, 780.0]
        );
    }

    #[test]
    fn test_ppd_path_for_rejects_traversal() {
        assert!(ppd_path_for("../etc/passwd").is_none());
        assert!(ppd_path_for("").is_none());
        assert_eq!(
            ppd_path_for("HP_LaserJet").unwrap(),
            PathBuf::from("/etc/cups/ppd/HP_LaserJet.ppd")
        );
    }

    #[tokio::test]
    async fn test_can_connect_unreachable() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let server = "127.0.0.1".to_object(&*thread).await?;
        let mut params = Parameters::default();
        params.push(server);
        params.push(Value::Int(1)); // unprivileged port unlikely to be open
        let result = can_connect(thread, params).await?;
        assert_eq!(result, Some(Value::from(false)));
        Ok(())
    }

    #[tokio::test]
    async fn test_get_cups_default_printer_returns_value_or_null() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = get_cups_default_printer(thread, Parameters::default()).await?;
        // The returned value is environment-dependent: a String when a default is configured,
        // null otherwise.  Either is valid.
        match result {
            Some(Value::Object(_)) => {}
            other => panic!("unexpected result: {other:?}"),
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_get_cups_default_printers_no_panic() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = get_cups_default_printers(thread, Parameters::default()).await?;
        assert!(matches!(result, Some(Value::Object(_))));
        Ok(())
    }

    #[tokio::test]
    async fn test_get_cups_port_returns_valid_port() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = get_cups_port(thread, Parameters::default()).await?;
        let Some(Value::Int(port)) = result else {
            panic!("expected Int, got {result:?}");
        };
        assert!((1..=65_535).contains(&port));
        Ok(())
    }

    #[tokio::test]
    async fn test_get_cups_server_returns_string() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = get_cups_server(thread, Parameters::default()).await?;
        assert!(matches!(result, Some(Value::Object(_))));
        Ok(())
    }

    #[tokio::test]
    async fn test_get_media_missing_printer() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let printer = "__nonexistent_ristretto_printer__"
            .to_object(&*thread)
            .await?;
        let mut params = Parameters::default();
        params.push(printer);
        let result = get_media(thread, params).await?;
        assert_eq!(result, Some(Value::Object(None)));
        Ok(())
    }

    #[tokio::test]
    async fn test_get_output_bins_missing_printer() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let printer = "__nonexistent_ristretto_printer__"
            .to_object(&*thread)
            .await?;
        let mut params = Parameters::default();
        params.push(printer);
        let result = get_output_bins(thread, params).await?;
        assert_eq!(result, Some(Value::Object(None)));
        Ok(())
    }

    #[tokio::test]
    async fn test_get_page_sizes_missing_printer() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let printer = "__nonexistent_ristretto_printer__"
            .to_object(&*thread)
            .await?;
        let mut params = Parameters::default();
        params.push(printer);
        let result = get_page_sizes(thread, params).await?;
        assert_eq!(result, Some(Value::Object(None)));
        Ok(())
    }

    #[tokio::test]
    async fn test_get_resolutions_missing_printer() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let printer = "__nonexistent_ristretto_printer__"
            .to_object(&*thread)
            .await?;
        let mut params = Parameters::default();
        params.push(printer);
        params.push(Value::Object(None));
        let result = get_resolutions(thread, params).await?;
        assert_eq!(result, None);
        Ok(())
    }

    #[tokio::test]
    async fn test_init_ids_returns_boolean() -> Result<()> {
        let (_vm, thread) = crate::test::thread().await?;
        let result = init_ids(thread, Parameters::default()).await?;
        assert!(matches!(result, Some(Value::Int(0 | 1))));
        Ok(())
    }
}
