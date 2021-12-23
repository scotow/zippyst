use std::str::{FromStr, Utf8Error};

#[cfg(feature = "fetch")]
use hyper::{StatusCode, Uri};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[cfg(feature = "fetch")]
    #[error("invalid url")]
    InvalidUrl { source: <Uri as FromStr>::Err },
    #[cfg(feature = "fetch")]
    #[error("failed to fetch page content")]
    ContentFetchingFailure { source: hyper::Error },
    #[cfg(feature = "fetch")]
    #[error("invalid status code")]
    InvalidStatusCode { code: StatusCode },
    #[cfg(feature = "fetch")]
    #[error("failed to follow redirection")]
    RedirectionFailure,
    #[cfg(feature = "fetch")]
    #[error("failed to stream page content")]
    ContentStreamingFailure { source: hyper::Error },
    #[cfg(feature = "fetch")]
    #[error("page content is not valid UTF-8")]
    InvalidUtf8Content { source: Utf8Error },
    #[error("invalid CSS selector")]
    InvalidCssSelector,
    #[error("cannot find script tag")]
    ScriptNotFound,
    #[error("failed to extract variable")]
    VariableExtractionFailure,
    #[error("failed to compute variable")]
    VariableComputationFailure {
        source: tinyexpr::error::TinyExprError,
    },
    #[error("failed to extract link generator")]
    LinkGeneratorExtractionFailure,
    #[error("failed to compute link key")]
    LinkComputationFailure {
        source: tinyexpr::error::TinyExprError,
    },
}
