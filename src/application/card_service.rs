use std::path::Path;

use crate::domain::Board;
use crate::infrastructure::storage::StorageError;

use super::{BoardService, BoardServiceError};

/// Service for card-related operations.
///
/// Provides high-level operations for creating, updating, moving,
/// and deleting cards within a board.
pub struct CardService {
    board_service: BoardService,
}

/// Errors that can occur during card operations.
#[derive(Debug, thiserror::Error)]
#[allow(dead_code)]
pub enum CardServiceError {
    #[error("Board service error: {0}")]
    BoardService(#[from] BoardServiceError),
    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),
    #[error("Card not found: {0}")]
    CardNotFound(String),
    #[error("Column not found: {0}")]
    ColumnNotFound(String),
    #[error("Invalid card data: {0}")]
    InvalidData(String),
}

/// Information about a created card.
pub struct CreatedCardInfo {
    pub card_id: String,
    pub board: Board,
}

impl CardService {
    /// Creates a new card service.
    pub fn new() -> Self {
        Self {
            board_service: BoardService::new(),
        }
    }

    /// Creates a new card in the board.
    ///
    /// # Arguments
    /// * `base_path` - Path to the board directory
    /// * `title` - Card title
    /// * `description` - Optional description
    /// * `assignee` - Optional assignee name
    /// * `column_id` - Optional target column (defaults to "todo")
    pub fn create(
        &self,
        base_path: &Path,
        title: String,
        description: Option<String>,
        assignee: Option<String>,
        column_id: Option<String>,
    ) -> Result<CreatedCardInfo, CardServiceError> {
        let mut board = self.board_service.load(base_path)?;

        // Validate column if specified
        if let Some(ref col_id) = column_id {
            if !board.columns.iter().any(|c| c.id == *col_id) {
                return Err(CardServiceError::ColumnNotFound(col_id.clone()));
            }
        }

        let card_id = board.create_card(title, description, assignee, column_id);

        self.board_service.save(&board, base_path)?;

        Ok(CreatedCardInfo { card_id, board })
    }

    /// Moves a card to a different column.
    ///
    /// # Arguments
    /// * `base_path` - Path to the board directory
    /// * `card_id` - ID of the card to move
    /// * `column_id` - Target column ID
    pub fn move_to(
        &self,
        base_path: &Path,
        card_id: &str,
        column_id: &str,
    ) -> Result<Board, CardServiceError> {
        let mut board = self.board_service.load(base_path)?;

        // Validate card exists
        if board.get_card(card_id).is_none() {
            return Err(CardServiceError::CardNotFound(card_id.to_string()));
        }

        // Validate column exists
        if !board.columns.iter().any(|c| c.id == column_id) {
            return Err(CardServiceError::ColumnNotFound(column_id.to_string()));
        }

        // Move card
        let success = board.move_card(card_id, column_id);
        if !success {
            return Err(CardServiceError::CardNotFound(card_id.to_string()));
        }

        self.board_service.save(&board, base_path)?;

        Ok(board)
    }

    /// Updates a card's details.
    ///
    /// # Arguments
    /// * `base_path` - Path to the board directory
    /// * `card_id` - ID of the card to update
    /// * `title` - Optional new title
    /// * `description` - Optional new description (None to clear)
    /// * `assignee` - Optional new assignee (None to clear)
    pub fn update(
        &self,
        base_path: &Path,
        card_id: &str,
        title: Option<String>,
        description: Option<Option<String>>,
        assignee: Option<Option<String>>,
    ) -> Result<Board, CardServiceError> {
        let mut board = self.board_service.load(base_path)?;

        let card = board
            .get_card_mut(card_id)
            .ok_or_else(|| CardServiceError::CardNotFound(card_id.to_string()))?;

        if let Some(new_title) = title {
            card.set_title(new_title);
        }

        if let Some(new_description) = description {
            card.set_description(new_description);
        }

        if let Some(new_assignee) = assignee {
            card.set_assignee(new_assignee);
        }

        self.board_service.save(&board, base_path)?;

        Ok(board)
    }

    /// Deletes a card from the board.
    ///
    /// # Arguments
    /// * `base_path` - Path to the board directory
    /// * `card_id` - ID of the card to delete
    pub fn delete(&self, base_path: &Path, card_id: &str) -> Result<Board, CardServiceError> {
        let mut board = self.board_service.load(base_path)?;

        let deleted = board.delete_card(card_id);
        if !deleted {
            return Err(CardServiceError::CardNotFound(card_id.to_string()));
        }

        self.board_service.save(&board, base_path)?;

        Ok(board)
    }

    /// Gets a card by ID.
    pub fn get(&self, base_path: &Path, card_id: &str) -> Result<Board, CardServiceError> {
        let board = self.board_service.load(base_path)?;

        if board.get_card(card_id).is_none() {
            return Err(CardServiceError::CardNotFound(card_id.to_string()));
        }

        Ok(board)
    }

    /// Lists all cards in the board.
    pub fn list(&self, base_path: &Path) -> Result<Board, CardServiceError> {
        Ok(self.board_service.load(base_path)?)
    }
}

impl Default for CardService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn setup_test_board() -> (TempDir, CardService) {
        let temp_dir = TempDir::new().unwrap();
        let board_service = BoardService::new();
        board_service
            .initialize(temp_dir.path(), Some("Test".to_string()))
            .unwrap();
        let card_service = CardService::new();
        (temp_dir, card_service)
    }

    #[test]
    fn test_create_card() {
        // Arrange
        let (temp_dir, service) = setup_test_board();

        // Act
        let result = service
            .create(
                temp_dir.path(),
                "Test Task".to_string(),
                Some("Description".to_string()),
                Some("Alice".to_string()),
                None,
            )
            .unwrap();

        // Assert
        assert_eq!(result.card_id, "TES-001");
        assert_eq!(result.board.cards.len(), 1);
    }

    #[test]
    fn test_create_card_invalid_column() {
        // Arrange
        let (temp_dir, service) = setup_test_board();

        // Act
        let result = service.create(
            temp_dir.path(),
            "Test".to_string(),
            None,
            None,
            Some("invalid".to_string()),
        );

        // Assert
        assert!(matches!(result, Err(CardServiceError::ColumnNotFound(_))));
    }

    #[test]
    fn test_move_card() {
        // Arrange
        let (temp_dir, service) = setup_test_board();
        let created = service
            .create(temp_dir.path(), "Task".to_string(), None, None, None)
            .unwrap();

        // Act
        let board = service
            .move_to(temp_dir.path(), &created.card_id, "in_progress")
            .unwrap();

        // Assert
        let card = board.get_card(&created.card_id).unwrap();
        assert_eq!(card.column_id, "in_progress");
    }

    #[test]
    fn test_move_card_not_found() {
        // Arrange
        let (temp_dir, service) = setup_test_board();

        // Act
        let result = service.move_to(temp_dir.path(), "TES-999", "done");

        // Assert
        assert!(matches!(result, Err(CardServiceError::CardNotFound(_))));
    }

    #[test]
    fn test_update_card() {
        // Arrange
        let (temp_dir, service) = setup_test_board();
        let created = service
            .create(temp_dir.path(), "Old Title".to_string(), None, None, None)
            .unwrap();

        // Act
        let board = service
            .update(
                temp_dir.path(),
                &created.card_id,
                Some("New Title".to_string()),
                Some(Some("New Desc".to_string())),
                Some(Some("Bob".to_string())),
            )
            .unwrap();

        // Assert
        let card = board.get_card(&created.card_id).unwrap();
        assert_eq!(card.title, "New Title");
        assert_eq!(card.description, Some("New Desc".to_string()));
        assert_eq!(card.assignee, Some("Bob".to_string()));
    }

    #[test]
    fn test_delete_card() {
        // Arrange
        let (temp_dir, service) = setup_test_board();
        let created = service
            .create(temp_dir.path(), "Task".to_string(), None, None, None)
            .unwrap();

        // Act
        let board = service.delete(temp_dir.path(), &created.card_id).unwrap();

        // Assert
        assert!(board.get_card(&created.card_id).is_none());
        assert_eq!(board.cards.len(), 0);
    }
}
