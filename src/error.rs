use reqwest::header::InvalidHeaderValue;
use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Request(#[from] reqwest::Error),
    #[error(transparent)]
    Url(#[from] url::ParseError),
    #[error("Invalid Authorization header value: {_0}")]
    InvalidAuthorizationHeader(InvalidHeaderValue),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Deserialize, Debug)]
pub struct ApiResponseMeta {}

#[derive(Deserialize, Debug)]
pub struct ApiResponse<T> {
    pub data: T,
    pub meta: Option<ApiResponseMeta>,
}

pub type ApiResult<T> = Result<ApiResponse<T>>;