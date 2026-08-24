use tracing::warn;

use crate::api_error::ApiError;

pub fn sanitize_path_segment(input: &str) -> Result<String, ApiError> {
    let trimmed = input.trim();

    if trimmed.is_empty()
        || trimmed.contains('/')
        || trimmed.contains('\\')
        || trimmed.contains("..")
    {
        warn!(value = %input, "rejected unsafe path segment");
        return Err(ApiError::BadRequest("invalid request".into()));
    }

    Ok(trimmed.to_string())
}



