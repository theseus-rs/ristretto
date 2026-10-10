//! HTTP helpers used by the runtime archive downloader.
//!
//! On native targets these wrap `reqwest`. On `wasm32-wasip2` they use `wstd`
//! (which targets the `wasi:http` interface). The same async signatures are
//! exposed so callers don't need to care.

#[cfg(not(all(target_family = "wasm", target_os = "unknown")))]
use crate::{Error, Result};

/// Headers passed to the HTTP helpers.
pub(crate) type Headers = Vec<(String, String)>;

/// Issue an HTTP GET and return the response body as bytes.
#[cfg(not(target_family = "wasm"))]
pub(crate) async fn get_bytes(
    url: &str,
    headers: &Headers,
    query: &[(&str, &str)],
) -> Result<Vec<u8>> {
    use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

    let mut header_map = HeaderMap::new();
    for (name, value) in headers {
        let name = HeaderName::try_from(name.as_str())
            .map_err(|error| Error::ParseError(error.to_string()))?;
        let value = HeaderValue::try_from(value.as_str())
            .map_err(|error| Error::ParseError(error.to_string()))?;
        header_map.insert(name, value);
    }
    let client = crate::tls::reqwest_client()?;
    let mut retries = 0_u32;
    loop {
        let result = match client
            .get(url)
            .headers(header_map.clone())
            .query(query)
            .send()
            .await
        {
            Ok(response) => response.error_for_status()?.bytes().await,
            Err(error) => Err(error),
        };
        match result {
            Ok(bytes) => return Ok(bytes.to_vec()),
            // Connections can fail before the headers arrive or before the body is complete.
            // Retry the whole GET so that partial archives are never returned to the extractor.
            Err(error)
                if retries < 2 && (error.is_request() || error.is_body() || error.is_decode()) =>
            {
                retries += 1;
                tracing::warn!(%error, retries, "Retrying failed runtime download");
                tokio::time::sleep(std::time::Duration::from_millis(100 * u64::from(retries)))
                    .await;
            }
            Err(error) => return Err(error.into()),
        }
    }
}

/// Issue an HTTP GET and deserialize the response body as JSON.
#[cfg(not(target_family = "wasm"))]
pub(crate) async fn get_json<T>(url: &str, headers: &Headers, query: &[(&str, &str)]) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let bytes = get_bytes(url, headers, query).await?;
    serde_json::from_slice::<T>(&bytes).map_err(|error| Error::SerdeError(error.to_string()))
}

#[cfg(target_os = "wasi")]
#[expect(clippy::unused_async)]
pub(crate) async fn get_bytes(
    url: &str,
    headers: &Headers,
    query: &[(&str, &str)],
) -> Result<Vec<u8>> {
    let url = build_url(url, query);
    let headers = headers.clone();
    wstd::runtime::block_on(async move { fetch_bytes(&url, &headers).await })
}

#[cfg(target_os = "wasi")]
pub(crate) async fn get_json<T>(url: &str, headers: &Headers, query: &[(&str, &str)]) -> Result<T>
where
    T: serde::de::DeserializeOwned,
{
    let bytes = get_bytes(url, headers, query).await?;
    serde_json::from_slice::<T>(&bytes).map_err(|error| Error::SerdeError(error.to_string()))
}

#[cfg(target_os = "wasi")]
fn build_url(url: &str, query: &[(&str, &str)]) -> String {
    if query.is_empty() {
        return url.to_string();
    }
    let separator = if url.contains('?') { '&' } else { '?' };
    let pairs = query
        .iter()
        .map(|(k, v)| format!("{}={}", urlencode(k), urlencode(v)))
        .collect::<Vec<_>>()
        .join("&");
    format!("{url}{separator}{pairs}")
}

#[cfg(target_os = "wasi")]
fn urlencode(value: &str) -> String {
    use std::fmt::Write;
    let mut out = String::with_capacity(value.len());
    for byte in value.as_bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                out.push(*byte as char);
            }
            other => {
                let _ = write!(out, "%{other:02X}");
            }
        }
    }
    out
}

/// Follow up to 10 redirects when fetching a URL via wstd, detecting cycles.
#[cfg(target_os = "wasi")]
async fn fetch_bytes(initial_url: &str, headers: &Headers) -> Result<Vec<u8>> {
    use std::collections::HashSet;
    use wstd::http::{Body, Client, Request};

    let mut current = initial_url.to_string();
    let mut visited: HashSet<String> = HashSet::new();
    visited.insert(current.clone());
    let client = Client::new();
    for _ in 0..10 {
        let mut builder = Request::builder().method("GET").uri(&current);
        for (name, value) in headers {
            builder = builder.header(name.as_str(), value.as_str());
        }
        let request = builder
            .body(Body::empty())
            .map_err(|error| Error::RequestError(error.to_string()))?;
        let mut response = client
            .send(request)
            .await
            .map_err(|error| Error::RequestError(error.to_string()))?;
        let status = response.status();
        if status.is_redirection() {
            let Some(location) = response
                .headers()
                .get("location")
                .and_then(|v| v.to_str().ok())
            else {
                return Err(Error::RequestError(format!(
                    "redirect status {status} without Location header",
                )));
            };
            let next = resolve_redirect(&current, location);
            if !visited.insert(next.clone()) {
                return Err(Error::RequestError(format!(
                    "redirect cycle detected at {next}",
                )));
            }
            current = next;
            continue;
        }
        if !status.is_success() {
            return Err(Error::RequestError(format!(
                "request to {current} failed with status {status}",
            )));
        }
        let bytes = response
            .body_mut()
            .contents()
            .await
            .map_err(|error| Error::RequestError(error.to_string()))?
            .to_vec();
        return Ok(bytes);
    }
    Err(Error::RequestError(format!(
        "too many redirects starting at {initial_url}",
    )))
}

#[cfg(target_os = "wasi")]
fn resolve_redirect(base: &str, location: &str) -> String {
    if location.starts_with("http://") || location.starts_with("https://") {
        return location.to_string();
    }
    if location.starts_with('/')
        && let Some(scheme_end) = base.find("://")
    {
        let after_scheme = &base[scheme_end + 3..];
        if let Some(path_start) = after_scheme.find('/') {
            return format!("{}{}", &base[..scheme_end + 3 + path_start], location);
        }
        return format!("{base}{location}");
    }
    if let Some(slash) = base.rfind('/') {
        format!("{}/{}", &base[..slash], location)
    } else {
        location.to_string()
    }
}

#[cfg(all(test, not(target_family = "wasm")))]
mod tests {
    #![expect(
        clippy::panic_in_result_fn,
        reason = "tests assert HTTP results after fallible local server setup"
    )]

    use super::*;
    use std::time::Duration;
    use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
    use tokio::net::TcpListener;

    const TRUNCATED: &[u8] =
        b"HTTP/1.1 200 OK\r\nContent-Length: 4\r\nConnection: close\r\n\r\npar";
    const COMPLETE: &[u8] =
        b"HTTP/1.1 200 OK\r\nContent-Length: 4\r\nConnection: close\r\n\r\nfull";
    const CLOSED: &[u8] = b"";

    async fn serve_responses(listener: &TcpListener, responses: &[&[u8]]) -> std::io::Result<()> {
        for response in responses {
            let (stream, _) = listener.accept().await?;
            let mut stream = BufReader::new(stream);
            loop {
                let mut line = String::new();
                if stream.read_line(&mut line).await? == 0 || line == "\r\n" {
                    break;
                }
            }
            stream.get_mut().write_all(response).await?;
            stream.get_mut().shutdown().await?;
        }
        Ok(())
    }

    async fn fetch_responses(
        responses: &[&[u8]],
    ) -> std::result::Result<Result<Vec<u8>>, Box<dyn std::error::Error>> {
        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let url = format!("http://{}/archive", listener.local_addr()?);
        let headers = Headers::new();
        let (result, server) = tokio::time::timeout(Duration::from_secs(5), async {
            tokio::join!(
                get_bytes(&url, &headers, &[]),
                serve_responses(&listener, responses)
            )
        })
        .await?;
        server?;
        Ok(result)
    }

    #[tokio::test]
    async fn test_retry_truncated_body() -> std::result::Result<(), Box<dyn std::error::Error>> {
        assert_eq!(fetch_responses(&[TRUNCATED, COMPLETE]).await??, b"full");
        Ok(())
    }

    #[tokio::test]
    async fn test_truncated_body_retry_limit() -> std::result::Result<(), Box<dyn std::error::Error>>
    {
        assert!(matches!(
            fetch_responses(&[TRUNCATED; 3]).await?,
            Err(Error::RequestError(_))
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_retry_closed_connection() -> std::result::Result<(), Box<dyn std::error::Error>> {
        assert_eq!(fetch_responses(&[CLOSED, COMPLETE]).await??, b"full");
        Ok(())
    }

    #[tokio::test]
    async fn test_closed_connection_retry_limit()
    -> std::result::Result<(), Box<dyn std::error::Error>> {
        assert!(matches!(
            fetch_responses(&[CLOSED; 3]).await?,
            Err(Error::RequestError(_))
        ));
        Ok(())
    }

    #[tokio::test]
    async fn test_http_error_is_not_retried() -> std::result::Result<(), Box<dyn std::error::Error>>
    {
        let response = b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\nConnection: close\r\n\r\n";
        assert!(matches!(
            fetch_responses(&[response]).await?,
            Err(Error::RequestError(_))
        ));
        Ok(())
    }
}
