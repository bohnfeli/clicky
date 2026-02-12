use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::{Card, Column};

/// Represents a kanban board containing columns and cards.
///
/// A board is the top-level container for a project's workflow.
/// Each directory can have one board stored in the .clicky folder.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Board {
    /// Unique board identifier (usually the project name)
    pub id: String,
    /// Board name
    pub name: String,
    /// Prefix for card IDs (e.g., "PRJ" for PRJ-001)
    pub card_id_prefix: String,
    /// Counter for generating next card ID
    pub next_card_number: u32,
    /// Columns in this board
    pub columns: Vec<Column>,
    /// All cards in this board
    pub cards: Vec<Card>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl Board {
    /// Creates a new board with default columns.
    ///
    /// # Arguments
    /// * `id` - Board identifier (e.g., "myproject")
    /// * `name` - Display name for the board
    ///
    /// # Example
    /// ```
    /// use clicky::domain::Board;
    ///
    /// let board = Board::new("myproject".to_string(), "My Project".to_string());
    /// assert_eq!(board.id, "myproject");
    /// assert_eq!(board.name, "My Project");
    /// assert_eq!(board.columns.len(), 3); // Default columns
    /// ```
    pub fn new(id: String, name: String) -> Self {
        let now = Utc::now();
        let prefix = Self::generate_prefix(&id);

        let default_columns = vec![
            Column::new("todo".to_string(), "To Do".to_string(), 0),
            Column::new("in_progress".to_string(), "In Progress".to_string(), 1),
            Column::new("done".to_string(), "Done".to_string(), 2),
        ];

        Self {
            id,
            name,
            card_id_prefix: prefix,
            next_card_number: 1,
            columns: default_columns,
            cards: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Generates a card ID prefix from the board ID.
    ///
    /// Takes up to 3 uppercase characters from the board ID.
    fn generate_prefix(board_id: &str) -> String {
        board_id
            .chars()
            .filter(|c| c.is_alphabetic())
            .take(3)
            .collect::<String>()
            .to_uppercase()
    }

    /// Generates the next unique card ID.
    ///
    /// Format: PREFIX-NNN (e.g., "PRJ-001")
    pub fn generate_card_id(&mut self) -> String {
        let id = format!("{}-{:03}", self.card_id_prefix, self.next_card_number);
        self.next_card_number += 1;
        self.updated_at = Utc::now();
        id
    }

    /// Adds a new card to the board.
    ///
    /// The card is placed in the specified column (or "todo" if not specified).
    /// Returns the generated card ID.
    ///
    /// # Arguments
    /// * `title` - Card title
    /// * `description` - Optional description
    /// * `assignee` - Optional assignee
    /// * `column_id` - Target column (defaults to "todo")
    pub fn create_card(
        &mut self,
        title: String,
        description: Option<String>,
        assignee: Option<String>,
        column_id: Option<String>,
    ) -> String {
        let card_id = self.generate_card_id();
        let target_column = column_id.unwrap_or_else(|| "todo".to_string());

        let mut card = Card::new(card_id.clone(), title, target_column.clone());
        card.description = description;
        card.assignee = assignee;

        // Add to column
        if let Some(column) = self.columns.iter_mut().find(|c| c.id == target_column) {
            column.add_card(card_id.clone());
        }

        self.cards.push(card);
        self.updated_at = Utc::now();

        card_id
    }

    /// Moves a card to a different column.
    ///
    /// Returns true if successful, false if card or column not found.
    pub fn move_card(&mut self, card_id: &str, target_column_id: &str) -> bool {
        // Check if target column exists
        if !self.columns.iter().any(|c| c.id == target_column_id) {
            return false;
        }

        // Find and update the card
        if let Some(card) = self.cards.iter_mut().find(|c| c.id == card_id) {
            let source_column_id = card.column_id.clone();

            // Remove from source column
            if let Some(column) = self.columns.iter_mut().find(|c| c.id == source_column_id) {
                column.remove_card(card_id);
            }

            // Add to target column
            if let Some(column) = self.columns.iter_mut().find(|c| c.id == target_column_id) {
                column.add_card(card_id.to_string());
            }

            // Update card
            card.move_to(target_column_id.to_string());
            self.updated_at = Utc::now();

            true
        } else {
            false
        }
    }

    /// Gets a card by ID.
    pub fn get_card(&self, card_id: &str) -> Option<&Card> {
        self.cards.iter().find(|c| c.id == card_id)
    }

    /// Gets a mutable reference to a card by ID.
    pub fn get_card_mut(&mut self, card_id: &str) -> Option<&mut Card> {
        self.cards.iter_mut().find(|c| c.id == card_id)
    }

    /// Deletes a card from the board.
    ///
    /// Returns true if the card was found and deleted.
    pub fn delete_card(&mut self, card_id: &str) -> bool {
        if let Some(pos) = self.cards.iter().position(|c| c.id == card_id) {
            let card = &self.cards[pos];

            // Remove from column
            if let Some(column) = self.columns.iter_mut().find(|c| c.id == card.column_id) {
                column.remove_card(card_id);
            }

            // Remove card
            self.cards.remove(pos);
            self.updated_at = Utc::now();

            true
        } else {
            false
        }
    }

    /// Adds a column to the board.
    #[allow(dead_code)]
    pub fn add_column(&mut self, id: String, name: String, order: u32) {
        let column = Column::new(id, name, order);
        self.columns.push(column);
        self.columns.sort_by_key(|c| c.order);
        self.updated_at = Utc::now();
    }

    /// Removes a column from the board.
    ///
    /// Cards in the column are moved to the first column.
    #[allow(dead_code)]
    pub fn remove_column(&mut self, column_id: &str) -> bool {
        if self.columns.len() <= 1 {
            return false; // Can't remove the last column
        }

        if let Some(pos) = self.columns.iter().position(|c| c.id == column_id) {
            let column = &self.columns[pos];
            let cards_to_move: Vec<String> = column.cards.clone();

            // Find first remaining column
            let first_column_id = self
                .columns
                .iter()
                .find(|c| c.id != column_id)
                .map(|c| c.id.clone())
                .unwrap();

            // Move cards
            for card_id in cards_to_move {
                self.move_card(&card_id, &first_column_id);
            }

            // Remove column
            self.columns.remove(pos);
            self.updated_at = Utc::now();

            true
        } else {
            false
        }
    }

    /// Gets cards in a specific column.
    pub fn get_cards_in_column(&self, column_id: &str) -> Vec<&Card> {
        self.cards
            .iter()
            .filter(|c| c.column_id == column_id)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_creation() {
        // Arrange
        let id = "myproject".to_string();
        let name = "My Project".to_string();

        // Act
        let board = Board::new(id.clone(), name.clone());

        // Assert
        assert_eq!(board.id, id);
        assert_eq!(board.name, name);
        assert_eq!(board.card_id_prefix, "MYP");
        assert_eq!(board.columns.len(), 3);
        assert_eq!(board.next_card_number, 1);
    }

    #[test]
    fn test_generate_prefix() {
        assert_eq!(Board::generate_prefix("myproject"), "MYP");
        assert_eq!(Board::generate_prefix("my-project"), "MYP");
        assert_eq!(Board::generate_prefix("my_project_123"), "MYP");
        assert_eq!(Board::generate_prefix("ab"), "AB");
    }

    #[test]
    fn test_create_card() {
        // Arrange
        let mut board = Board::new("test".to_string(), "Test".to_string());

        // Act
        let card_id = board.create_card(
            "Test Task".to_string(),
            Some("Description".to_string()),
            Some("Alice".to_string()),
            None,
        );

        // Assert
        assert_eq!(card_id, "TES-001");
        assert_eq!(board.cards.len(), 1);
        assert_eq!(board.next_card_number, 2);

        let card = board.get_card(&card_id).unwrap();
        assert_eq!(card.title, "Test Task");
        assert_eq!(card.description, Some("Description".to_string()));
        assert_eq!(card.assignee, Some("Alice".to_string()));

        let todo_column = board.columns.iter().find(|c| c.id == "todo").unwrap();
        assert!(todo_column.has_card(&card_id));
    }

    #[test]
    fn test_move_card() {
        // Arrange
        let mut board = Board::new("test".to_string(), "Test".to_string());
        let card_id = board.create_card("Task".to_string(), None, None, None);

        // Act
        let success = board.move_card(&card_id, "in_progress");

        // Assert
        assert!(success);
        let card = board.get_card(&card_id).unwrap();
        assert_eq!(card.column_id, "in_progress");

        let todo_column = board.columns.iter().find(|c| c.id == "todo").unwrap();
        assert!(!todo_column.has_card(&card_id));

        let progress_column = board
            .columns
            .iter()
            .find(|c| c.id == "in_progress")
            .unwrap();
        assert!(progress_column.has_card(&card_id));
    }

    #[test]
    fn test_delete_card() {
        // Arrange
        let mut board = Board::new("test".to_string(), "Test".to_string());
        let card_id = board.create_card("Task".to_string(), None, None, None);

        // Act
        let deleted = board.delete_card(&card_id);

        // Assert
        assert!(deleted);
        assert!(board.get_card(&card_id).is_none());

        let todo_column = board.columns.iter().find(|c| c.id == "todo").unwrap();
        assert!(!todo_column.has_card(&card_id));
    }
}
