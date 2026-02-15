//! TUI application state.

use crate::application::board_service::MoveDirection;
use crate::application::{BoardService, CardService};
use crate::cli::tui::state::{CardFormData, FormMode, InputMode, Selection, View};
use crate::domain::Board;
use std::path::PathBuf;

/// Main TUI application.
pub struct App {
    /// Current view state (replaces state, focus, input_mode, etc.)
    pub view: View,
    /// Board path
    pub board_path: PathBuf,
    /// Board data
    pub board: Option<Board>,
    /// Error message to display
    pub error_message: Option<String>,
    /// Form data for card creation/editing
    pub form_data: Option<CardFormData>,
}

impl App {
    pub fn new(board_path: PathBuf) -> Self {
        Self {
            view: View::default(),
            board_path,
            board: None,
            error_message: None,
            form_data: None,
        }
    }

    pub fn toggle_help(&mut self) {
        self.view = self.view.clone().toggle_help();
        if !self.view.is_help() {
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

    pub fn get_current_column(&self) -> Option<&str> {
        let selected_column = self.view.selected_column();
        self.board
            .as_ref()
            .and_then(|b| b.columns.get(selected_column))
            .map(|c| c.id.as_str())
    }

    pub fn move_left(&mut self) {
        if let View::Board {
            selected_column, ..
        } = &mut self.view
        {
            if *selected_column > 0 {
                *selected_column -= 1;
                self.reset_selection();
            }
        }
    }

    pub fn move_right(&mut self) {
        if let View::Board {
            selected_column, ..
        } = &mut self.view
        {
            if let Some(board) = &self.board {
                if *selected_column < board.columns.len().saturating_sub(1) {
                    *selected_column += 1;
                    self.reset_selection();
                }
            }
        }
    }

    pub fn move_up(&mut self) {
        if let View::Board {
            selected_column,
            selection,
        } = &mut self.view
        {
            if let Some(board) = &self.board {
                if let Some(column) = board.columns.get(*selected_column) {
                    let card_count = board
                        .cards
                        .iter()
                        .filter(|c| c.column_id == column.id)
                        .count();

                    match selection {
                        Selection::Highlighted { column, card_index }
                            if *column == *selected_column =>
                        {
                            *card_index = card_index.saturating_sub(1);
                        }
                        _ => {
                            if card_count > 0 {
                                *selection = Selection::Highlighted {
                                    column: *selected_column,
                                    card_index: card_count.saturating_sub(1),
                                };
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn move_down(&mut self) {
        if let View::Board {
            selected_column,
            selection,
        } = &mut self.view
        {
            if let Some(board) = &self.board {
                if let Some(column) = board.columns.get(*selected_column) {
                    let card_count = board
                        .cards
                        .iter()
                        .filter(|c| c.column_id == column.id)
                        .count();

                    match selection {
                        Selection::Highlighted { column, card_index }
                            if *column == *selected_column =>
                        {
                            if *card_index < card_count.saturating_sub(1) {
                                *card_index += 1;
                            }
                        }
                        _ => {
                            if card_count > 0 {
                                *selection = Selection::Highlighted {
                                    column: *selected_column,
                                    card_index: 0,
                                };
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn enter_cards(&mut self) {
        if let View::Board {
            selected_column,
            selection,
        } = &mut self.view
        {
            if let Some(board) = &self.board {
                if let Some(column) = board.columns.get(*selected_column) {
                    let card_id = match selection {
                        Selection::Highlighted {
                            column: col,
                            card_index,
                        } if *col == *selected_column => column.cards.get(*card_index).cloned(),
                        Selection::Highlighted { .. } => column.cards.first().cloned(),
                        _ => return,
                    };

                    if let Some(id) = card_id {
                        *selection = Selection::Selected { card_id: id };
                    }
                }
            }
        }
    }

    pub fn open_card_detail(&mut self) {
        self.enter_cards();
        if let Some(card_id) = self.view.selected_card_id().map(|s| s.to_string()) {
            self.view = View::CardDetail { card_id };
        }
    }

    fn reset_selection(&mut self) {
        if let View::Board { selection, .. } = &mut self.view {
            *selection = Selection::None;
        }
    }

    pub fn exit_cards(&mut self) {
        if let View::Board { selection, .. } = &mut self.view {
            *selection = Selection::None;
        }
    }

    pub fn clear_error(&mut self) {
        self.error_message = None;
    }

    pub fn start_create_card(&mut self) {
        let column_id = self.get_current_column().unwrap_or("todo").to_string();
        self.view = View::CardForm {
            mode: FormMode::Create { column_id },
        };
        self.form_data = Some(CardFormData::default());
    }

    pub fn next_form_field(&mut self) {
        if let Some(form_data) = &mut self.form_data {
            form_data.current_field = form_data.current_field.next();
        }
    }

    pub fn prev_form_field(&mut self) {
        if let Some(form_data) = &mut self.form_data {
            form_data.current_field = form_data.current_field.prev();
        }
    }

    pub fn get_current_field_input_mut(&mut self) -> Option<&mut String> {
        self.form_data.as_mut().map(|fd| fd.current_field_mut())
    }

    pub fn set_input_mode(&mut self, mode: InputMode) {
        if let Some(form_data) = &mut self.form_data {
            form_data.input_mode = mode;
        }
    }

    pub fn get_input_mode(&self) -> InputMode {
        self.form_data
            .as_ref()
            .map(|fd| fd.input_mode)
            .unwrap_or(InputMode::Normal)
    }

    pub fn submit_card(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let form_data = self.form_data.take().ok_or("No form data available")?;

        if !form_data.is_valid() {
            self.error_message = Some("Title is required".to_string());
            self.form_data = Some(form_data);
            return Ok(());
        }

        let (title, description, assignee) = form_data.to_card_values();

        let column_id = match &self.view {
            View::CardForm {
                mode: FormMode::Create { column_id },
            } => column_id.clone(),
            View::CardForm {
                mode: FormMode::Edit { card_id },
            } => {
                if let Some(board) = &self.board {
                    if let Some(card) = board.get_card(card_id) {
                        card.column_id.clone()
                    } else {
                        "todo".to_string()
                    }
                } else {
                    "todo".to_string()
                }
            }
            _ => "todo".to_string(),
        };

        let card_service = CardService::new();
        match &self.view {
            View::CardForm {
                mode: FormMode::Edit { card_id },
            } => {
                card_service.update(
                    &self.board_path,
                    card_id,
                    Some(title),
                    Some(description),
                    Some(assignee),
                )?;
            }
            _ => {
                card_service.create(
                    &self.board_path,
                    title,
                    description,
                    assignee,
                    Some(column_id),
                )?;
            }
        }

        self.load_board()?;
        self.view = View::default();
        Ok(())
    }

    pub fn cancel_card(&mut self) {
        self.form_data = None;
        self.view = View::default();
    }

    pub fn start_move_card(&mut self) {
        if let Some(card_id) = self.view.selected_card_id().map(|s| s.to_string()) {
            let target_column = if let Some(board) = &self.board {
                board
                    .get_card(&card_id)
                    .and_then(|c| board.columns.iter().position(|col| col.id == c.column_id))
            } else {
                None
            }
            .unwrap_or(0);

            self.view = View::MoveCard {
                card_id,
                target_column,
            };
            self.clear_error();
        }
    }

    pub fn move_card_left(&mut self) {
        if let View::MoveCard { target_column, .. } = &mut self.view {
            if *target_column > 0 {
                *target_column -= 1;
            }
        }
    }

    pub fn move_card_right(&mut self) {
        if let View::MoveCard { target_column, .. } = &mut self.view {
            if let Some(board) = &self.board {
                if *target_column < board.columns.len().saturating_sub(1) {
                    *target_column += 1;
                }
            }
        }
    }

    pub fn confirm_move_card(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let (card_id, target_column_idx) = match &self.view {
            View::MoveCard {
                card_id,
                target_column,
            } => (card_id.clone(), *target_column),
            _ => return Err("Not in move card view".into()),
        };

        let target_column_id = self
            .board
            .as_ref()
            .and_then(|b| b.columns.get(target_column_idx))
            .map(|c| c.id.clone())
            .ok_or("Invalid target column")?;

        let card_service = CardService::new();
        card_service.move_to(&self.board_path, &card_id, &target_column_id)?;

        self.load_board()?;
        self.view = View::CardDetail { card_id };
        self.clear_error();

        Ok(())
    }

    pub fn cancel_move_card(&mut self) {
        if let View::MoveCard { card_id, .. } = &self.view {
            self.view = View::CardDetail {
                card_id: card_id.clone(),
            };
            self.clear_error();
        }
    }

    pub fn quick_move_card_left(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.view.is_card_selected() {
            if let Some(card_id) = self.view.selected_card_id().map(|s| s.to_string()) {
                if let View::Board {
                    selected_column, ..
                } = &self.view
                {
                    if *selected_column > 0 {
                        let target_column_idx = *selected_column - 1;
                        self.move_selected_card_to(&card_id, target_column_idx)?;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn quick_move_card_right(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.view.is_card_selected() {
            if let Some(card_id) = self.view.selected_card_id().map(|s| s.to_string()) {
                if let View::Board {
                    selected_column, ..
                } = &self.view
                {
                    if let Some(board) = &self.board {
                        if *selected_column < board.columns.len().saturating_sub(1) {
                            let target_column_idx = *selected_column + 1;
                            self.move_selected_card_to(&card_id, target_column_idx)?;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn move_selected_card_to(
        &mut self,
        card_id: &str,
        target_column_idx: usize,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let target_column_id = self
            .board
            .as_ref()
            .and_then(|b| b.columns.get(target_column_idx))
            .map(|c| c.id.clone())
            .ok_or("Invalid target column")?;

        let card_service = CardService::new();
        card_service.move_to(&self.board_path, card_id, &target_column_id)?;
        self.load_board()?;

        if let View::Board {
            selected_column, ..
        } = &mut self.view
        {
            *selected_column = target_column_idx;
        }

        Ok(())
    }

    pub fn deselect_card(&mut self) {
        if let View::Board { selection, .. } = &mut self.view {
            if matches!(selection, Selection::Selected { .. }) {
                *selection = Selection::None;
            }
        }
    }

    pub fn start_edit_card(&mut self, card_id: String) {
        if let Some(board) = &self.board {
            if let Some(card) = board.get_card(&card_id) {
                self.view = View::CardForm {
                    mode: FormMode::Edit {
                        card_id: card_id.clone(),
                    },
                };
                self.form_data = Some(CardFormData::for_edit(
                    card.title.clone(),
                    card.description.clone(),
                    card.assignee.clone(),
                ));
            }
        }
    }

    pub fn start_delete_card(&mut self, card_id: String) {
        self.view = View::ConfirmDelete { card_id };
        self.clear_error();
    }

    pub fn confirm_delete_card(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let card_id = match &self.view {
            View::ConfirmDelete { card_id } => card_id.clone(),
            _ => return Err("Not in confirm delete view".into()),
        };

        let card_service = CardService::new();
        card_service.delete(&self.board_path, &card_id)?;

        self.load_board()?;
        self.view = View::default();

        Ok(())
    }

    pub fn cancel_delete_card(&mut self) {
        if let View::ConfirmDelete { card_id } = &self.view {
            self.view = View::CardDetail {
                card_id: card_id.clone(),
            };
        }
    }

    pub fn move_selected_card_up(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.view.is_card_selected() {
            return Ok(());
        }

        if let Some(card_id) = self.view.selected_card_id() {
            let board_service = BoardService::new();
            if board_service
                .reorder_card_in_column(&self.board_path, card_id, MoveDirection::Up)
                .is_ok()
            {
                self.load_board()?;
            }
        }
        Ok(())
    }

    pub fn move_selected_card_down(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.view.is_card_selected() {
            return Ok(());
        }

        if let Some(card_id) = self.view.selected_card_id() {
            let board_service = BoardService::new();
            if board_service
                .reorder_card_in_column(&self.board_path, card_id, MoveDirection::Down)
                .is_ok()
            {
                self.load_board()?;
            }
        }
        Ok(())
    }

    pub fn get_selected_card_index(&self) -> Option<usize> {
        if let (Some(board), Some(card_id)) = (&self.board, self.view.selected_card_id()) {
            let selected_column = self.view.selected_column();
            if let Some(column) = board.columns.get(selected_column) {
                for (i, cid) in column.cards.iter().enumerate() {
                    if cid == card_id {
                        return Some(i);
                    }
                }
            }
        }
        None
    }

    #[allow(dead_code)]
    pub fn get_highlighted_card_index(&self) -> Option<usize> {
        if let View::Board {
            selection: Selection::Highlighted { card_index, .. },
            ..
        } = &self.view
        {
            Some(*card_index)
        } else {
            None
        }
    }

    pub fn get_form_data(&self) -> Option<&CardFormData> {
        self.form_data.as_ref()
    }
}

#[cfg(test)]
#[path = "./app_tests.rs"]
mod app_tests;

impl Default for App {
    fn default() -> Self {
        Self::new(PathBuf::from("."))
    }
}
