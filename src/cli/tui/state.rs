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
    Help,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Focus {
    /// Focused on columns
    Columns,
    /// Focused on cards
    Cards,
    /// Focused on card details
    CardDetails,
    /// Focused on input field
    Input,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InputMode {
    /// Normal navigation mode
    Normal,
    /// Editing text input
    Editing,
}

impl Default for InputMode {
    fn default() -> Self {
        Self::Normal
    }
}
