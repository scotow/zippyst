#[cfg(feature = "fetch")]
use std::str::Utf8Error;

use hyper::http::uri::InvalidUri;
#[cfg(feature = "fetch")]
use hyper::StatusCode;
use thiserror::Error;
use tinyexpr::error::TinyExprError;

#[derive(Error, Debug)]
pub enum Error {
    #[cfg(feature = "fetch")]
    #[error("invalid url")]
    InvalidUrl { source: InvalidUri },
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
    InvalidUtf8PageContent { source: Utf8Error },
    #[error("invalid CSS selector")]
    InvalidCssSelector,
    #[error("cannot find script tag")]
    ScriptNotFound,
    #[error("failed to extract variable")]
    VariableExtractionFailure,
    #[error("failed to compute variable")]
    VariableComputationFailure { source: TinyExprError },
    #[error("failed to extract link generator")]
    LinkGeneratorExtractionFailure,
    #[error("failed to compute link key")]
    LinkComputationFailure { source: TinyExprError },
    #[error("failed to extract domain name")]
    DomainExtractionFailure,
    #[error("failed to extract file id")]
    FileIdExtractionFailure,
    #[error("failed to extract filename")]
    FilenameExtractionFailure,
    #[error("filename is not valid UTF-8")]
    InvalidUtf8Filename { source: Utf8Error },
}
