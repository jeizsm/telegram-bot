//! IO backend.
//!
//! `CurlConnector` is default connector unless feature `curl_connector` is disabled and
//! feature `hyper_connector` is enabled. This behaviour will change after hyper release.

mod _base;
#[cfg(feature = "curl_connector")]
pub mod curl;
#[cfg(feature = "hyper_connector")]
pub mod hyper;
#[cfg(feature = "rustls_connector")]
pub mod rustls;

use tokio_core::reactor::Handle;

pub use self::_base::Connector;
#[cfg(feature = "curl_connector")]
pub use self::curl::CurlConnector;
#[cfg(feature = "hyper_connector")]
pub use self::hyper::HyperConnector;
#[cfg(feature = "rustls_connector")]
pub use self::rustls::RustlsConnector;


/// Returns default connector.
///
/// See module level documentation for details.
#[cfg(feature = "curl_connector")]
pub fn default_connector(handle: &Handle) -> Box<Connector> {
    curl::default_connector(handle)
}

/// Returns default connector.
///
/// See module level documentation for details.
#[cfg(all(not(feature = "curl_connector"), feature = "hyper_connector"))]
pub fn default_connector(handle: &Handle) -> Box<Connector> {
    hyper::default_connector(handle)
}

/// Returns default connector.
///
/// See module level documentation for details.
#[cfg(all(not(feature = "curl_connector"), not(feature = "hyper_connector"), feature = "rustls_connector"))]
pub fn default_connector(handle: &Handle) -> Box<Connector> {
    rustls::default_connector(handle)
}
