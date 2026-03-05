use crate::utils::{AppError, Result};
use validator::Validate;

pub fn validate_request<T: Validate>(data: &T) -> Result<()> {
    data.validate()
        .map_err(|e| AppError::Validation(format!("Validation failed: {}", e)))
}