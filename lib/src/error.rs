use std::str::Utf8Error;

#[cfg(feature = "fetch")]
use http::uri::InvalidUri;
#[cfg(feature = "fetch")]
use hyper::StatusCode;
use thiserror::Error;

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
    #[error("cannot find script tag")]
    ScriptNotFound,
    #[error("failed to extract link generator")]
    LinkGeneratorExtractionFailure,
    #[error("failed to compute link key")]
    LinkComputationFailure,
    #[error("failed to extract domain name")]
    DomainExtractionFailure,
    #[error("failed to extract file id")]
    FileIdExtractionFailure,
    #[error("failed to extract file key")]
    FileKeyExtractionFailure,
    #[error("failed to extract filename")]
    FilenameExtractionFailure,
    #[error("filename is not valid UTF-8")]
    InvalidUtf8Filename { source: Utf8Error },
}
