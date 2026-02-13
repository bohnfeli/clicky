//! Interactive prompts for the `create` command.

use std::path::Path;

use anyhow::Result;
#[cfg(feature = "interactive")]
use inquire::{required, Select, Text};

use crate::application::CardService;

/// Run interactive prompts for creating a new card.
#[cfg(feature = "interactive")]
pub fn run_interactive_create(base_path: &Path) -> Result<()> {
    println!("🎫 Create a new card\n");

    let service = CardService::new();

    // Load board to get available columns
    let board = service.list(base_path)?;

    // Get card title
    let title = Text::new("Card title:")
        .with_validator(required!("Title is required"))
        .with_placeholder("What needs to be done?")
        .prompt()?;

    // Get description (optional)
    let description = Text::new("Description (optional):")
        .with_placeholder("Add more details...")
        .prompt()?;

    let description = if description.trim().is_empty() {
        None
    } else {
        Some(description)
    };

    // Get assignee (optional)
    let assignee = Text::new("Assignee (optional):")
        .with_placeholder("Who is working on this?")
        .prompt()?;

    let assignee = if assignee.trim().is_empty() {
        None
    } else {
        Some(assignee)
    };

    // Select column
    let column_options: Vec<&str> = board.columns.iter().map(|c| c.id.as_str()).collect();
    let selected_column = Select::new("Column:", column_options)
        .with_starting_cursor(0)
        .prompt()?;

    // Create the card
    let result = service.create(
        base_path,
        title,
        description,
        assignee,
        Some(selected_column.to_string()),
    )?;

    println!("\n✓ Created card {}", result.card_id);
    println!(
        "  Title: {}",
        result.board.get_card(&result.card_id).unwrap().title
    );

    Ok(())
}

/// Non-interactive fallback for when the feature is not enabled.
#[cfg(not(feature = "interactive"))]
pub fn run_interactive_create(_base_path: &Path) -> anyhow::Result<()> {
    Err(anyhow::anyhow!(
        "Interactive mode is not enabled. Build with --features interactive to use this feature."
    ))
}
