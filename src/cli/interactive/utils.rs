//! Utility functions for interactive prompts.

#[cfg(feature = "interactive")]
use inquire::validator::Validation;

/// Validates that input is not empty.
#[cfg(feature = "interactive")]
pub fn non_empty_validator(input: &str) -> Result<Validation, inquire::InquireError> {
    if input.trim().is_empty() {
        Ok(Validation::Invalid("Input cannot be empty".into()))
    } else {
        Ok(Validation::Valid)
    }
}

/// Formats a card for display in selection lists.
pub fn format_card_option(card_id: &str, title: &str, column: Option<&str>) -> String {
    match column {
        Some(col) => format!("{} [{}]: {}", card_id, col, title),
        None => format!("{}: {}", card_id, title),
    }
}
