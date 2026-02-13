//! Interactive prompts for the `update` command.

use std::path::Path;

use anyhow::Result;
#[cfg(feature = "interactive")]
use inquire::{Confirm, Select, Text};

use crate::application::CardService;

/// Run interactive prompts for updating a card.
#[cfg(feature = "interactive")]
pub fn run_interactive_update(base_path: &Path) -> Result<()> {
    println!("✏️  Update a card\n");

    let service = CardService::new();

    // Load board
    let board = service.list(base_path)?;

    // If no cards, exit early
    if board.cards.is_empty() {
        println!("No cards found on this board.");
        return Ok(());
    }

    // Select card
    let card_options: Vec<String> = board
        .cards
        .iter()
        .map(|c| format!("{}: {}", c.id, c.title))
        .collect();

    let selected_card_str = Select::new("Select card to update:", card_options).prompt()?;

    // Extract card ID from selection
    let card_id = selected_card_str
        .split(':')
        .next()
        .unwrap_or("")
        .to_string();

    let card = board
        .get_card(&card_id)
        .ok_or_else(|| anyhow::anyhow!("Card not found"))?;

    println!("\nSelected: {}\n", card.title);

    // Select which fields to update
    let mut title: Option<String> = None;
    let mut description: Option<Option<String>> = None;
    let mut assignee: Option<Option<String>> = None;

    let update_title = Confirm::new("Update title?").with_default(false).prompt()?;
    if update_title {
        let new_title = Text::new("New title:").with_default(&card.title).prompt()?;
        title = Some(new_title);
    }

    let update_description = Confirm::new("Update description?")
        .with_default(false)
        .prompt()?;
    if update_description {
        let current_desc = card.description.as_deref().unwrap_or("");
        let has_description = !current_desc.is_empty();

        if has_description {
            let clear_desc = Confirm::new("Clear description?")
                .with_default(false)
                .prompt()?;
            if clear_desc {
                description = Some(None);
            } else {
                let new_desc = Text::new("New description:")
                    .with_default(current_desc)
                    .prompt()?;
                description = Some(Some(new_desc));
            }
        } else {
            let new_desc = Text::new("Add description:").prompt()?;
            if !new_desc.trim().is_empty() {
                description = Some(Some(new_desc));
            }
        }
    }

    let update_assignee = Confirm::new("Update assignee?")
        .with_default(false)
        .prompt()?;
    if update_assignee {
        let current_assignee = card.assignee.as_deref().unwrap_or("");
        let has_assignee = !current_assignee.is_empty();

        if has_assignee {
            let clear_assignee = Confirm::new("Clear assignee?")
                .with_default(false)
                .prompt()?;
            if clear_assignee {
                assignee = Some(None);
            } else {
                let new_assignee = Text::new("New assignee:")
                    .with_default(current_assignee)
                    .prompt()?;
                assignee = Some(Some(new_assignee));
            }
        } else {
            let new_assignee = Text::new("Add assignee:").prompt()?;
            if !new_assignee.trim().is_empty() {
                assignee = Some(Some(new_assignee));
            }
        }
    }

    // Check if any changes were made
    if title.is_none() && description.is_none() && assignee.is_none() {
        println!("No changes made.");
        return Ok(());
    }

    // Update the card
    let updated_board = service.update(base_path, &card_id, title, description, assignee)?;

    let updated_card = updated_board.get_card(&card_id).unwrap();

    println!("\n✓ Updated {}", card_id);
    println!("  Title: {}", updated_card.title);

    Ok(())
}

/// Non-interactive fallback for when the feature is not enabled.
#[cfg(not(feature = "interactive"))]
pub fn run_interactive_update(_base_path: &Path) -> anyhow::Result<()> {
    Err(anyhow::anyhow!(
        "Interactive mode is not enabled. Build with --features interactive to use this feature."
    ))
}
