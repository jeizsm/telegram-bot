use telegram_bot_raw;

error_chain! {
    foreign_links {
        Url(::hyper::error::UriError) #[cfg(any(feature = "hyper_connector", feature = "rustls_connector"))];
        Hyper(::hyper::Error) #[cfg(any(feature = "hyper_connector", feature = "rustls_connector"))];
        Curl(::curl::Error) #[cfg(feature = "curl_connector")];
        CurlPerformError(::tokio_curl::PerformError) #[cfg(feature = "curl_connector")];
        Io(::std::io::Error);
    }

    links {
        Raw(telegram_bot_raw::Error, telegram_bot_raw::ErrorKind);
    }
}
