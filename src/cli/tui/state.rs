//! TUI state management.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppState {
    /// View the kanban board
    Board,
    /// View card details
    CardDetail,
    /// Create a new card
    CreateCard,
    /// Edit a card
    EditCard,
    /// Confirm deletion
    ConfirmDelete,
    /// Help overlay
    #[allow(dead_code)]
    Help,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    /// Focused on columns
    Columns,
    /// Focused on cards
    Cards,
    /// Focused on card details
    #[allow(dead_code)]
    CardDetails,
    /// Focused on input field
    #[allow(dead_code)]
    Input,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum InputMode {
    /// Normal navigation mode
    #[default]
    Normal,
    /// Editing text input
    Editing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FormField {
    Title,
    Description,
    Assignee,
}

#[derive(Debug, Clone, Default)]
pub struct CardFormData {
    pub title: String,
    pub description: String,
    pub assignee: String,
}
