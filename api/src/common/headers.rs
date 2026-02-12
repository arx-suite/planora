use actix_web::HttpRequest;
use std::str::FromStr;

use super::ApiError;

pub fn extract_header<T: FromStr>(req: &HttpRequest, name: &'static str) -> Result<T, ApiError> {
    let value = req
        .headers()
        .get(name)
        .ok_or_else(|| ApiError::BadRequest(format!("Missing header: {name}")))?
        .to_str()
        .map_err(|_| ApiError::BadRequest(format!("Invalid header encoding: {name}")))?;

    T::from_str(value).map_err(|_| ApiError::BadRequest(format!("Invalid header value: {name}")))
}
