//! TLS configuration for HTTP clients.

/// Creates a Reqwest client after installing the selected Rustls crypto provider.
pub(crate) fn reqwest_client() -> reqwest::Result<reqwest::Client> {
    #[cfg(feature = "tls-rustls-ring")]
    {
        use std::sync::Once;

        static INIT: Once = Once::new();
        INIT.call_once(|| {
            drop(rustls::crypto::ring::default_provider().install_default());
        });
    }

    #[cfg(any(feature = "tls-rustls-aws-lc-rs", feature = "tls-rustls-ring"))]
    {
        reqwest::Client::builder().tls_backend_rustls().build()
    }

    #[cfg(not(any(feature = "tls-rustls-aws-lc-rs", feature = "tls-rustls-ring")))]
    Ok(reqwest::Client::new())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_reqwest_client() -> reqwest::Result<()> {
        drop(super::reqwest_client()?);
        Ok(())
    }
}
