//! TUI state management.
//!
//! This module provides unified view state management for the TUI application,
//! replacing scattered state enums with a cohesive View hierarchy.

/// Main application view state.
///
/// Replaces the previous scattered state (AppState, Focus, InputMode, card_selected, etc.)
/// with a unified enum that captures the complete UI state in one place.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum View {
    /// Board view with card selection state
    Board {
        /// Currently selected column index
        selected_column: usize,
        /// Card selection state within the board
        selection: Selection,
    },
    /// Viewing card details
    CardDetail { card_id: String },
    /// Creating or editing a card
    CardForm { mode: FormMode },
    /// Moving card between columns
    MoveCard {
        card_id: String,
        target_column: usize,
    },
    /// Confirming deletion
    ConfirmDelete { card_id: String },
    /// Help overlay (stores previous view to return to)
    Help { previous: Box<View> },
}

impl Default for View {
    fn default() -> Self {
        View::Board {
            selected_column: 0,
            selection: Selection::None,
        }
    }
}

impl View {
    /// Get the currently selected card ID if any.
    pub fn selected_card_id(&self) -> Option<&str> {
        match self {
            View::Board {
                selection: Selection::Selected { card_id },
                ..
            } => Some(card_id),
            View::CardDetail { card_id } => Some(card_id),
            View::MoveCard { card_id, .. } => Some(card_id),
            View::ConfirmDelete { card_id } => Some(card_id),
            _ => None,
        }
    }

    /// Get the currently selected column index.
    pub fn selected_column(&self) -> usize {
        match self {
            View::Board {
                selected_column, ..
            } => *selected_column,
            _ => 0,
        }
    }

    /// Check if currently in board view with a card selected.
    pub fn is_card_selected(&self) -> bool {
        matches!(
            self,
            View::Board {
                selection: Selection::Selected { .. },
                ..
            }
        )
    }

    /// Check if currently in help view.
    pub fn is_help(&self) -> bool {
        matches!(self, View::Help { .. })
    }

    /// Toggle help overlay.
    /// Returns the new view state.
    pub fn toggle_help(self) -> Self {
        match self {
            View::Help { previous } => *previous,
            other => View::Help {
                previous: Box::new(other),
            },
        }
    }
}

/// Card selection state within the board view.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Selection {
    /// No card selected, browsing columns only
    #[default]
    None,
    /// Card is highlighted (pre-selected) but not confirmed
    Highlighted { column: usize, card_index: usize },
    /// Card is confirmed selected by ID
    Selected { card_id: String },
}

impl Selection {
    /// Get the card ID if selected.
    #[allow(dead_code)]
    pub fn card_id(&self) -> Option<&str> {
        match self {
            Selection::Selected { card_id } => Some(card_id),
            _ => None,
        }
    }

    /// Get the card index if highlighted.
    #[allow(dead_code)]
    pub fn card_index(&self) -> Option<usize> {
        match self {
            Selection::Highlighted { card_index, .. } => Some(*card_index),
            _ => None,
        }
    }

    /// Check if this selection is for given column.
    #[allow(dead_code)]
    pub fn is_in_column(&self, column: usize) -> bool {
        match self {
            Selection::Highlighted { column: c, .. } => *c == column,
            _ => true,
        }
    }
}

/// Form mode for card creation/editing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormMode {
    /// Creating a new card in a specific column
    Create { column_id: String },
    /// Editing an existing card
    Edit { card_id: String },
}

/// Form field selection for card forms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FormField {
    #[default]
    Title,
    Description,
    Assignee,
}

impl FormField {
    /// Move to the next field.
    pub fn next(self) -> Self {
        match self {
            FormField::Title => FormField::Description,
            FormField::Description => FormField::Assignee,
            FormField::Assignee => FormField::Title,
        }
    }

    /// Move to the previous field.
    pub fn prev(self) -> Self {
        match self {
            FormField::Title => FormField::Assignee,
            FormField::Description => FormField::Title,
            FormField::Assignee => FormField::Description,
        }
    }
}

/// Data for card creation/editing forms.
#[derive(Debug, Clone, Default)]
pub struct CardFormData {
    pub title: String,
    pub description: String,
    pub assignee: String,
    pub current_field: FormField,
    pub input_mode: InputMode,
}

impl CardFormData {
    /// Create form data for editing an existing card.
    pub fn for_edit(title: String, description: Option<String>, assignee: Option<String>) -> Self {
        Self {
            title,
            description: description.unwrap_or_default(),
            assignee: assignee.unwrap_or_default(),
            current_field: FormField::Title,
            input_mode: InputMode::Normal,
        }
    }

    /// Get mutable reference to current field's input.
    pub fn current_field_mut(&mut self) -> &mut String {
        match self.current_field {
            FormField::Title => &mut self.title,
            FormField::Description => &mut self.description,
            FormField::Assignee => &mut self.assignee,
        }
    }

    /// Check if the form has a valid title.
    pub fn is_valid(&self) -> bool {
        !self.title.trim().is_empty()
    }

    /// Convert to optional values for card creation/update.
    #[allow(clippy::wrong_self_convention)]
    pub fn to_card_values(self) -> (String, Option<String>, Option<String>) {
        let description = if self.description.trim().is_empty() {
            None
        } else {
            Some(self.description)
        };
        let assignee = if self.assignee.trim().is_empty() {
            None
        } else {
            Some(self.assignee)
        };
        (self.title, description, assignee)
    }
}

/// Input mode for form fields.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum InputMode {
    /// Normal navigation mode
    #[default]
    Normal,
    /// Actively editing text input
    Editing,
}
