use std::path::{Path, PathBuf};

use crate::domain::Board;
use crate::infrastructure::storage::{BoardStorage, JsonBoardRepository, StorageError};
use crate::infrastructure::BoardRepository;

/// Direction to move a card within its column.
pub enum MoveDirection {
    Up,
    Down,
}

/// Service for board-related operations.
///
/// This service provides high-level operations for creating, loading,
/// and managing kanban boards. It abstracts the storage details from
/// the CLI interface.
pub struct BoardService {
    repository: JsonBoardRepository,
}

/// Errors that can occur during board operations.
#[derive(Debug, thiserror::Error)]
#[allow(dead_code)]
pub enum BoardServiceError {
    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),
    #[error("Board already initialized in this directory")]
    AlreadyInitialized,
    #[error("No board found. Run 'clicky init' to create one.")]
    BoardNotFound,
    #[error("Invalid board name: {0}")]
    InvalidName(String),
}

impl BoardService {
    /// Creates a new board service.
    pub fn new() -> Self {
        Self {
            repository: JsonBoardRepository::new(),
        }
    }

    /// Initializes a new board in the specified directory.
    ///
    /// # Arguments
    /// * `base_path` - Directory where the board will be created
    /// * `name` - Name for the board (optional, defaults to directory name)
    ///
    /// # Errors
    /// Returns `BoardServiceError::AlreadyInitialized` if a board already exists.
    pub fn initialize(
        &self,
        base_path: &Path,
        name: Option<String>,
    ) -> Result<Board, BoardServiceError> {
        let board_path = BoardStorage::board_path(base_path);

        if self.repository.exists(&board_path) {
            return Err(BoardServiceError::AlreadyInitialized);
        }

        let board_name = name.unwrap_or_else(|| {
            base_path
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("board")
                .to_string()
        });

        let board_id = Self::sanitize_id(&board_name);
        let board = Board::new(board_id, board_name);

        self.repository.save(&board, &board_path)?;

        Ok(board)
    }

    /// Loads the board from the specified directory.
    ///
    /// # Arguments
    /// * `base_path` - Directory containing the board
    pub fn load(&self, base_path: &Path) -> Result<Board, BoardServiceError> {
        let board_path = BoardStorage::board_path(base_path);

        if !self.repository.exists(&board_path) {
            return Err(BoardServiceError::BoardNotFound);
        }

        Ok(self.repository.load(&board_path)?)
    }

    /// Finds and loads a board by searching upward from a path.
    ///
    /// Searches from `start_path` up the directory tree for a `.clicky` directory.
    #[allow(dead_code)]
    pub fn find_and_load(&self, start_path: &Path) -> Result<(Board, PathBuf), BoardServiceError> {
        match BoardStorage::find_board_path(start_path) {
            Some(board_path) => {
                let board = self.repository.load(&board_path)?;
                let base_path = board_path
                    .parent()
                    .and_then(|p| p.parent())
                    .unwrap_or(start_path)
                    .to_path_buf();
                Ok((board, base_path))
            }
            None => Err(BoardServiceError::BoardNotFound),
        }
    }

    /// Saves a board to the specified directory.
    pub fn save(&self, board: &Board, base_path: &Path) -> Result<(), BoardServiceError> {
        let board_path = BoardStorage::board_path(base_path);
        Ok(self.repository.save(board, &board_path)?)
    }

    /// Deletes the board from the specified directory.
    #[allow(dead_code)]
    pub fn delete(&self, base_path: &Path) -> Result<(), BoardServiceError> {
        let board_path = BoardStorage::board_path(base_path);
        Ok(self.repository.delete(&board_path)?)
    }

    /// Checks if a board exists in the specified directory.
    pub fn exists(&self, base_path: &Path) -> bool {
        let board_path = BoardStorage::board_path(base_path);
        self.repository.exists(&board_path)
    }

    /// Reorders a card within its column by moving it up or down.
    ///
    /// # Arguments
    /// * `base_path` - Directory containing the board
    /// * `card_id` - ID of the card to reorder
    /// * `direction` - Direction to move the card (Up or Down)
    ///
    /// # Errors
    /// Returns `BoardServiceError::BoardNotFound` if no board exists.
    /// Returns an error if the card is not found.
    pub fn reorder_card_in_column(
        &self,
        base_path: &Path,
        card_id: &str,
        direction: MoveDirection,
    ) -> Result<(), BoardServiceError> {
        let board_path = BoardStorage::board_path(base_path);

        if !self.repository.exists(&board_path) {
            return Err(BoardServiceError::BoardNotFound);
        }

        let mut board = self.repository.load(&board_path)?;

        let card = board.get_card(card_id).ok_or_else(|| {
            BoardServiceError::InvalidName(format!("Card not found: {}", card_id))
        })?;

        let card_column_id = card.column_id.clone();

        let moved = if let Some(column) = board.columns.iter_mut().find(|c| c.id == card_column_id)
        {
            match direction {
                MoveDirection::Up => column.move_card_up(card_id),
                MoveDirection::Down => column.move_card_down(card_id),
            }
        } else {
            false
        };

        if !moved {
            return Err(BoardServiceError::InvalidName(
                "Card could not be moved (already at edge)".to_string(),
            ));
        }

        board.updated_at = chrono::Utc::now();
        self.repository.save(&board, &board_path)?;

        Ok(())
    }

    /// Sanitizes a name to create a valid board ID.
    ///
    /// Converts to lowercase, replaces spaces with hyphens,
    /// and removes special characters.
    fn sanitize_id(name: &str) -> String {
        name.to_lowercase()
            .replace(" ", "-")
            .replace("_", "-")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect()
    }
}

impl Default for BoardService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[path = "./board_service_tests.rs"]
mod board_service_tests;
