//! Interactive prompts for the `list` command.

use std::path::Path;

use anyhow::Result;
#[cfg(feature = "interactive")]
use inquire::{Confirm, Select};

use crate::application::CardService;

/// Run interactive prompts for listing cards.
#[cfg(feature = "interactive")]
pub fn run_interactive_list(base_path: &Path) -> Result<()> {
    println!("📋 List cards\n");

    let service = CardService::new();
    let board = service.list(base_path)?;

    // Ask about filters
    let use_filters = Confirm::new("Would you like to apply filters?")
        .with_default(false)
        .prompt()?;

    let (column_filter, assignee_filter) = if use_filters {
        // Column filter
        let filter_by_column = Confirm::new("Filter by column?")
            .with_default(false)
            .prompt()?;

        let column_filter = if filter_by_column {
            let column_options: Vec<&str> = board.columns.iter().map(|c| c.id.as_str()).collect();
            let selected = Select::new("Select column:", column_options).prompt()?;
            Some(selected.to_string())
        } else {
            None
        };

        // Assignee filter
        let filter_by_assignee = Confirm::new("Filter by assignee?")
            .with_default(false)
            .prompt()?;

        let assignee_filter = if filter_by_assignee {
            // Get unique assignees from cards
            let mut assignees: Vec<String> = board
                .cards
                .iter()
                .filter_map(|c| c.assignee.clone())
                .collect();
            assignees.sort();
            assignees.dedup();

            if assignees.is_empty() {
                println!("No assignees found on any cards.");
                None
            } else {
                let selected = Select::new("Select assignee:", assignees).prompt()?;
                Some(selected)
            }
        } else {
            None
        };

        (column_filter, assignee_filter)
    } else {
        (None, None)
    };

    // Display results
    println!("\nBoard: {} ({})", board.name, board.id);
    println!("Total cards: {}\n", board.cards.len());

    for column in &board.columns {
        // Skip if column filter is specified and doesn't match
        if let Some(ref filter) = column_filter {
            if column.id != *filter {
                continue;
            }
        }

        let cards_in_column: Vec<_> = board
            .cards
            .iter()
            .filter(|c| {
                c.column_id == column.id
                    && assignee_filter
                        .as_ref()
                        .is_none_or(|a| c.assignee.as_ref() == Some(a))
            })
            .collect();

        println!("{} ({})", column.name, column.id);
        println!("{}", "─".repeat(column.name.len() + column.id.len() + 3));

        if cards_in_column.is_empty() {
            println!("  (no cards)\n");
        } else {
            for card in cards_in_column {
                let assignee_str = card
                    .assignee
                    .as_ref()
                    .map(|a| format!(" [@{}]", a))
                    .unwrap_or_default();
                println!("  {}: {}{}", card.id, card.title, assignee_str);
            }
            println!();
        }
    }

    Ok(())
}

/// Non-interactive fallback for when the feature is not enabled.
#[cfg(not(feature = "interactive"))]
pub fn run_interactive_list(_base_path: &Path) -> anyhow::Result<()> {
    Err(anyhow::anyhow!(
        "Interactive mode is not enabled. Build with --features interactive to use this feature."
    ))
}
