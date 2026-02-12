use serde::{Deserialize, Serialize};

/// Represents a column in the kanban board.
///
/// Columns categorize cards by their status or workflow stage.
/// Common examples: "To Do", "In Progress", "Done"
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Column {
    /// Unique identifier for the column (e.g., "todo", "in_progress")
    pub id: String,
    /// Display name for the column
    pub name: String,
    /// Order position for display (lower = left)
    pub order: u32,
    /// IDs of cards in this column, in order
    pub cards: Vec<String>,
}

impl Column {
    /// Creates a new column with the given ID and name.
    ///
    /// # Arguments
    /// * `id` - Unique identifier (usually lowercase, no spaces)
    /// * `name` - Human-readable display name
    /// * `order` - Position in the board (0 = leftmost)
    ///
    /// # Example
    /// ```
    /// use clicky::domain::Column;
    ///
    /// let column = Column::new("todo".to_string(), "To Do".to_string(), 0);
    /// assert_eq!(column.id, "todo");
    /// assert_eq!(column.name, "To Do");
    /// assert!(column.cards.is_empty());
    /// ```
    pub fn new(id: String, name: String, order: u32) -> Self {
        Self {
            id,
            name,
            order,
            cards: Vec::new(),
        }
    }

    /// Adds a card to this column.
    ///
    /// The card is added at the end of the column by default.
    pub fn add_card(&mut self, card_id: String) {
        if !self.cards.contains(&card_id) {
            self.cards.push(card_id);
        }
    }

    /// Removes a card from this column.
    ///
    /// Returns true if the card was found and removed, false otherwise.
    pub fn remove_card(&mut self, card_id: &str) -> bool {
        if let Some(pos) = self.cards.iter().position(|id| id == card_id) {
            self.cards.remove(pos);
            true
        } else {
            false
        }
    }

    /// Checks if a card is in this column.
    #[allow(dead_code)]
    pub fn has_card(&self, card_id: &str) -> bool {
        self.cards.contains(&card_id.to_string())
    }

    /// Returns the number of cards in this column.
    #[allow(dead_code)]
    pub fn card_count(&self) -> usize {
        self.cards.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_column_creation() {
        // Arrange
        let id = "todo".to_string();
        let name = "To Do".to_string();

        // Act
        let column = Column::new(id.clone(), name.clone(), 0);

        // Assert
        assert_eq!(column.id, id);
        assert_eq!(column.name, name);
        assert_eq!(column.order, 0);
        assert!(column.cards.is_empty());
    }

    #[test]
    fn test_add_card() {
        // Arrange
        let mut column = Column::new("todo".to_string(), "To Do".to_string(), 0);

        // Act
        column.add_card("PRJ-001".to_string());
        column.add_card("PRJ-002".to_string());

        // Assert
        assert_eq!(column.cards.len(), 2);
        assert!(column.has_card("PRJ-001"));
        assert!(column.has_card("PRJ-002"));
    }

    #[test]
    fn test_add_duplicate_card() {
        // Arrange
        let mut column = Column::new("todo".to_string(), "To Do".to_string(), 0);

        // Act
        column.add_card("PRJ-001".to_string());
        column.add_card("PRJ-001".to_string()); // Duplicate

        // Assert
        assert_eq!(column.cards.len(), 1);
    }

    #[test]
    fn test_remove_card() {
        // Arrange
        let mut column = Column::new("todo".to_string(), "To Do".to_string(), 0);
        column.add_card("PRJ-001".to_string());
        column.add_card("PRJ-002".to_string());

        // Act
        let removed = column.remove_card("PRJ-001");

        // Assert
        assert!(removed);
        assert_eq!(column.cards.len(), 1);
        assert!(!column.has_card("PRJ-001"));
    }

    #[test]
    fn test_remove_nonexistent_card() {
        // Arrange
        let mut column = Column::new("todo".to_string(), "To Do".to_string(), 0);
        column.add_card("PRJ-001".to_string());

        // Act
        let removed = column.remove_card("PRJ-999");

        // Assert
        assert!(!removed);
        assert_eq!(column.cards.len(), 1);
    }
}
