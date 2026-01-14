pub mod constants;
pub mod cookie;
mod error;
pub mod extractors;
pub mod headers;
mod response;
pub mod time;
mod utils;

use actix_web::HttpResponse;

pub type ApiResponse = Result<HttpResponse, ApiError>;

pub use error::ApiError;
pub use response::{ApiResult, PaginatedResult, PaginationQuery};
pub use utils::env;
