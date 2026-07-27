//! Streaming access to repository resources.
//!
//! [`Transport`] implementations receive a validated [`TransportRequest`] and return a
//! [`TransportResponse`] whose body is consumed incrementally. Native builds provide
//! [`HttpTransport`] and [`FileTransport`]; callers can register additional schemes through
//! [`ResolverBuilder::transport`](crate::ResolverBuilder::transport). Artifact bytes flow directly
//! into a transactional destination rather than being buffered by the resolver.

#[cfg(not(target_family = "wasm"))]
use crate::{Authentication, Error};
use crate::{BoxFuture, RemoteRepository, Result};
use bytes::Bytes;
use futures_util::Stream;
#[cfg(not(target_family = "wasm"))]
use futures_util::TryStreamExt;
use std::fmt;
use std::pin::Pin;

/// A fallible stream of downloaded byte chunks.
#[cfg(not(target_family = "wasm"))]
pub type ByteStream = Pin<Box<dyn Stream<Item = Result<Bytes>> + Send>>;

/// A fallible stream of downloaded byte chunks.
#[cfg(target_family = "wasm")]
pub type ByteStream = Pin<Box<dyn Stream<Item = Result<Bytes>>>>;

/// Type of repository resource being retrieved.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResourceKind {
    /// POM dependency descriptor.
    Pom,
    /// Repository metadata.
    Metadata,
    /// Version-level metadata for a mutable snapshot.
    SnapshotMetadata,
    /// Binary or attached artifact.
    Artifact,
    /// Checksum sidecar.
    Checksum,
}

/// Input passed to a registered transport.
#[derive(Debug, Clone)]
pub struct TransportRequest {
    /// Fully resolved resource URL.
    pub url: String,
    /// Repository configuration.
    pub repository: RemoteRepository,
    /// Resource category.
    pub kind: ResourceKind,
}

/// Successful transport response.
pub struct TransportResponse {
    /// Response body.
    pub body: ByteStream,
    /// Declared response size.
    pub content_length: Option<u64>,
    /// Whether the body came from the optional local repository cache.
    pub from_cache: bool,
}

impl fmt::Debug for TransportResponse {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("TransportResponse")
            .field("content_length", &self.content_length)
            .field("from_cache", &self.from_cache)
            .finish_non_exhaustive()
    }
}

/// Extensible repository resource transport.
pub trait Transport: fmt::Debug + Send + Sync {
    /// Returns whether this transport supports a URL scheme.
    fn supports(&self, scheme: &str) -> bool;

    /// Retrieves a resource, returning `None` for a definitive not-found response.
    fn get<'a>(
        &'a self,
        request: &'a TransportRequest,
    ) -> BoxFuture<'a, Result<Option<TransportResponse>>>;
}

/// Encoding of a configured TLS trust anchor.
#[cfg(not(target_family = "wasm"))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CertificateEncoding {
    /// PEM-encoded X.509 certificate.
    Pem,
    /// DER-encoded X.509 certificate.
    Der,
}

/// One custom TLS trust anchor.
#[cfg(not(target_family = "wasm"))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TrustAnchor {
    /// Certificate encoding.
    pub encoding: CertificateEncoding,
    /// Certificate bytes.
    pub certificate: Vec<u8>,
}

/// TLS trust configuration for [`HttpTransport`].
#[cfg(not(target_family = "wasm"))]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TlsConfiguration {
    /// Whether platform/bundled trust roots remain enabled.
    pub default_roots: bool,
    /// Additional or replacement trust anchors.
    pub trust_anchors: Vec<TrustAnchor>,
}

#[cfg(not(target_family = "wasm"))]
impl Default for TlsConfiguration {
    fn default() -> Self {
        Self {
            default_roots: true,
            trust_anchors: Vec::new(),
        }
    }
}

#[cfg(not(target_family = "wasm"))]
impl TlsConfiguration {
    /// Adds a PEM-encoded trust anchor.
    #[must_use]
    pub fn with_pem_certificate(mut self, certificate: impl Into<Vec<u8>>) -> Self {
        self.trust_anchors.push(TrustAnchor {
            encoding: CertificateEncoding::Pem,
            certificate: certificate.into(),
        });
        self
    }

    /// Adds a DER-encoded trust anchor.
    #[must_use]
    pub fn with_der_certificate(mut self, certificate: impl Into<Vec<u8>>) -> Self {
        self.trust_anchors.push(TrustAnchor {
            encoding: CertificateEncoding::Der,
            certificate: certificate.into(),
        });
        self
    }

    /// Uses only explicitly configured trust anchors.
    #[must_use]
    pub fn only_configured_roots(mut self) -> Self {
        self.default_roots = false;
        self
    }
}

/// Native HTTP and HTTPS transport.
#[cfg(not(target_family = "wasm"))]
#[derive(Debug, Clone, Default)]
pub struct HttpTransport {
    tls: TlsConfiguration,
}

#[cfg(not(target_family = "wasm"))]
impl HttpTransport {
    /// Creates an HTTP transport with explicit TLS trust configuration.
    #[must_use]
    pub fn with_tls(tls: TlsConfiguration) -> Self {
        Self { tls }
    }

    /// Returns this transport's TLS trust configuration.
    #[must_use]
    pub fn tls(&self) -> &TlsConfiguration {
        &self.tls
    }
}

#[cfg(not(target_family = "wasm"))]
impl Transport for HttpTransport {
    fn supports(&self, scheme: &str) -> bool {
        matches!(scheme, "http" | "https")
    }

    fn get<'a>(
        &'a self,
        request: &'a TransportRequest,
    ) -> BoxFuture<'a, Result<Option<TransportResponse>>> {
        Box::pin(async move {
            use futures_util::StreamExt;
            use reqwest::header::{HeaderName, HeaderValue};

            request.repository.validate_resource_url(&request.url)?;
            let mut builder = client_builder(&self.tls)?;
            if matches!(
                &request.repository.authentication,
                Some(Authentication::Headers(_))
            ) {
                builder = builder.redirect(reqwest::redirect::Policy::custom(|attempt| {
                    if attempt.previous().len() >= 10 {
                        return attempt.error("too many redirects");
                    }
                    if attempt
                        .previous()
                        .last()
                        .is_some_and(|previous| !same_origin(previous, attempt.url()))
                    {
                        return attempt.error(
                            "refusing to forward custom authentication headers across origins",
                        );
                    }
                    attempt.follow()
                }));
            }
            if let Some(proxy) = &request.repository.proxy
                && proxy_applies(proxy, &request.url)
            {
                let mut reqwest_proxy = configured_proxy(proxy, &request.url)?;
                if let Some(Authentication::Basic { username, password }) = &proxy.authentication {
                    reqwest_proxy = reqwest_proxy.basic_auth(username, password.expose());
                }
                builder = builder.proxy(reqwest_proxy);
            }
            let client = builder.build()?;
            let mut request_builder = client.get(&request.url);
            match &request.repository.authentication {
                Some(Authentication::Basic { username, password }) => {
                    request_builder = request_builder.basic_auth(username, Some(password.expose()));
                }
                Some(Authentication::Bearer(token)) => {
                    request_builder = request_builder.bearer_auth(token.expose());
                }
                Some(Authentication::Headers(headers)) => {
                    for (name, value) in headers {
                        let name = HeaderName::try_from(name.as_str()).map_err(|error| {
                            Error::InvalidConfiguration(format!(
                                "invalid authentication header: {error}"
                            ))
                        })?;
                        let mut value = HeaderValue::try_from(value.expose()).map_err(|error| {
                            Error::InvalidConfiguration(format!(
                                "invalid authentication header value: {error}"
                            ))
                        })?;
                        value.set_sensitive(true);
                        request_builder = request_builder.header(name, value);
                    }
                }
                None => {}
            }
            let response = request_builder.send().await?;
            if response.status() == reqwest::StatusCode::NOT_FOUND {
                return Ok(None);
            }
            let response = response.error_for_status()?;
            let content_length = response.content_length();
            let body = response.bytes_stream().map(|result| {
                result.map_err(|error| Error::Transport {
                    url: error
                        .url()
                        .map_or_else(|| "<stream>".to_string(), ToString::to_string),
                    message: error.to_string(),
                })
            });
            Ok(Some(TransportResponse {
                body: Box::pin(body),
                content_length,
                from_cache: false,
            }))
        })
    }
}

#[cfg(not(target_family = "wasm"))]
fn same_origin(left: &url::Url, right: &url::Url) -> bool {
    left.scheme() == right.scheme()
        && left.host_str() == right.host_str()
        && left.port_or_known_default() == right.port_or_known_default()
}

#[cfg(not(target_family = "wasm"))]
fn client_builder(tls: &TlsConfiguration) -> Result<reqwest::ClientBuilder> {
    #[cfg(feature = "tls-rustls-ring")]
    {
        use std::sync::Once;

        static INIT: Once = Once::new();
        INIT.call_once(|| {
            drop(rustls::crypto::ring::default_provider().install_default());
        });
    }

    #[cfg(any(
        feature = "tls-native-tls",
        feature = "tls-rustls-aws-lc-rs",
        feature = "tls-rustls-ring"
    ))]
    {
        let certificates = tls
            .trust_anchors
            .iter()
            .map(|anchor| {
                let certificate = match anchor.encoding {
                    CertificateEncoding::Pem => reqwest::Certificate::from_pem(&anchor.certificate),
                    CertificateEncoding::Der => reqwest::Certificate::from_der(&anchor.certificate),
                };
                certificate.map_err(|error| {
                    Error::InvalidConfiguration(format!(
                        "invalid TLS trust anchor certificate: {error}"
                    ))
                })
            })
            .collect::<Result<Vec<_>>>()?;
        let builder = {
            #[cfg(any(feature = "tls-rustls-aws-lc-rs", feature = "tls-rustls-ring"))]
            {
                reqwest::Client::builder().tls_backend_rustls()
            }
            #[cfg(not(any(feature = "tls-rustls-aws-lc-rs", feature = "tls-rustls-ring")))]
            {
                reqwest::Client::builder()
            }
        };
        Ok(if tls.default_roots {
            builder.tls_certs_merge(certificates)
        } else {
            builder.tls_certs_only(certificates)
        })
    }
    #[cfg(not(any(
        feature = "tls-native-tls",
        feature = "tls-rustls-aws-lc-rs",
        feature = "tls-rustls-ring"
    )))]
    {
        if !tls.default_roots || !tls.trust_anchors.is_empty() {
            return Err(Error::InvalidConfiguration(
                "TLS trust configuration requires a resolver TLS feature".to_string(),
            ));
        }
        Ok(reqwest::Client::builder())
    }
}

#[cfg(not(target_family = "wasm"))]
fn proxy_applies(proxy: &crate::Proxy, resource_url: &str) -> bool {
    let Ok(url) = url::Url::parse(resource_url) else {
        return true;
    };
    let Some(host) = url.host_str() else {
        return true;
    };
    !proxy
        .non_proxy_hosts
        .iter()
        .flat_map(|patterns| patterns.split(['|', ',']))
        .map(str::trim)
        .any(|pattern| !pattern.is_empty() && host_matches(pattern, host))
}

#[cfg(not(target_family = "wasm"))]
fn configured_proxy(proxy: &crate::Proxy, request_url: &str) -> Result<reqwest::Proxy> {
    reqwest::Proxy::all(&proxy.url).map_err(|error| Error::Transport {
        url: request_url.to_string(),
        message: error.to_string(),
    })
}

#[cfg(not(target_family = "wasm"))]
fn host_matches(pattern: &str, host: &str) -> bool {
    let pattern = pattern.to_ascii_lowercase();
    let host = host.to_ascii_lowercase();
    let pattern = pattern.as_bytes();
    let host = host.as_bytes();
    let (mut pattern_index, mut host_index) = (0, 0);
    let (mut star, mut retry_host) = (None, 0);

    while host_index < host.len() {
        if pattern.get(pattern_index) == host.get(host_index) {
            pattern_index += 1;
            host_index += 1;
        } else if pattern.get(pattern_index) == Some(&b'*') {
            star = Some(pattern_index);
            pattern_index += 1;
            retry_host = host_index;
        } else if let Some(star_index) = star {
            retry_host += 1;
            host_index = retry_host;
            pattern_index = star_index + 1;
        } else {
            return false;
        }
    }
    pattern
        .get(pattern_index..)
        .is_some_and(|remaining| remaining.iter().all(|byte| *byte == b'*'))
}

#[cfg(all(test, not(target_family = "wasm")))]
mod tests {
    #![expect(
        clippy::expect_used,
        reason = "transport fixtures use explicit panic messages for setup failures"
    )]

    use super::{
        CertificateEncoding, FileTransport, HttpTransport, TlsConfiguration, client_builder,
        host_matches, proxy_applies, same_origin,
    };
    use crate::{
        Authentication, Proxy, RemoteRepository, ResourceKind, SecretString, Transport,
        TransportRequest,
    };
    use futures_util::StreamExt;
    use std::collections::BTreeMap;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;
    use url::Url;

    async fn serve_once(
        response: &'static [u8],
    ) -> (String, tokio::task::JoinHandle<std::io::Result<Vec<u8>>>) {
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind HTTP fixture");
        let address = listener.local_addr().expect("HTTP fixture address");
        let task = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await?;
            let mut request = vec![0; 16 * 1024];
            let count = stream.read(&mut request).await?;
            request.truncate(count);
            stream.write_all(response).await?;
            Ok(request)
        });
        (format!("http://{address}/"), task)
    }

    async fn http_request(authentication: Option<Authentication>) -> (Vec<u8>, Vec<u8>) {
        let (url, server) =
            serve_once(b"HTTP/1.1 200 OK\r\nContent-Length: 4\r\nConnection: close\r\n\r\nbody")
                .await;
        let mut repository = RemoteRepository::new("test", &url).expect("HTTP fixture repository");
        repository.authentication = authentication;
        let request = TransportRequest {
            repository,
            url,
            kind: ResourceKind::Pom,
        };
        let mut response = HttpTransport::default()
            .get(&request)
            .await
            .expect("HTTP request")
            .expect("HTTP response");
        let body = response
            .body
            .next()
            .await
            .expect("body chunk")
            .expect("body bytes")
            .to_vec();
        let request = server
            .await
            .expect("HTTP server task")
            .expect("HTTP request");
        (request, body)
    }

    #[test]
    fn non_proxy_host_patterns_are_case_insensitive_globs() {
        assert!(host_matches("*.example.com", "repo.EXAMPLE.com"));
        assert!(host_matches(
            "repo*-internal.example",
            "repo17-internal.example"
        ));
        assert!(host_matches("LOCALHOST", "localhost"));
        assert!(!host_matches("*.example.com", "example.org"));
        assert!(!host_matches("repo.example", "repo.invalid"));
    }

    #[test]
    fn configures_explicit_tls_trust() {
        let tls = TlsConfiguration::default()
            .with_pem_certificate(b"invalid pem".to_vec())
            .with_der_certificate(vec![1, 2, 3])
            .only_configured_roots();
        let transport = HttpTransport::with_tls(tls.clone());
        assert_eq!(transport.tls(), &tls);
        assert!(!transport.tls().default_roots);
        assert_eq!(
            transport
                .tls()
                .trust_anchors
                .first()
                .expect("PEM trust anchor")
                .encoding,
            CertificateEncoding::Pem
        );
        assert!(
            client_builder(&tls)
                .and_then(|builder| builder.build().map_err(Into::into))
                .is_err()
        );
        assert!(
            client_builder(&TlsConfiguration::default().with_der_certificate(Vec::new()))
                .and_then(|builder| builder.build().map_err(Into::into))
                .is_err()
        );
        let configured_roots = client_builder(&TlsConfiguration::default().only_configured_roots());
        #[cfg(any(
            feature = "tls-native-tls",
            feature = "tls-rustls-aws-lc-rs",
            feature = "tls-rustls-ring"
        ))]
        assert!(configured_roots.is_ok());
        #[cfg(not(any(
            feature = "tls-native-tls",
            feature = "tls-rustls-aws-lc-rs",
            feature = "tls-rustls-ring"
        )))]
        assert!(configured_roots.is_err());
    }

    #[tokio::test]
    async fn performs_http_requests_with_every_authentication_kind() {
        let (request, body) = http_request(None).await;
        assert_eq!(body, b"body");
        assert!(String::from_utf8_lossy(&request).starts_with("GET /"));

        let (request, _) = http_request(Some(Authentication::Basic {
            username: "user".to_string(),
            password: SecretString::new("pass"),
        }))
        .await;
        assert!(
            String::from_utf8_lossy(&request)
                .to_ascii_lowercase()
                .contains("authorization: basic dxnlcjpwyxnz")
        );

        let (request, _) =
            http_request(Some(Authentication::Bearer(SecretString::new("token")))).await;
        assert!(
            String::from_utf8_lossy(&request)
                .to_ascii_lowercase()
                .contains("authorization: bearer token")
        );

        let (request, _) = http_request(Some(Authentication::Headers(BTreeMap::from([(
            "X-Repository-Token".to_string(),
            SecretString::new("custom"),
        )]))))
        .await;
        assert!(
            String::from_utf8_lossy(&request)
                .to_ascii_lowercase()
                .contains("x-repository-token: custom")
        );
    }

    #[tokio::test]
    #[expect(
        clippy::too_many_lines,
        reason = "the HTTP failure scenarios share the same one-shot server harness"
    )]
    async fn handles_http_statuses_invalid_headers_and_cross_origin_redirects() {
        let (url, server) =
            serve_once(b"HTTP/1.1 404 Not Found\r\nContent-Length: 0\r\n\r\n").await;
        let repository = RemoteRepository::new("test", &url).expect("repository");
        let request = TransportRequest {
            repository,
            url,
            kind: ResourceKind::Metadata,
        };
        assert!(
            HttpTransport::default()
                .get(&request)
                .await
                .expect("404 request")
                .is_none()
        );
        server.await.expect("server task").expect("request");

        let (url, server) = serve_once(
            b"HTTP/1.1 500 Internal Server Error\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
        )
        .await;
        let repository = RemoteRepository::new("test", &url).expect("repository");
        let request = TransportRequest {
            repository,
            url,
            kind: ResourceKind::Pom,
        };
        assert!(HttpTransport::default().get(&request).await.is_err());
        server.await.expect("server task").expect("request");

        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("bind redirect fixture");
        let redirect_base = format!(
            "http://{}/",
            listener.local_addr().expect("redirect fixture address")
        );
        let redirect_url = format!("{redirect_base}redirect");
        let redirect_target = redirect_url.clone();
        let redirect_server = tokio::spawn(async move {
            loop {
                let (mut stream, _) = listener.accept().await.expect("redirect connection");
                let mut request = vec![0; 16 * 1024];
                let _ = stream.read(&mut request).await.expect("redirect request");
                let response = format!(
                    "HTTP/1.1 302 Found\r\nLocation: {redirect_target}\r\n\
                     Content-Length: 0\r\nConnection: close\r\n\r\n"
                );
                stream
                    .write_all(response.as_bytes())
                    .await
                    .expect("redirect response");
            }
        });
        let mut repository =
            RemoteRepository::new("redirects", redirect_base).expect("redirect repository");
        repository.authentication = Some(Authentication::Headers(BTreeMap::from([(
            "X-Token".to_string(),
            SecretString::new("value"),
        )])));
        let redirect_request = TransportRequest {
            repository,
            url: redirect_url,
            kind: ResourceKind::Pom,
        };
        assert!(
            HttpTransport::default()
                .get(&redirect_request)
                .await
                .is_err()
        );
        redirect_server.abort();

        let (url, server) =
            serve_once(b"HTTP/1.1 200 OK\r\nContent-Length: 5\r\nConnection: close\r\n\r\nx").await;
        let repository = RemoteRepository::new("test", &url).expect("repository");
        let request = TransportRequest {
            repository,
            url,
            kind: ResourceKind::Artifact,
        };
        let mut response = HttpTransport::default()
            .get(&request)
            .await
            .expect("truncated response")
            .expect("response");
        assert!(format!("{response:?}").contains("content_length"));
        let mut stream_failed = false;
        while let Some(chunk) = response.body.next().await {
            if chunk.is_err() {
                stream_failed = true;
                break;
            }
        }
        assert!(stream_failed);
        server.await.expect("server task").expect("request");

        for (name, value) in [("bad header", "value"), ("X-Token", "bad\nvalue")] {
            let mut repository =
                RemoteRepository::new("test", "http://127.0.0.1:9/").expect("repository");
            repository.authentication = Some(Authentication::Headers(BTreeMap::from([(
                name.to_string(),
                SecretString::new(value),
            )])));
            let request = TransportRequest {
                repository,
                url: "http://127.0.0.1:9/resource".to_string(),
                kind: ResourceKind::Pom,
            };
            assert!(HttpTransport::default().get(&request).await.is_err());
        }

        let (url, server) = serve_once(
            b"HTTP/1.1 302 Found\r\nLocation: http://localhost:9/other\r\nContent-Length: 0\r\n\r\n",
        )
        .await;
        let mut repository = RemoteRepository::new("test", &url).expect("repository");
        repository.authentication = Some(Authentication::Headers(BTreeMap::from([(
            "X-Token".to_string(),
            SecretString::new("value"),
        )])));
        let request = TransportRequest {
            repository,
            url,
            kind: ResourceKind::Pom,
        };
        assert!(HttpTransport::default().get(&request).await.is_err());
        server.await.expect("server task").expect("request");
    }

    #[tokio::test]
    async fn applies_and_bypasses_http_proxies() {
        let (proxy_url, server) =
            serve_once(b"HTTP/1.1 200 OK\r\nContent-Length: 5\r\n\r\nproxy").await;
        let mut proxy = Proxy::new(&proxy_url).expect("proxy");
        proxy.authentication = Some(Authentication::Basic {
            username: "proxy".to_string(),
            password: SecretString::new("pass"),
        });
        let mut repository =
            RemoteRepository::new("test", "http://upstream.invalid/").expect("repository");
        repository.proxy = Some(proxy);
        let request = TransportRequest {
            repository,
            url: "http://upstream.invalid/resource".to_string(),
            kind: ResourceKind::Artifact,
        };
        let response = HttpTransport::default()
            .get(&request)
            .await
            .expect("proxy request")
            .expect("proxy response");
        assert_eq!(response.content_length, Some(5));
        let request_bytes = server.await.expect("server task").expect("proxy request");
        assert!(
            String::from_utf8_lossy(&request_bytes)
                .to_ascii_lowercase()
                .contains("proxy-authorization: basic")
        );

        let (url, server) = serve_once(b"HTTP/1.1 200 OK\r\nContent-Length: 6\r\n\r\ndirect").await;
        let mut repository = RemoteRepository::new("test", &url).expect("repository");
        let mut proxy = Proxy::new("http://127.0.0.1:9/").expect("proxy");
        proxy.non_proxy_hosts = vec!["127.*".to_string()];
        repository.proxy = Some(proxy);
        let request = TransportRequest {
            repository,
            url,
            kind: ResourceKind::Pom,
        };
        assert!(HttpTransport::default().get(&request).await.is_ok());
        server.await.expect("server task").expect("direct request");

        let mut repository =
            RemoteRepository::new("test", "http://upstream.invalid/").expect("repository");
        let mut proxy = Proxy::new("http://127.0.0.1:9/").expect("proxy");
        proxy.authentication = Some(Authentication::Bearer(SecretString::new("unsupported")));
        repository.proxy = Some(proxy);
        let request = TransportRequest {
            repository,
            url: "http://upstream.invalid/resource".to_string(),
            kind: ResourceKind::Pom,
        };
        assert!(HttpTransport::default().get(&request).await.is_err());

        for proxy_url in ["not a URL", "http://user:secret@127.0.0.1:9/"] {
            let mut proxy = Proxy::new("http://127.0.0.1:9/").expect("proxy");
            proxy.url = proxy_url.to_string();
            let mut repository =
                RemoteRepository::new("test", "http://upstream.invalid/").expect("repository");
            repository.proxy = Some(proxy);
            let request = TransportRequest {
                repository,
                url: "http://upstream.invalid/resource".to_string(),
                kind: ResourceKind::Pom,
            };
            assert!(HttpTransport::default().get(&request).await.is_err());
        }
        let mut invalid_proxy = Proxy::new("http://127.0.0.1:9/").expect("proxy");
        invalid_proxy.url = "not a URL".to_string();
        assert!(super::configured_proxy(&invalid_proxy, "http://upstream.invalid/").is_err());
    }

    async fn assert_file_transport_helpers(
        request: TransportRequest,
        directory: &tempfile::TempDir,
    ) {
        let missing = TransportRequest {
            repository: request.repository.clone(),
            url: Url::from_file_path(directory.path().join("missing"))
                .expect("missing URL")
                .to_string(),
            kind: ResourceKind::Pom,
        };
        assert!(
            FileTransport
                .get(&missing)
                .await
                .expect("missing")
                .is_none()
        );
        assert!(same_origin(
            &Url::parse("https://example.com/path").expect("left URL"),
            &Url::parse("https://example.com/other").expect("right URL")
        ));
        assert!(!same_origin(
            &Url::parse("https://example.com").expect("left URL"),
            &Url::parse("http://example.com").expect("right URL")
        ));
        assert!(proxy_applies(
            &Proxy::new("http://proxy.example").expect("proxy"),
            "not a URL"
        ));
        assert!(proxy_applies(
            &Proxy::new("http://proxy.example").expect("proxy"),
            "file:///tmp/artifact"
        ));
        let invalid_file_request = TransportRequest {
            repository: request.repository,
            url: "https://example.com/artifact".to_string(),
            kind: ResourceKind::Artifact,
        };
        assert!(FileTransport.get(&invalid_file_request).await.is_err());

        let hosted_repository =
            RemoteRepository::new("hosted-file", "file://example.com/repository/")
                .expect("hosted file repository");
        let hosted_request = TransportRequest {
            repository: hosted_repository,
            url: "file://example.com/repository/artifact.jar".to_string(),
            kind: ResourceKind::Artifact,
        };
        assert!(FileTransport.get(&hosted_request).await.is_err());
        assert!(
            super::file_url_path(
                &Url::parse("file://example.com/repository/").expect("hosted URL"),
                "file://example.com/repository/",
                "hosted file URL",
            )
            .is_err()
        );
        assert!(
            super::repository_file_path(
                &Url::parse("file://example.com/repository/").expect("hosted URL"),
                "file://example.com/repository/",
            )
            .is_err()
        );
        assert!(super::is_virtual_file_root(
            &Url::parse("file:///").expect("virtual file root")
        ));
        assert!(!super::is_virtual_file_root(
            &Url::parse("file:///repository/").expect("file repository")
        ));
    }

    #[tokio::test]
    async fn reads_file_transport_resources_and_helpers() {
        let directory = tempfile::TempDir::new().expect("temporary directory");
        let path = directory.path().join("artifact.jar");
        tokio::fs::write(&path, b"file")
            .await
            .expect("file fixture");
        let url = Url::from_file_path(&path).expect("file URL").to_string();
        let repository = RemoteRepository::new(
            "file",
            Url::from_directory_path(directory.path()).expect("URL"),
        )
        .expect("file repository");
        let request = TransportRequest {
            repository,
            url,
            kind: ResourceKind::Artifact,
        };
        let mut response = FileTransport
            .get(&request)
            .await
            .expect("file request")
            .expect("file response");
        assert_eq!(response.content_length, Some(4));
        assert_eq!(
            response
                .body
                .next()
                .await
                .expect("file chunk")
                .expect("file bytes"),
            bytes::Bytes::from_static(b"file")
        );
        assert!(FileTransport.supports("file"));
        assert!(!FileTransport.supports("https"));
        assert!(HttpTransport::default().supports("http"));
        assert!(!HttpTransport::default().supports("file"));
        assert_file_transport_helpers(request, &directory).await;
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn reports_file_transport_permission_errors() {
        use std::os::unix::fs::PermissionsExt;

        let directory = tempfile::TempDir::new().expect("temporary repository");
        let blocked = directory.path().join("blocked");
        tokio::fs::create_dir(&blocked)
            .await
            .expect("blocked directory");
        let artifact = blocked.join("artifact.jar");
        tokio::fs::write(&artifact, b"artifact")
            .await
            .expect("artifact fixture");
        let original_permissions = tokio::fs::metadata(&blocked)
            .await
            .expect("blocked metadata")
            .permissions();
        tokio::fs::set_permissions(&blocked, std::fs::Permissions::from_mode(0o000))
            .await
            .expect("block directory");

        let repository = RemoteRepository::new(
            "file",
            Url::from_directory_path(directory.path()).expect("repository URL"),
        )
        .expect("file repository");
        let request = TransportRequest {
            repository,
            url: Url::from_file_path(&artifact)
                .expect("artifact URL")
                .to_string(),
            kind: ResourceKind::Artifact,
        };
        let transport_error = FileTransport.get(&request).await;
        let response_error = super::file_response(artifact, false).await;

        tokio::fs::set_permissions(&blocked, original_permissions)
            .await
            .expect("restore blocked permissions");
        assert!(transport_error.is_err());
        assert!(response_error.is_err());

        let base_parent = directory.path().join("base-parent");
        let base = base_parent.join("repository");
        tokio::fs::create_dir_all(&base)
            .await
            .expect("nested repository");
        let nested_artifact = base.join("artifact.jar");
        tokio::fs::write(&nested_artifact, b"artifact")
            .await
            .expect("nested artifact");
        let base_permissions = tokio::fs::metadata(&base_parent)
            .await
            .expect("base parent metadata")
            .permissions();
        let repository = RemoteRepository::new(
            "blocked-base",
            Url::from_directory_path(&base).expect("blocked repository URL"),
        )
        .expect("blocked repository");
        let request = TransportRequest {
            repository,
            url: Url::from_file_path(&nested_artifact)
                .expect("nested artifact URL")
                .to_string(),
            kind: ResourceKind::Artifact,
        };
        tokio::fs::set_permissions(&base_parent, std::fs::Permissions::from_mode(0o000))
            .await
            .expect("block base parent");
        let base_error = FileTransport.get(&request).await;
        tokio::fs::set_permissions(&base_parent, base_permissions)
            .await
            .expect("restore base parent");
        assert!(base_error.is_err());
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn rejects_file_repository_symlinks_that_escape_the_root() {
        let repository_directory =
            tempfile::TempDir::new().expect("temporary repository directory");
        let outside_directory = tempfile::TempDir::new().expect("temporary outside directory");
        let outside = outside_directory.path().join("secret.jar");
        tokio::fs::write(&outside, b"secret")
            .await
            .expect("outside fixture");
        let link = repository_directory.path().join("artifact.jar");
        std::os::unix::fs::symlink(&outside, &link).expect("symlink fixture");

        let repository = RemoteRepository::new(
            "file",
            Url::from_directory_path(repository_directory.path()).expect("repository URL"),
        )
        .expect("file repository");
        let request = TransportRequest {
            url: Url::from_file_path(link).expect("resource URL").to_string(),
            repository,
            kind: ResourceKind::Artifact,
        };
        assert!(FileTransport.get(&request).await.is_err());
    }
}

/// Native filesystem repository transport.
#[cfg(not(target_family = "wasm"))]
#[derive(Debug, Clone, Default)]
pub struct FileTransport;

#[cfg(not(target_family = "wasm"))]
impl Transport for FileTransport {
    fn supports(&self, scheme: &str) -> bool {
        scheme == "file"
    }

    fn get<'a>(
        &'a self,
        request: &'a TransportRequest,
    ) -> BoxFuture<'a, Result<Option<TransportResponse>>> {
        Box::pin(async move {
            request.repository.validate_resource_url(&request.url)?;
            let url = url::Url::parse(&request.url)?;
            let path = file_url_path(
                &url,
                &request.url,
                "file URL could not be converted to a local path",
            )?;
            let base_url = url::Url::parse(&request.repository.url)?;
            let canonical_base = if is_virtual_file_root(&base_url) {
                None
            } else {
                let base_path = repository_file_path(&base_url, &request.repository.url)?;
                match tokio::fs::canonicalize(base_path).await {
                    Ok(path) => Some(path),
                    Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
                    Err(error) => return Err(error.into()),
                }
            };
            let canonical_path = match tokio::fs::canonicalize(path).await {
                Ok(path) => path,
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
                Err(error) => return Err(error.into()),
            };
            if canonical_base
                .as_ref()
                .is_some_and(|base| !canonical_path.starts_with(base))
            {
                return Err(Error::Transport {
                    url: request.url.clone(),
                    message: "file repository resource resolves outside its configured root"
                        .to_string(),
                });
            }
            file_response(canonical_path, false).await
        })
    }
}

#[cfg(not(target_family = "wasm"))]
fn is_virtual_file_root(url: &url::Url) -> bool {
    url.scheme() == "file" && url.host_str().is_none() && url.path() == "/"
}

#[cfg(not(target_family = "wasm"))]
fn file_url_path(url: &url::Url, error_url: &str, message: &str) -> Result<std::path::PathBuf> {
    if url.scheme() != "file" || !matches!(url.host_str(), None | Some("localhost")) {
        return Err(Error::Transport {
            url: error_url.to_string(),
            message: message.to_string(),
        });
    }
    url.to_file_path().map_err(|()| Error::Transport {
        url: error_url.to_string(),
        message: message.to_string(),
    })
}

#[cfg(not(target_family = "wasm"))]
fn repository_file_path(url: &url::Url, repository_url: &str) -> Result<std::path::PathBuf> {
    file_url_path(
        url,
        repository_url,
        "file repository URL could not be converted to a local path",
    )
}

#[cfg(not(target_family = "wasm"))]
pub(crate) async fn file_response(
    path: std::path::PathBuf,
    from_cache: bool,
) -> Result<Option<TransportResponse>> {
    use tokio::io::AsyncReadExt;

    let file = match tokio::fs::File::open(path).await {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error.into()),
    };
    let content_length = file.metadata().await.ok().map(|metadata| metadata.len());
    let body = futures_util::stream::try_unfold(file, |mut file| async move {
        let mut buffer = vec![0; 64 * 1024];
        let count = file.read(&mut buffer).await?;
        if count == 0 {
            Ok::<Option<(Bytes, tokio::fs::File)>, std::io::Error>(None)
        } else {
            buffer.truncate(count);
            Ok::<Option<(Bytes, tokio::fs::File)>, std::io::Error>(Some((
                Bytes::from(buffer),
                file,
            )))
        }
    })
    .map_err(Error::from);
    Ok(Some(TransportResponse {
        body: Box::pin(body),
        content_length,
        from_cache,
    }))
}
