mod client;
pub mod client_impl;

#[cfg(feature = "host-reqwest")]
pub mod reqwest;

pub use self::client::*;

pub(crate) const USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "-", env!("CARGO_PKG_VERSION"));

/// Try to instantiate a HTTP client that is suitable for the current platform.
pub fn default_http_client() -> Option<impl HttpClient + Send + Sync + 'static> {
    cfg_if::cfg_if! {
        if #[cfg(feature = "host-reqwest")] {
            Some(self::reqwest::ReqwestHttpClient::default())
        } else {
            // Note: We need something to use with turbofish otherwise returning
            // a plain None will complain about not being able to infer the "T"
            // in Option<T>
            #[derive(Debug)]
            enum Unimplemented {}
            impl HttpClient for Unimplemented {
                fn request(&self, _request: HttpRequest) -> futures::future::BoxFuture<'_, Result<HttpResponse, anyhow::Error>> {
                    match *self {}
                }
            }

            Option::<Unimplemented>::None
        }
    }
}
