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
#[path = "./column_tests.rs"]
mod column_tests;
