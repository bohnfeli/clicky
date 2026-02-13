//! TUI application state.

use crate::application::BoardService;
use crate::cli::tui::state::{AppState, Focus, InputMode};
use crate::domain::Board;
use std::path::PathBuf;

/// Main TUI application.
pub struct App {
    /// Current application state (view mode)
    pub state: AppState,
    /// Current focus area
    pub focus: Focus,
    /// Input mode (normal or editing)
    pub input_mode: InputMode,
    /// Board path
    pub board_path: PathBuf,
    /// Board data
    pub board: Option<Board>,
    /// Currently selected column index
    pub selected_column: usize,
    /// Currently selected card index within column
    pub selected_card: Option<usize>,
    /// Current input string (for editing)
    pub input: String,
    /// Input cursor position
    pub cursor_position: usize,
    /// Error message to display
    pub error_message: Option<String>,
    /// Whether to show help overlay
    pub show_help: bool,
}

impl App {
    pub fn new(board_path: PathBuf) -> Self {
        Self {
            state: AppState::Board,
            focus: Focus::Columns,
            input_mode: InputMode::default(),
            board_path,
            board: None,
            selected_column: 0,
            selected_card: None,
            input: String::new(),
            cursor_position: 0,
            error_message: None,
            show_help: false,
        }
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
        if !self.show_help {
            self.clear_error();
        }
    }

    pub fn load_board(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let board_service = BoardService::new();
        if board_service.exists(&self.board_path) {
            let board = board_service.load(&self.board_path)?;
            self.board = Some(board);
            self.error_message = None;
        } else {
            self.error_message =
                Some("No board found. Run 'clicky init' to create one.".to_string());
        }
        Ok(())
    }

    pub fn selected_card_id(&self) -> Option<String> {
        if let (Some(board), Some(card_idx)) = (&self.board, self.selected_card) {
            let column = board.columns.get(self.selected_column)?;
            let cards_in_column: Vec<_> = board
                .cards
                .iter()
                .filter(|c| c.column_id == column.id)
                .collect();

            if let Some(card) = cards_in_column.get(card_idx) {
                return Some(card.id.clone());
            }
        }
        None
    }

    pub fn get_current_column(&self) -> Option<&str> {
        self.board
            .as_ref()
            .and_then(|b| b.columns.get(self.selected_column))
            .map(|c| c.id.as_str())
    }

    pub fn move_left(&mut self) {
        if self.selected_column > 0 {
            self.selected_column -= 1;
            self.selected_card = None;
            self.focus = Focus::Columns;
        }
    }

    pub fn move_right(&mut self) {
        if let Some(board) = &self.board {
            if self.selected_column < board.columns.len().saturating_sub(1) {
                self.selected_column += 1;
                self.selected_card = None;
                self.focus = Focus::Columns;
            }
        }
    }

    pub fn move_up(&mut self) {
        if let Some(card_idx) = self.selected_card {
            if card_idx > 0 {
                self.selected_card = Some(card_idx - 1);
            }
        } else if let Some(board) = &self.board {
            if let Some(column) = board.columns.get(self.selected_column) {
                let card_count = board
                    .cards
                    .iter()
                    .filter(|c| c.column_id == column.id)
                    .count();
                if card_count > 0 {
                    self.selected_card = Some(card_count - 1);
                    self.focus = Focus::Cards;
                }
            }
        }
    }

    pub fn move_down(&mut self) {
        if let Some(board) = &self.board {
            if let Some(column) = board.columns.get(self.selected_column) {
                let card_count = board
                    .cards
                    .iter()
                    .filter(|c| c.column_id == column.id)
                    .count();
                let current_idx = self.selected_card.unwrap_or(0);

                if current_idx < card_count.saturating_sub(1) {
                    self.selected_card = Some(current_idx + 1);
                    self.focus = Focus::Cards;
                }
            }
        }
    }

    pub fn enter_cards(&mut self) {
        self.focus = Focus::Cards;
        self.selected_card = Some(0);
    }

    pub fn exit_cards(&mut self) {
        self.focus = Focus::Columns;
        self.selected_card = None;
    }

    pub fn clear_error(&mut self) {
        self.error_message = None;
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new(PathBuf::from("."))
    }
}
