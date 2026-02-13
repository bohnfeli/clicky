//! Interactive prompts for the `init` command.

use std::path::Path;

use anyhow::Result;
#[cfg(feature = "interactive")]
use inquire::{Confirm, Select, Text};

use crate::application::BoardService;

/// Run interactive prompts for initializing a new board.
#[cfg(feature = "interactive")]
pub fn run_interactive_init(base_path: &Path) -> Result<()> {
    println!("🏗️  Let's initialize a new kanban board!\n");

    let service = BoardService::new();

    // Check if board already exists
    if service.exists(base_path) {
        return Err(anyhow::anyhow!(
            "Board already initialized in this directory. Use 'clicky info' to view it."
        ));
    }

    // Get board name
    let default_name = base_path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("board");

    let board_name = Text::new("Board name:")
        .with_default(default_name)
        .with_placeholder("e.g., My Awesome Project")
        .prompt()?;

    // Column customization
    let customize_columns = Confirm::new("Would you like to customize the columns?")
        .with_default(false)
        .prompt()?;

    let board = if customize_columns {
        let column_presets = vec![
            "Default (To Do, In Progress, Done)",
            "Simple (To Do, Done)",
            "Development (Backlog, In Progress, Review, Done)",
            "Custom",
        ];

        let preset = Select::new("Choose a column preset:", column_presets).prompt()?;

        match preset {
            "Default (To Do, In Progress, Done)" => {
                service.initialize(base_path, Some(board_name))?
            }
            "Simple (To Do, Done)" => {
                let mut board = service.initialize(base_path, Some(board_name))?;
                // Remove in_progress column
                board.remove_column("in_progress");
                service.save(&board, base_path)?;
                board
            }
            "Development (Backlog, In Progress, Review, Done)" => {
                let mut board = service.initialize(base_path, Some(board_name))?;
                // Add backlog and review columns
                board.add_column("backlog".to_string(), "Backlog".to_string(), 0);
                board.add_column("review".to_string(), "Review".to_string(), 2);
                board.remove_column("todo");
                board.columns.sort_by_key(|c| c.order);
                service.save(&board, base_path)?;
                board
            }
            "Custom" => {
                // For custom, we'll just use defaults for now
                // Custom column creation can be added in a future iteration
                println!(
                    "Using default columns for now. You can edit .clicky/board.json to customize."
                );
                service.initialize(base_path, Some(board_name))?
            }
            _ => unreachable!(),
        }
    } else {
        service.initialize(base_path, Some(board_name))?
    };

    println!(
        "\n✓ Initialized board '{}' in {}",
        board.name,
        base_path.display()
    );
    println!("  Card ID prefix: {}", board.card_id_prefix);
    let column_names: Vec<&str> = board.columns.iter().map(|c| c.name.as_str()).collect();
    println!("  Columns: {}", column_names.join(", "));

    Ok(())
}

/// Non-interactive fallback for when the feature is not enabled.
#[cfg(not(feature = "interactive"))]
pub fn run_interactive_init(_base_path: &Path) -> anyhow::Result<()> {
    Err(anyhow::anyhow!(
        "Interactive mode is not enabled. Build with --features interactive to use this feature."
    ))
}
