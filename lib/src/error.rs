use std::str::{FromStr, Utf8Error};

use hyper::{StatusCode, Uri};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("invalid url")]
    InvalidUrl { source: <Uri as FromStr>::Err },
    #[error("failed to fetch page content")]
    ContentFetchingFailure { source: hyper::Error },
    #[error("invalid status code")]
    InvalidStatusCode { code: StatusCode },
    #[error("failed to follow redirection")]
    RedirectionFailure,
    #[error("failed to stream page content")]
    ContentStreamingFailure { source: hyper::Error },
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
