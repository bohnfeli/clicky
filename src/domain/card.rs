use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Represents a task card in the kanban board.
///
/// Cards are the primary unit of work in Clicky. Each card has a unique ID,
/// title, optional description, and can be assigned to someone.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Card {
    /// Unique identifier (e.g., "PRJ-001")
    pub id: String,
    /// Card title
    pub title: String,
    /// Optional detailed description
    pub description: Option<String>,
    /// Current column/status
    pub column_id: String,
    /// Optional assignee name
    pub assignee: Option<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last update timestamp
    pub updated_at: DateTime<Utc>,
}

impl Card {
    /// Creates a new card with the given ID and title.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for the card
    /// * `title` - The card title
    /// * `column_id` - Initial column for the card
    ///
    /// # Example
    /// ```
    /// use clicky::domain::Card;
    ///
    /// let card = Card::new("PRJ-001".to_string(), "Implement login".to_string(), "todo".to_string());
    /// assert_eq!(card.id, "PRJ-001");
    /// assert_eq!(card.title, "Implement login");
    /// ```
    pub fn new(id: String, title: String, column_id: String) -> Self {
        let now = Utc::now();
        Self {
            id,
            title,
            description: None,
            column_id,
            assignee: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Moves the card to a different column.
    ///
    /// # Arguments
    /// * `column_id` - The target column ID
    pub fn move_to(&mut self, column_id: String) {
        self.column_id = column_id;
        self.updated_at = Utc::now();
    }

    /// Updates the card's title.
    pub fn set_title(&mut self, title: String) {
        self.title = title;
        self.updated_at = Utc::now();
    }

    /// Updates the card's description.
    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
        self.updated_at = Utc::now();
    }

    /// Updates the card's assignee.
    pub fn set_assignee(&mut self, assignee: Option<String>) {
        self.assignee = assignee;
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
#[path = "./card_tests.rs"]
mod card_tests;
