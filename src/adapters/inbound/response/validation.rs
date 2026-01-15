use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use validator::{ValidationErrors, ValidationError};

// REGEX
pub static USERNAME_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-zA-Z0-9_]+$").unwrap());

// FORMATTER
pub fn format_validation_errors(
    errors: ValidationErrors,
) -> HashMap<String, Vec<String>> {
    let mut map = HashMap::new();

    for (field, field_errors) in errors.field_errors() {
        let messages = field_errors
            .iter()
            .map(|e| map_error(field, e))
            .collect();

        map.insert(field.to_string(), messages);
    }

    map
}

fn map_error(field: &str, e: &ValidationError) -> String {
    match e.code.as_ref() {
        "required" => format!("{} is required", field),
        "length" => {
            match (e.params.get("min"), e.params.get("max")) {
                (Some(min), Some(max)) =>
                    format!("{} must be between {} and {} characters", field, min, max),
                (Some(min), None) if min == &serde_json::json!(1) =>
                    format!("{} is required", field),
                (Some(min), None) =>
                    format!("{} must be at least {} characters", field, min),
                (None, Some(max)) =>
                    format!("{} must be at most {} characters", field, max),
                _ =>
                    format!("{} length is invalid", field),
            }
        }
        "range" => {
            match (e.params.get("min"), e.params.get("max")) {
                (Some(min), Some(max)) =>
                    format!("{} must be between {} and {}", field, min, max),
                (Some(min), None) =>
                    format!("{} must be greater than or equal to {}", field, min),
                (None, Some(max)) =>
                    format!("{} must be less than or equal to {}", field, max),
                _ =>
                    format!("{} range is invalid", field),
            }
        }
        "email" => format!("{} must be a valid email address", field),
        "regex" => format!("{} contains invalid characters", field),
        _ => format!("{} is invalid", field),
    }
}
