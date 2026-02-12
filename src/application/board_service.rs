use std::path::{Path, PathBuf};

use crate::domain::Board;
use crate::infrastructure::storage::{BoardStorage, JsonBoardRepository, StorageError};
use crate::infrastructure::BoardRepository;

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
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_initialize_board() {
        // Arrange
        let temp_dir = TempDir::new().unwrap();
        let service = BoardService::new();

        // Act
        let board = service
            .initialize(temp_dir.path(), Some("My Project".to_string()))
            .unwrap();

        // Assert
        assert_eq!(board.name, "My Project");
        assert_eq!(board.id, "my-project");
        assert_eq!(board.card_id_prefix, "MYP");
        assert!(service.exists(temp_dir.path()));
    }

    #[test]
    fn test_initialize_board_default_name() {
        // Arrange
        let temp_dir = TempDir::new().unwrap();
        let service = BoardService::new();

        // Act
        let board = service.initialize(temp_dir.path(), None).unwrap();

        // Assert - should use directory name
        assert!(!board.name.is_empty());
    }

    #[test]
    fn test_initialize_already_exists() {
        // Arrange
        let temp_dir = TempDir::new().unwrap();
        let service = BoardService::new();
        service.initialize(temp_dir.path(), None).unwrap();

        // Act
        let result = service.initialize(temp_dir.path(), None);

        // Assert
        assert!(matches!(result, Err(BoardServiceError::AlreadyInitialized)));
    }

    #[test]
    fn test_load_board() {
        // Arrange
        let temp_dir = TempDir::new().unwrap();
        let service = BoardService::new();
        let created = service
            .initialize(temp_dir.path(), Some("Test".to_string()))
            .unwrap();

        // Act
        let loaded = service.load(temp_dir.path()).unwrap();

        // Assert
        assert_eq!(created.id, loaded.id);
        assert_eq!(created.name, loaded.name);
    }

    #[test]
    fn test_load_not_found() {
        // Arrange
        let temp_dir = TempDir::new().unwrap();
        let service = BoardService::new();

        // Act
        let result = service.load(temp_dir.path());

        // Assert
        assert!(matches!(result, Err(BoardServiceError::BoardNotFound)));
    }

    #[test]
    fn test_sanitize_id() {
        assert_eq!(BoardService::sanitize_id("My Project"), "my-project");
        assert_eq!(BoardService::sanitize_id("My_Project"), "my-project");
        assert_eq!(
            BoardService::sanitize_id("My Project 123"),
            "my-project-123"
        );
        assert_eq!(BoardService::sanitize_id("My@Project!"), "myproject");
    }
}
