//! Interactive prompts for the `show` command.

use std::path::Path;

use anyhow::Result;
#[cfg(feature = "interactive")]
use inquire::Select;

use crate::application::CardService;

/// Run interactive prompts for showing card details.
#[cfg(feature = "interactive")]
pub fn run_interactive_show(base_path: &Path) -> Result<()> {
    println!("📄 Show card details\n");

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

    let selected_card_str = Select::new("Select card to view:", card_options).prompt()?;

    // Extract card ID from selection
    let card_id = selected_card_str
        .split(':')
        .next()
        .unwrap_or("")
        .to_string();

    // Get card details
    let board = service.get(base_path, &card_id)?;
    let card = board.get_card(&card_id).unwrap();

    let column = board
        .columns
        .iter()
        .find(|c| c.id == card.column_id)
        .unwrap();

    // Display card details
    println!("\nCard: {}", card.id);
    println!("  Title:       {}", card.title);
    if let Some(ref desc) = card.description {
        println!("  Description: {}", desc);
    }
    println!("  Column:      {} ({})", column.name, card.column_id);
    if let Some(ref assignee) = card.assignee {
        println!("  Assignee:    {}", assignee);
    }
    println!(
        "  Created:     {}",
        card.created_at.format("%Y-%m-%d %H:%M")
    );
    println!(
        "  Updated:     {}",
        card.updated_at.format("%Y-%m-%d %H:%M")
    );

    Ok(())
}

/// Non-interactive fallback for when the feature is not enabled.
#[cfg(not(feature = "interactive"))]
pub fn run_interactive_show(_base_path: &Path) -> anyhow::Result<()> {
    Err(anyhow::anyhow!(
        "Interactive mode is not enabled. Build with --features interactive to use this feature."
    ))
}
