extern crate antidote;
#[macro_use]
extern crate error_chain;
extern crate futures;
extern crate tokio_core;
extern crate telegram_bot_raw;

#[cfg(feature = "curl_connector")]
extern crate curl;
#[cfg(feature = "curl_connector")]
extern crate tokio_curl;

#[cfg(any(feature = "hyper_connector", feature = "rustls_connector"))]
extern crate hyper;
#[cfg(feature = "hyper_connector")]
extern crate hyper_tls;

#[cfg(feature = "rustls_connector")]
extern crate hyper_rustls;

mod api;
mod errors;
mod future;
mod stream;

pub mod connector;
pub mod prelude;
pub mod types;

pub use self::api::{Api, Config};
pub use connector::*;
pub use self::errors::{Error, ErrorKind};
pub use self::future::TelegramFuture;
pub use stream::UpdatesStream;
pub use prelude::*;
pub use types::*;
