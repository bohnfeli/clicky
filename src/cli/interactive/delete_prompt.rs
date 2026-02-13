//! Interactive prompts for the `delete` command.

use std::path::Path;

use anyhow::Result;
#[cfg(feature = "interactive")]
use inquire::{Confirm, Select};

use crate::application::CardService;

/// Run interactive prompts for deleting a card.
#[cfg(feature = "interactive")]
pub fn run_interactive_delete(base_path: &Path) -> Result<()> {
    println!("🗑️  Delete a card\n");

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

    let selected_card_str = Select::new("Select card to delete:", card_options).prompt()?;

    // Extract card ID from selection
    let card_id = selected_card_str
        .split(':')
        .next()
        .unwrap_or("")
        .to_string();

    let card = board
        .get_card(&card_id)
        .ok_or_else(|| anyhow::anyhow!("Card not found"))?;

    // Show card details
    println!("\nCard to delete:");
    println!("  ID: {}", card.id);
    println!("  Title: {}", card.title);
    if let Some(ref desc) = card.description {
        println!("  Description: {}", desc);
    }
    if let Some(ref assignee) = card.assignee {
        println!("  Assignee: {}", assignee);
    }

    // Confirm deletion
    let confirm = Confirm::new("\nAre you sure you want to delete this card?")
        .with_default(false)
        .prompt()?;

    if !confirm {
        println!("Cancelled.");
        return Ok(());
    }

    // Delete the card
    service.delete(base_path, &card_id)?;

    println!("\n✓ Deleted {}", card_id);

    Ok(())
}

/// Non-interactive fallback for when the feature is not enabled.
#[cfg(not(feature = "interactive"))]
pub fn run_interactive_delete(_base_path: &Path) -> anyhow::Result<()> {
    Err(anyhow::anyhow!(
        "Interactive mode is not enabled. Build with --features interactive to use this feature."
    ))
}
