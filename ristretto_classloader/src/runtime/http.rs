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
    use reqwest::Client;
    use reqwest::header::{HeaderMap, HeaderName, HeaderValue};

    let mut header_map = HeaderMap::new();
    for (name, value) in headers {
        let name = HeaderName::try_from(name.as_str())
            .map_err(|error| Error::ParseError(error.to_string()))?;
        let value = HeaderValue::try_from(value.as_str())
            .map_err(|error| Error::ParseError(error.to_string()))?;
        header_map.insert(name, value);
    }
    let response = Client::new()
        .get(url)
        .headers(header_map)
        .query(query)
        .send()
        .await?
        .error_for_status()?;
    Ok(response.bytes().await?.to_vec())
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

/// Follow up to 10 redirects when fetching a URL via wstd.
#[cfg(target_os = "wasi")]
async fn fetch_bytes(initial_url: &str, headers: &Headers) -> Result<Vec<u8>> {
    use wstd::http::{Body, Client, Request};

    let mut current = initial_url.to_string();
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
            current = resolve_redirect(&current, location);
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
