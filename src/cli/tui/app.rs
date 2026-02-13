//! TUI application state.

use crate::application::{BoardService, CardService};
use crate::cli::tui::state::{AppState, CardFormData, Focus, FormField, InputMode};
use crate::domain::Board;
use std::path::PathBuf;

/// Main TUI application.
pub struct App {
    /// Current application state (view mode)
    pub state: AppState,
    /// Current focus area
    pub focus: Focus,
    /// Input mode (normal or editing)
    pub input_mode: InputMode,
    /// Board path
    pub board_path: PathBuf,
    /// Board data
    pub board: Option<Board>,
    /// Currently selected column index
    pub selected_column: usize,
    /// Currently selected card index within column
    pub selected_card: Option<usize>,
    /// Currently selected target column for moving cards
    pub selected_target_column: usize,
    /// Current input string (for editing)
    #[allow(dead_code)]
    pub input: String,
    /// Input cursor position
    #[allow(dead_code)]
    pub cursor_position: usize,
    /// Error message to display
    pub error_message: Option<String>,
    /// Whether to show help overlay
    pub show_help: bool,
    /// Current form field being edited
    pub form_field: FormField,
    /// Form data for card creation/editing
    pub form_data: CardFormData,
    /// Card ID being edited (None for create mode)
    pub editing_card_id: Option<String>,
}

impl App {
    pub fn new(board_path: PathBuf) -> Self {
        Self {
            state: AppState::Board,
            focus: Focus::Columns,
            input_mode: InputMode::default(),
            board_path,
            board: None,
            selected_column: 0,
            selected_card: None,
            selected_target_column: 0,
            input: String::new(),
            cursor_position: 0,
            error_message: None,
            show_help: false,
            form_field: FormField::Title,
            form_data: CardFormData::default(),
            editing_card_id: None,
        }
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
        if !self.show_help {
            self.clear_error();
        }
    }

    pub fn load_board(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let board_service = BoardService::new();
        if board_service.exists(&self.board_path) {
            let board = board_service.load(&self.board_path)?;
            self.board = Some(board);
            self.error_message = None;
        } else {
            self.error_message =
                Some("No board found. Run 'clicky init' to create one.".to_string());
        }
        Ok(())
    }

    pub fn selected_card_id(&self) -> Option<String> {
        if let (Some(board), Some(card_idx)) = (&self.board, self.selected_card) {
            let column = board.columns.get(self.selected_column)?;
            let cards_in_column: Vec<_> = board
                .cards
                .iter()
                .filter(|c| c.column_id == column.id)
                .collect();

            if let Some(card) = cards_in_column.get(card_idx) {
                return Some(card.id.clone());
            }
        }
        None
    }

    pub fn get_current_column(&self) -> Option<&str> {
        self.board
            .as_ref()
            .and_then(|b| b.columns.get(self.selected_column))
            .map(|c| c.id.as_str())
    }

    pub fn move_left(&mut self) {
        if self.selected_column > 0 {
            self.selected_column -= 1;
            self.selected_card = None;
            self.focus = Focus::Columns;
        }
    }

    pub fn move_right(&mut self) {
        if let Some(board) = &self.board {
            if self.selected_column < board.columns.len().saturating_sub(1) {
                self.selected_column += 1;
                self.selected_card = None;
                self.focus = Focus::Columns;
            }
        }
    }

    pub fn move_up(&mut self) {
        if let Some(card_idx) = self.selected_card {
            if card_idx > 0 {
                self.selected_card = Some(card_idx - 1);
            }
        } else if let Some(board) = &self.board {
            if let Some(column) = board.columns.get(self.selected_column) {
                let card_count = board
                    .cards
                    .iter()
                    .filter(|c| c.column_id == column.id)
                    .count();
                if card_count > 0 {
                    self.selected_card = Some(card_count - 1);
                    self.focus = Focus::Cards;
                }
            }
        }
    }

    pub fn move_down(&mut self) {
        if let Some(board) = &self.board {
            if let Some(column) = board.columns.get(self.selected_column) {
                let card_count = board
                    .cards
                    .iter()
                    .filter(|c| c.column_id == column.id)
                    .count();
                let current_idx = self.selected_card.unwrap_or(0);

                if current_idx < card_count.saturating_sub(1) {
                    self.selected_card = Some(current_idx + 1);
                    self.focus = Focus::Cards;
                }
            }
        }
    }

    pub fn enter_cards(&mut self) {
        self.focus = Focus::Cards;
        self.selected_card = Some(0);
    }

    pub fn enter_card_detail(&mut self) {
        if self.focus == Focus::Cards && self.selected_card.is_some() {
            self.state = AppState::CardDetail;
        }
    }

    pub fn exit_cards(&mut self) {
        self.focus = Focus::Columns;
        self.selected_card = None;
    }

    pub fn clear_error(&mut self) {
        self.error_message = None;
    }

    pub fn start_create_card(&mut self) {
        self.clear_form();
        self.state = AppState::CreateCard;
        self.form_field = FormField::Title;
        self.input_mode = InputMode::Normal;
    }

    pub fn next_form_field(&mut self) {
        self.form_field = match self.form_field {
            FormField::Title => FormField::Description,
            FormField::Description => FormField::Assignee,
            FormField::Assignee => FormField::Title,
        };
    }

    pub fn prev_form_field(&mut self) {
        self.form_field = match self.form_field {
            FormField::Title => FormField::Assignee,
            FormField::Description => FormField::Title,
            FormField::Assignee => FormField::Description,
        };
    }

    pub fn get_current_field_input_mut(&mut self) -> &mut String {
        match self.form_field {
            FormField::Title => &mut self.form_data.title,
            FormField::Description => &mut self.form_data.description,
            FormField::Assignee => &mut self.form_data.assignee,
        }
    }

    pub fn submit_card(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let title = self.form_data.title.trim();
        if title.is_empty() {
            self.error_message = Some("Title is required".to_string());
            return Ok(());
        }

        let description = if self.form_data.description.trim().is_empty() {
            None
        } else {
            Some(self.form_data.description.clone())
        };

        let assignee = if self.form_data.assignee.trim().is_empty() {
            None
        } else {
            Some(self.form_data.assignee.clone())
        };

        let column_id = self.get_current_column().unwrap_or("todo").to_string();

        let card_service = CardService::new();
        card_service.create(
            &self.board_path,
            title.to_string(),
            description,
            assignee,
            Some(column_id),
        )?;

        self.load_board()?;
        self.clear_form();
        self.state = AppState::Board;
        Ok(())
    }

    pub fn cancel_card(&mut self) {
        self.clear_form();
        self.state = AppState::Board;
    }

    pub fn start_move_card(&mut self) {
        if let Some(card_id) = self.selected_card_id() {
            self.state = AppState::MoveCard;
            if let Some(board) = &self.board {
                if let Some(card) = board.get_card(&card_id) {
                    self.selected_target_column = board
                        .columns
                        .iter()
                        .position(|c| c.id == card.column_id)
                        .unwrap_or(0);
                }
            }
            self.clear_error();
        }
    }

    pub fn move_card_left(&mut self) {
        if self.selected_target_column > 0 {
            self.selected_target_column -= 1;
        }
    }

    pub fn move_card_right(&mut self) {
        if let Some(board) = &self.board {
            if self.selected_target_column < board.columns.len().saturating_sub(1) {
                self.selected_target_column += 1;
            }
        }
    }

    pub fn confirm_move_card(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let card_id = self.selected_card_id().ok_or("No card selected")?;

        let target_column_id = self
            .board
            .as_ref()
            .and_then(|b| b.columns.get(self.selected_target_column))
            .map(|c| c.id.clone())
            .ok_or("Invalid target column")?;

        let card_service = CardService::new();
        card_service.move_to(&self.board_path, &card_id, &target_column_id)?;

        self.load_board()?;
        self.state = AppState::CardDetail;
        self.clear_error();

        Ok(())
    }

    pub fn cancel_move_card(&mut self) {
        self.state = AppState::CardDetail;
        self.clear_error();
    }

    fn clear_form(&mut self) {
        self.form_data = CardFormData::default();
        self.form_field = FormField::Title;
        self.input_mode = InputMode::Normal;
        self.editing_card_id = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::application::BoardService;
    use tempfile::TempDir;

    #[test]
    fn test_form_navigation() {
        let mut app = App::default();

        app.start_create_card();
        assert_eq!(app.form_field, FormField::Title);

        app.next_form_field();
        assert_eq!(app.form_field, FormField::Description);

        app.next_form_field();
        assert_eq!(app.form_field, FormField::Assignee);

        app.next_form_field();
        assert_eq!(app.form_field, FormField::Title);

        app.prev_form_field();
        assert_eq!(app.form_field, FormField::Assignee);
    }

    #[test]
    fn test_form_field_input() {
        let mut app = App::default();
        app.start_create_card();

        *app.get_current_field_input_mut() = "Test Title".to_string();
        assert_eq!(app.form_data.title, "Test Title");

        app.next_form_field();
        *app.get_current_field_input_mut() = "Test Description".to_string();
        assert_eq!(app.form_data.description, "Test Description");

        app.next_form_field();
        *app.get_current_field_input_mut() = "Test Assignee".to_string();
        assert_eq!(app.form_data.assignee, "Test Assignee");
    }

    #[test]
    fn test_submit_card_empty_title() {
        let temp_dir = TempDir::new().unwrap();
        let board_service = BoardService::new();
        board_service
            .initialize(temp_dir.path(), Some("Test".to_string()))
            .unwrap();

        let mut app = App::new(temp_dir.path().to_path_buf());
        app.load_board().unwrap();
        app.start_create_card();

        *app.get_current_field_input_mut() = "".to_string();
        app.submit_card().unwrap();

        assert!(app.error_message.is_some());
        assert_eq!(app.error_message, Some("Title is required".to_string()));
        assert_eq!(app.state, AppState::CreateCard);
    }

    #[test]
    fn test_create_card_success() {
        let temp_dir = TempDir::new().unwrap();
        let board_service = BoardService::new();
        board_service
            .initialize(temp_dir.path(), Some("Test".to_string()))
            .unwrap();

        let mut app = App::new(temp_dir.path().to_path_buf());
        app.load_board().unwrap();
        app.start_create_card();

        *app.get_current_field_input_mut() = "Test Card".to_string();
        app.next_form_field();
        *app.get_current_field_input_mut() = "Test Description".to_string();
        app.next_form_field();
        *app.get_current_field_input_mut() = "Test Assignee".to_string();

        app.submit_card().unwrap();

        assert!(app.error_message.is_none());
        assert_eq!(app.state, AppState::Board);
        assert!(app.board.is_some());
        assert_eq!(app.board.as_ref().unwrap().cards.len(), 1);
        assert_eq!(app.board.as_ref().unwrap().cards[0].title, "Test Card");
    }

    #[test]
    fn test_cancel_card() {
        let mut app = App::default();
        app.start_create_card();

        *app.get_current_field_input_mut() = "Test Title".to_string();
        app.cancel_card();

        assert_eq!(app.state, AppState::Board);
        assert_eq!(app.form_data.title, "");
        assert_eq!(app.form_data.description, "");
        assert_eq!(app.form_data.assignee, "");
    }

    #[test]
    fn test_delete_card() {
        use crate::application::CardService;

        let temp_dir = TempDir::new().unwrap();
        let board_service = BoardService::new();
        board_service
            .initialize(temp_dir.path(), Some("Test".to_string()))
            .unwrap();

        let card_service = CardService::new();
        let created = card_service
            .create(temp_dir.path(), "Test Card".to_string(), None, None, None)
            .unwrap();

        let mut app = App::new(temp_dir.path().to_path_buf());
        app.load_board().unwrap();
        assert_eq!(app.board.as_ref().unwrap().cards.len(), 1);

        card_service
            .delete(temp_dir.path(), &created.card_id)
            .unwrap();

        app.load_board().unwrap();
        assert_eq!(app.board.as_ref().unwrap().cards.len(), 0);
    }

    #[test]
    fn test_start_move_card() {
        let temp_dir = TempDir::new().unwrap();
        let board_service = BoardService::new();
        board_service
            .initialize(temp_dir.path(), Some("Test".to_string()))
            .unwrap();

        let card_service = CardService::new();
        let _created = card_service
            .create(temp_dir.path(), "Test Card".to_string(), None, None, None)
            .unwrap();

        let mut app = App::new(temp_dir.path().to_path_buf());
        app.load_board().unwrap();
        app.selected_column = 0;
        app.selected_card = Some(0);

        app.start_move_card();

        assert_eq!(app.state, AppState::MoveCard);
        assert!(app.error_message.is_none());
    }

    #[test]
    fn test_move_card_navigation() {
        let temp_dir = TempDir::new().unwrap();
        let board_service = BoardService::new();
        board_service
            .initialize(temp_dir.path(), Some("Test".to_string()))
            .unwrap();

        let card_service = CardService::new();
        let _created = card_service
            .create(temp_dir.path(), "Test Card".to_string(), None, None, None)
            .unwrap();

        let mut app = App::new(temp_dir.path().to_path_buf());
        app.load_board().unwrap();
        app.selected_column = 0;
        app.selected_card = Some(0);

        app.start_move_card();
        let initial_target = app.selected_target_column;

        app.move_card_right();
        assert_eq!(app.selected_target_column, initial_target + 1);

        app.move_card_right();
        assert_eq!(app.selected_target_column, initial_target + 2);

        app.move_card_left();
        assert_eq!(app.selected_target_column, initial_target + 1);
    }

    #[test]
    fn test_move_card_right_boundary() {
        let temp_dir = TempDir::new().unwrap();
        let board_service = BoardService::new();
        board_service
            .initialize(temp_dir.path(), Some("Test".to_string()))
            .unwrap();

        let card_service = CardService::new();
        let _created = card_service
            .create(temp_dir.path(), "Test Card".to_string(), None, None, None)
            .unwrap();

        let mut app = App::new(temp_dir.path().to_path_buf());
        app.load_board().unwrap();
        app.selected_column = 0;
        app.selected_card = Some(0);

        app.start_move_card();
        app.selected_target_column = app.board.as_ref().unwrap().columns.len() - 1;

        app.move_card_right();
        assert_eq!(
            app.selected_target_column,
            app.board.as_ref().unwrap().columns.len() - 1
        );
    }

    #[test]
    fn test_move_card_left_boundary() {
        let temp_dir = TempDir::new().unwrap();
        let board_service = BoardService::new();
        board_service
            .initialize(temp_dir.path(), Some("Test".to_string()))
            .unwrap();

        let card_service = CardService::new();
        let _created = card_service
            .create(temp_dir.path(), "Test Card".to_string(), None, None, None)
            .unwrap();

        let mut app = App::new(temp_dir.path().to_path_buf());
        app.load_board().unwrap();
        app.selected_column = 0;
        app.selected_card = Some(0);

        app.start_move_card();
        app.selected_target_column = 0;

        app.move_card_left();
        assert_eq!(app.selected_target_column, 0);
    }

    #[test]
    fn test_confirm_move_card_success() {
        let temp_dir = TempDir::new().unwrap();
        let board_service = BoardService::new();
        board_service
            .initialize(temp_dir.path(), Some("Test".to_string()))
            .unwrap();

        let card_service = CardService::new();
        let created = card_service
            .create(temp_dir.path(), "Test Card".to_string(), None, None, None)
            .unwrap();

        let mut app = App::new(temp_dir.path().to_path_buf());
        app.load_board().unwrap();
        app.selected_column = 0;
        app.selected_card = Some(0);

        app.start_move_card();
        app.move_card_right();

        app.confirm_move_card().unwrap();

        assert_eq!(app.state, AppState::CardDetail);
        assert!(app.error_message.is_none());

        let card = app
            .board
            .as_ref()
            .unwrap()
            .get_card(&created.card_id)
            .unwrap();
        assert_eq!(card.column_id, "in_progress");
    }

    #[test]
    fn test_cancel_move_card() {
        let temp_dir = TempDir::new().unwrap();
        let board_service = BoardService::new();
        board_service
            .initialize(temp_dir.path(), Some("Test".to_string()))
            .unwrap();

        let card_service = CardService::new();
        let _created = card_service
            .create(temp_dir.path(), "Test Card".to_string(), None, None, None)
            .unwrap();

        let mut app = App::new(temp_dir.path().to_path_buf());
        app.load_board().unwrap();
        app.selected_column = 0;
        app.selected_card = Some(0);

        app.start_move_card();
        assert_eq!(app.state, AppState::MoveCard);

        app.cancel_move_card();

        assert_eq!(app.state, AppState::CardDetail);
        assert!(app.error_message.is_none());
    }

    #[test]
    fn test_complete_move_flow() {
        let temp_dir = TempDir::new().unwrap();
        let board_service = BoardService::new();
        board_service
            .initialize(temp_dir.path(), Some("Test".to_string()))
            .unwrap();

        let card_service = CardService::new();
        let created = card_service
            .create(temp_dir.path(), "Test Card".to_string(), None, None, None)
            .unwrap();

        let mut app = App::new(temp_dir.path().to_path_buf());
        app.load_board().unwrap();
        app.selected_column = 0;
        app.selected_card = Some(0);

        app.enter_cards();
        assert_eq!(app.state, AppState::Board);
        assert_eq!(app.focus, Focus::Cards);
        assert_eq!(app.selected_card, Some(0));

        app.start_move_card();
        assert_eq!(app.state, AppState::MoveCard);

        app.move_card_right();
        assert_eq!(app.selected_target_column, 1);

        app.confirm_move_card().unwrap();

        assert_eq!(app.state, AppState::CardDetail);
        assert!(app.error_message.is_none());

        let card = app
            .board
            .as_ref()
            .unwrap()
            .get_card(&created.card_id)
            .unwrap();
        assert_eq!(card.column_id, "in_progress");
    }

    #[test]
    fn test_enter_cards_focus_transitions_to_card_detail() {
        let temp_dir = TempDir::new().unwrap();
        let board_service = BoardService::new();
        board_service
            .initialize(temp_dir.path(), Some("Test".to_string()))
            .unwrap();

        let card_service = CardService::new();
        let _created = card_service
            .create(temp_dir.path(), "Test Card".to_string(), None, None, None)
            .unwrap();

        let mut app = App::new(temp_dir.path().to_path_buf());
        app.load_board().unwrap();
        app.selected_column = 0;

        assert_eq!(app.state, AppState::Board);
        assert_eq!(app.focus, Focus::Columns);

        app.enter_cards();
        assert_eq!(app.focus, Focus::Cards);
        assert_eq!(app.selected_card, Some(0));

        app.enter_card_detail();
        assert_eq!(app.state, AppState::CardDetail);
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new(PathBuf::from("."))
    }
}
