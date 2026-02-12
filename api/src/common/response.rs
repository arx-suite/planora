use actix_web::HttpResponse;

use super::ApiError;

type ApiResponse = Result<HttpResponse, ApiError>;

#[derive(serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct ApiResult<T>
where
    T: utoipa::ToSchema,
{
    pub success: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<T>,
}

impl<T> ApiResult<T>
where
    T: utoipa::ToSchema,
{
    pub fn success<M: Into<String>>(message: M, payload: T) -> Self {
        Self {
            success: true,
            message: message.into(),
            payload: Some(payload),
        }
    }

    pub fn to_ok_response(message: impl Into<String>, payload: T) -> ApiResponse
    where
        T: serde::Serialize,
    {
        Ok(HttpResponse::Ok().json(Self::success(message, payload)))
    }

    pub fn to_created_response(message: impl Into<String>, payload: T) -> ApiResponse
    where
        T: serde::Serialize,
    {
        Ok(HttpResponse::Created().json(Self::success(message, payload)))
    }
}

impl ApiResult<()> {
    pub fn ok<M: Into<String>>(message: M) -> Self {
        ApiResult {
            success: true,
            message: message.into(),
            payload: None,
        }
    }

    pub fn error<M: Into<String>>(message: M) -> ApiResult<()> {
        ApiResult {
            success: false,
            message: message.into(),
            payload: None,
        }
    }

    pub fn to_no_content(message: impl Into<String>) -> ApiResponse {
        let res = Self::ok(message);
        Ok(HttpResponse::NoContent().json(res))
    }

    pub fn to_bad_request(message: impl Into<String>) -> ApiResponse {
        Ok(HttpResponse::BadRequest().json(Self::error(message)))
    }

    pub fn to_unauthorized(message: impl Into<String>) -> ApiResponse {
        Ok(HttpResponse::Unauthorized().json(Self::error(message)))
    }

    pub fn to_not_found(message: impl Into<String>) -> ApiResponse {
        Ok(HttpResponse::NotFound().json(Self::error(message)))
    }

    pub fn to_internal_error(message: impl Into<String>) -> ApiResponse {
        Ok(HttpResponse::InternalServerError().json(Self::error(message)))
    }
}

#[cfg_attr(test, derive(serde::Deserialize))]
#[derive(serde::Serialize)]
pub struct PaginatedResult<T> {
    pub items: Vec<T>,
    pub total: Option<u64>,
    pub next_page: Option<String>,
    pub prev_page: Option<String>,
    pub page: Option<u64>,
    pub per_page: Option<u64>,
    pub total_pages: Option<u64>,
}

impl<T> PaginatedResult<T> {
    pub fn new(
        items: Vec<T>,
        total: Option<u64>,
        page: Option<u64>,
        per_page: Option<u64>,
        total_pages: Option<u64>,
        next_page: Option<String>,
        prev_page: Option<String>,
    ) -> Self {
        Self {
            items,
            total,
            page,
            per_page,
            total_pages,
            next_page,
            prev_page,
        }
    }
}

#[cfg_attr(test, derive(serde::Serialize))]
#[derive(serde::Deserialize)]
pub struct PaginationQuery {
    pub page: Option<u64>,
    pub per_page: Option<u64>,
    pub after: Option<String>,
}
