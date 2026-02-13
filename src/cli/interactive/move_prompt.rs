//! Interactive prompts for the `move` command.

use std::path::Path;

use anyhow::Result;
#[cfg(feature = "interactive")]
use inquire::Select;

use crate::application::CardService;

/// Run interactive prompts for moving a card.
#[cfg(feature = "interactive")]
pub fn run_interactive_move(base_path: &Path) -> Result<()> {
    println!("🔄 Move a card\n");

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

    let selected_card_str = Select::new("Select card to move:", card_options).prompt()?;

    // Extract card ID from selection
    let card_id = selected_card_str
        .split(':')
        .next()
        .unwrap_or("")
        .to_string();

    let card = board
        .get_card(&card_id)
        .ok_or_else(|| anyhow::anyhow!("Card not found"))?;

    let current_column = board
        .columns
        .iter()
        .find(|c| c.id == card.column_id)
        .map(|c| c.name.clone())
        .unwrap_or_else(|| card.column_id.clone());

    println!("  Currently in: {}\n", current_column);

    // Select target column
    let column_options: Vec<&str> = board
        .columns
        .iter()
        .filter(|c| c.id != card.column_id)
        .map(|c| c.id.as_str())
        .collect();

    if column_options.is_empty() {
        println!("No other columns to move to.");
        return Ok(());
    }

    let target_column = Select::new("Move to column:", column_options).prompt()?;

    // Move the card
    let updated_board = service.move_to(base_path, &card_id, target_column)?;

    let updated_card = updated_board.get_card(&card_id).unwrap();
    let column_name = updated_board
        .columns
        .iter()
        .find(|c| c.id == target_column)
        .map(|c| c.name.as_str())
        .unwrap_or(target_column);

    println!("\n✓ Moved {} to {}", card_id, column_name);
    println!("  Title: {}", updated_card.title);

    Ok(())
}

/// Non-interactive fallback for when the feature is not enabled.
#[cfg(not(feature = "interactive"))]
pub fn run_interactive_move(_base_path: &Path) -> anyhow::Result<()> {
    Err(anyhow::anyhow!(
        "Interactive mode is not enabled. Build with --features interactive to use this feature."
    ))
}
