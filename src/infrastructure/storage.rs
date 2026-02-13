use std::fs;
use std::path::{Path, PathBuf};

use thiserror::Error;

use crate::domain::Board;

/// Errors that can occur during storage operations.
#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum StorageError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Board not found at {0}")]
    BoardNotFound(PathBuf),
    #[error("Board already exists at {0}")]
    BoardAlreadyExists(PathBuf),
    #[error("Invalid board data: {0}")]
    InvalidData(String),
}

/// Trait for board persistence operations.
///
/// This abstraction allows for different storage implementations
/// (JSON files, database, etc.) without changing the application logic.
pub trait BoardRepository {
    /// Loads a board from storage.
    ///
    /// # Arguments
    /// * `path` - Path to the board storage location
    ///
    /// # Errors
    /// Returns `StorageError::BoardNotFound` if no board exists at the path.
    fn load(&self, path: &Path) -> Result<Board, StorageError>;

    /// Saves a board to storage.
    ///
    /// # Arguments
    /// * `board` - The board to save
    /// * `path` - Path to the board storage location
    fn save(&self, board: &Board, path: &Path) -> Result<(), StorageError>;

    /// Checks if a board exists at the given path.
    fn exists(&self, path: &Path) -> bool;

    /// Deletes a board from storage.
    #[allow(dead_code)]
    fn delete(&self, path: &Path) -> Result<(), StorageError>;
}

/// JSON file-based implementation of BoardRepository.
///
/// Stores boards as JSON files in the filesystem.
/// Each board is stored in a separate file named `board.json`.
pub struct JsonBoardRepository;

impl JsonBoardRepository {
    /// Creates a new JSON board repository.
    pub fn new() -> Self {
        Self
    }

    /// Ensures the parent directory exists.
    fn ensure_directory(&self, path: &Path) -> Result<(), StorageError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        Ok(())
    }
}

impl Default for JsonBoardRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl BoardRepository for JsonBoardRepository {
    fn load(&self, path: &Path) -> Result<Board, StorageError> {
        if !path.exists() {
            return Err(StorageError::BoardNotFound(path.to_path_buf()));
        }

        let content = fs::read_to_string(path)?;
        let board: Board = serde_json::from_str(&content)?;
        Ok(board)
    }

    fn save(&self, board: &Board, path: &Path) -> Result<(), StorageError> {
        self.ensure_directory(path)?;

        let content = serde_json::to_string_pretty(board)?;
        fs::write(path, content)?;
        Ok(())
    }

    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }

    fn delete(&self, path: &Path) -> Result<(), StorageError> {
        if !path.exists() {
            return Err(StorageError::BoardNotFound(path.to_path_buf()));
        }

        fs::remove_file(path)?;
        Ok(())
    }
}

/// Utility functions for managing board storage paths.
pub struct BoardStorage;

impl BoardStorage {
    /// Default directory name for clicky data.
    pub const CLICKY_DIR: &'static str = ".clicky";
    /// Default filename for board data.
    pub const BOARD_FILE: &'static str = "board.json";

    /// Gets the path to the board file in the given directory.
    ///
    /// Returns: `<base_path>/.clicky/board.json`
    pub fn board_path(base_path: &Path) -> PathBuf {
        base_path.join(Self::CLICKY_DIR).join(Self::BOARD_FILE)
    }

    /// Gets the path to the clicky directory in the given directory.
    #[allow(dead_code)]
    pub fn clicky_dir(base_path: &Path) -> PathBuf {
        base_path.join(Self::CLICKY_DIR)
    }

    /// Finds the board path by searching upward from the current directory.
    ///
    /// Starts at `start_path` and goes up the directory tree until it finds
    /// a `.clicky/board.json` file or reaches the filesystem root.
    pub fn find_board_path(start_path: &Path) -> Option<PathBuf> {
        let mut current = start_path.to_path_buf();

        loop {
            let board_path = Self::board_path(&current);
            if board_path.exists() {
                return Some(board_path);
            }

            match current.parent() {
                Some(parent) => current = parent.to_path_buf(),
                None => break,
            }
        }

        None
    }
}

#[cfg(test)]
#[path = "./storage_tests.rs"]
mod storage_tests;
