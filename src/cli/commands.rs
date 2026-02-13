use std::path::PathBuf;

use clap::{Parser, Subcommand};

/// Clicky - A CLI kanban board for human-agent collaboration
#[derive(Parser)]
#[command(name = "clicky")]
#[command(about = "A CLI kanban board for human-agent collaboration")]
#[command(version)]
pub struct Cli {
    /// Optional path to the board directory (defaults to current directory)
    #[arg(short, long, global = true)]
    pub path: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

/// Available CLI commands
#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new kanban board in the current directory
    Init {
        /// Name for the board (defaults to directory name)
        #[arg(short, long)]
        name: Option<String>,

        /// Use interactive mode
        #[arg(short, long)]
        interactive: bool,
    },

    /// Create a new card
    Create {
        /// Card title
        #[arg(required_unless_present = "interactive")]
        title: Option<String>,

        /// Card description
        #[arg(short, long)]
        description: Option<String>,

        /// Assignee name
        #[arg(short, long)]
        assignee: Option<String>,

        /// Column to place the card in (defaults to "todo")
        #[arg(short, long)]
        column: Option<String>,

        /// Use interactive mode
        #[arg(short, long)]
        interactive: bool,
    },

    /// Move a card to a different column
    Move {
        /// Card ID (e.g., PRJ-001)
        #[arg(required_unless_present = "interactive")]
        card_id: Option<String>,

        /// Target column ID (e.g., "in_progress", "done")
        #[arg(required_unless_present = "interactive")]
        column: Option<String>,

        /// Use interactive mode
        #[arg(short, long)]
        interactive: bool,
    },

    /// Show details of a specific card
    Show {
        /// Card ID (e.g., PRJ-001)
        #[arg(required_unless_present = "interactive")]
        card_id: Option<String>,

        /// Use interactive mode
        #[arg(short, long)]
        interactive: bool,
    },

    /// List all cards in the board
    List {
        /// Filter by column ID
        #[arg(short, long)]
        column: Option<String>,

        /// Filter by assignee
        #[arg(short, long)]
        assignee: Option<String>,

        /// Use interactive mode
        #[arg(short, long)]
        interactive: bool,
    },

    /// Update a card's details
    Update {
        /// Card ID (e.g., PRJ-001)
        #[arg(required_unless_present = "interactive")]
        card_id: Option<String>,

        /// New title
        #[arg(short, long)]
        title: Option<String>,

        /// New description
        #[arg(short, long)]
        description: Option<String>,

        /// Clear the description
        #[arg(long, conflicts_with = "description")]
        clear_description: bool,

        /// New assignee
        #[arg(short, long)]
        assignee: Option<String>,

        /// Clear the assignee
        #[arg(long, conflicts_with = "assignee")]
        clear_assignee: bool,

        /// Use interactive mode
        #[arg(short, long)]
        interactive: bool,
    },

    /// Delete a card from the board
    Delete {
        /// Card ID (e.g., PRJ-001)
        #[arg(required_unless_present = "interactive")]
        card_id: Option<String>,

        /// Skip confirmation prompt
        #[arg(short, long)]
        force: bool,

        /// Use interactive mode
        #[arg(short, long)]
        interactive: bool,
    },

    /// Show board information
    Info,

    /// Launch interactive TUI mode
    #[cfg(feature = "tui")]
    Tui,
}
