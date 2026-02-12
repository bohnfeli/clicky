use std::env;
use std::path::{Path, PathBuf};
use std::process;

use clap::Parser;

mod application;
mod cli;
mod domain;
mod infrastructure;

use application::{BoardService, CardService};
use cli::{Cli, Commands};
use infrastructure::storage::BoardStorage;

/// Exit codes for the application
#[allow(dead_code)]
mod exit_code {
    pub const SUCCESS: i32 = 0;
    pub const GENERAL_ERROR: i32 = 1;
    pub const BOARD_NOT_FOUND: i32 = 2;
    pub const INVALID_INPUT: i32 = 3;
}

fn main() {
    let cli = Cli::parse();

    // Determine the base path
    let base_path = cli
        .path
        .unwrap_or_else(|| env::current_dir().unwrap_or_else(|_| PathBuf::from(".")));

    // Execute the command
    let result = match cli.command {
        Commands::Init { name } => cmd_init(&base_path, name),
        Commands::Create {
            title,
            description,
            assignee,
            column,
        } => cmd_create(&base_path, title, description, assignee, column),
        Commands::Move { card_id, column } => cmd_move(&base_path, &card_id, &column),
        Commands::Show { card_id } => cmd_show(&base_path, &card_id),
        Commands::List { column, assignee } => cmd_list(&base_path, column, assignee),
        Commands::Update {
            card_id,
            title,
            description,
            clear_description,
            assignee,
            clear_assignee,
        } => cmd_update(
            &base_path,
            &card_id,
            title,
            description,
            clear_description,
            assignee,
            clear_assignee,
        ),
        Commands::Delete { card_id, force } => cmd_delete(&base_path, &card_id, force),
        Commands::Info => cmd_info(&base_path),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        process::exit(exit_code::GENERAL_ERROR);
    }
}

fn cmd_init(base_path: &Path, name: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    let service = BoardService::new();

    if service.exists(base_path) {
        return Err(
            "Board already initialized in this directory. Use 'clicky info' to view it.".into(),
        );
    }

    let board = service.initialize(base_path, name)?;

    println!(
        "✓ Initialized board '{}' in {}",
        board.name,
        base_path.display()
    );
    println!("  Card ID prefix: {}", board.card_id_prefix);
    let column_names: Vec<&str> = board.columns.iter().map(|c| c.name.as_str()).collect();
    println!("  Columns: {}", column_names.join(", "));

    Ok(())
}

fn cmd_create(
    base_path: &Path,
    title: String,
    description: Option<String>,
    assignee: Option<String>,
    column: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let service = CardService::new();

    let result = service.create(base_path, title, description, assignee, column)?;

    println!("✓ Created card {}", result.card_id);
    println!(
        "  Title: {}",
        result.board.get_card(&result.card_id).unwrap().title
    );

    Ok(())
}

fn cmd_move(
    base_path: &Path,
    card_id: &str,
    column: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let service = CardService::new();

    let board = service.move_to(base_path, card_id, column)?;
    let card = board.get_card(card_id).unwrap();
    let column_name = board
        .columns
        .iter()
        .find(|c| c.id == column)
        .map(|c| c.name.as_str())
        .unwrap_or(column);

    println!("✓ Moved {} to {}", card_id, column_name);
    println!("  Title: {}", card.title);

    Ok(())
}

fn cmd_show(base_path: &Path, card_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let service = CardService::new();

    let board = service.get(base_path, card_id)?;
    let card = board.get_card(card_id).unwrap();
    let column = board
        .columns
        .iter()
        .find(|c| c.id == card.column_id)
        .unwrap();

    println!("Card: {}", card.id);
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

fn cmd_list(
    base_path: &Path,
    column_filter: Option<String>,
    assignee_filter: Option<String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let service = CardService::new();

    let board = service.list(base_path)?;

    println!("Board: {} ({})", board.name, board.id);
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
                        .map_or(true, |a| c.assignee.as_ref() == Some(a))
            })
            .collect();

        println!("\n{} ({})", column.name, column.id);
        println!("{}", "─".repeat(column.name.len() + column.id.len() + 3));

        if cards_in_column.is_empty() {
            println!("  (no cards)");
        } else {
            for card in cards_in_column {
                let assignee_str = card
                    .assignee
                    .as_ref()
                    .map(|a| format!(" [@{}]", a))
                    .unwrap_or_default();
                println!("  {}: {}{}", card.id, card.title, assignee_str);
            }
        }
    }

    Ok(())
}

fn cmd_update(
    base_path: &Path,
    card_id: &str,
    title: Option<String>,
    description: Option<String>,
    clear_description: bool,
    assignee: Option<String>,
    clear_assignee: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let service = CardService::new();

    let desc_update = if clear_description {
        Some(None)
    } else {
        description.map(Some)
    };

    let assignee_update = if clear_assignee {
        Some(None)
    } else {
        assignee.map(Some)
    };

    let board = service.update(base_path, card_id, title, desc_update, assignee_update)?;
    let card = board.get_card(card_id).unwrap();

    println!("✓ Updated {}", card_id);
    println!("  Title: {}", card.title);

    Ok(())
}

fn cmd_delete(
    base_path: &Path,
    card_id: &str,
    force: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    if !force {
        print!("Are you sure you want to delete {}? [y/N] ", card_id);
        use std::io::Write;
        std::io::stdout().flush()?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        if !input.trim().eq_ignore_ascii_case("y") {
            println!("Cancelled.");
            return Ok(());
        }
    }

    let service = CardService::new();
    service.delete(base_path, card_id)?;

    println!("✓ Deleted {}", card_id);

    Ok(())
}

fn cmd_info(base_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let board_service = BoardService::new();

    if !board_service.exists(base_path) {
        // Try to find a board in parent directories
        match BoardStorage::find_board_path(base_path) {
            Some(path) => {
                let found_base = path.parent().unwrap().parent().unwrap();
                println!("Board found in parent directory: {}", found_base.display());
                println!(
                    "Run 'clicky --path {} info' to view it.",
                    found_base.display()
                );
                return Ok(());
            }
            None => {
                return Err("No board found. Run 'clicky init' to create one.".into());
            }
        }
    }

    let board = board_service.load(base_path)?;

    println!("Board: {}", board.name);
    println!("ID: {}", board.id);
    println!("Card ID prefix: {}", board.card_id_prefix);
    println!("Created: {}", board.created_at.format("%Y-%m-%d %H:%M"));
    println!("\nColumns:");

    for column in &board.columns {
        let card_count = board.get_cards_in_column(&column.id).len();
        println!("  {} ({}): {} cards", column.name, column.id, card_count);
    }

    println!("\nTotal cards: {}", board.cards.len());

    Ok(())
}
