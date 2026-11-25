use crate::error::{AppError, AppResult};

/// Trimmed text up to `max` characters; empty is allowed.
pub fn text(value: &str, label: &str, max: usize) -> AppResult<String> {
    let value = value.trim();
    if value.chars().count() > max {
        return Err(AppError::Validation(format!(
            "{label} must be at most {max} characters"
        )));
    }
    Ok(value.to_string())
}

/// Like [`text`], but empty is rejected.
pub fn required(value: &str, label: &str, max: usize) -> AppResult<String> {
    let value = text(value, label, max)?;
    if value.is_empty() {
        return Err(AppError::Validation(format!("{label} must not be empty")));
    }
    Ok(value)
}
