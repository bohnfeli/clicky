//! TUI rendering functions.

use ratatui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};
use std::io;

use crate::cli::tui::app::App;
use crate::cli::tui::state::AppState;
use crate::cli::tui::state::Focus;

/// Type alias for terminal
pub type Tui = Terminal<CrosstermBackend<io::Stdout>>;

/// Draw the footer with context-aware hints.
fn draw_footer(frame: &mut Frame, app: &App, area: Rect) {
    let hints = match app.state {
        AppState::Board => match app.focus {
            Focus::Columns => "h/l/← → Select column | k/j/↑↓ Navigate | Enter Details | ? Help",
            Focus::Cards => "Esc Exit selection | q Return to columns | ? Help",
            _ => "? Help",
        },
        AppState::CardDetail => "e Edit | d Delete | m Move | q Back | ? Help",
        AppState::CreateCard => "i Edit | Enter Save | Esc Cancel | ? Help",
        AppState::EditCard => "i Edit | Enter Save | Esc Cancel | ? Help",
        AppState::ConfirmDelete => "y Confirm | n Cancel",
        AppState::Help => "Esc Close help | ? Toggle",
    };

    let paragraph = Paragraph::new(hints)
        .style(Style::default().fg(Color::Cyan))
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, area);
}

/// Draw help overlay with comprehensive shortcuts.
fn draw_help_overlay(frame: &mut Frame) {
    let area = centered_rect(frame.size(), 70, 80);

    frame.render_widget(Clear, area);

    let help_text = vec![
        Line::from(" Keyboard Shortcuts "),
        Line::from(""),
        Line::from(" GLOBAL:"),
        Line::from("   q     Quit"),
        Line::from("   ?     Toggle help"),
        Line::from(""),
        Line::from(" BOARD VIEW:"),
        Line::from("   h/←   Previous column"),
        Line::from("   l/→   Next column"),
        Line::from("   k/↑   Previous card"),
        Line::from("   j/↓   Next card"),
        Line::from("   Enter Select card"),
        Line::from("   Esc   Exit selection"),
        Line::from(""),
        Line::from(" CARD DETAIL:"),
        Line::from("   e     Edit card"),
        Line::from("   d     Delete card"),
        Line::from("   m     Move card"),
        Line::from("   Esc   Return to board"),
        Line::from(""),
        Line::from(" Press ? or Esc to close "),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Help ")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        );

    let paragraph = Paragraph::new(help_text)
        .block(block)
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, area);
}

/// Draw application UI.
pub fn draw(frame: &mut Frame, app: &mut App) {
    // If help is shown, draw it and return early
    if app.show_help {
        draw_help_overlay(frame);
        return;
    }

    // Main layout: header + content + footer
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(2),
        ])
        .split(frame.size());

    // Draw main content based on state
    match app.state {
        AppState::Board => draw_board_view(frame, app, chunks[1]),
        AppState::CardDetail => draw_card_detail_view(frame, app, chunks[1]),
        AppState::CreateCard => draw_create_card_view(frame, app, chunks[1]),
        AppState::EditCard => draw_edit_card_view(frame, app, chunks[1]),
        AppState::ConfirmDelete => draw_confirm_delete_view(frame, app, chunks[1]),
        AppState::Help => {}
    }

    // Draw error message overlay (if present)
    if let Some(ref error) = app.error_message {
        draw_error_message(frame, error);
    } else {
        // Draw footer hints
        draw_footer(frame, app, chunks[2]);
    }
}

/// Draw the main board view with columns and cards.
fn draw_board_view(frame: &mut Frame, app: &App, area: Rect) {
    // Header
    let board_name = app
        .board
        .as_ref()
        .map(|b| b.name.as_str())
        .unwrap_or("No Board");

    let header = Paragraph::new(format!(" Clicky: {} ", board_name))
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);

    let header_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3), Constraint::Min(0)].as_ref())
        .split(area);

    frame.render_widget(header, header_chunks[0]);

    // Columns and cards
    draw_columns_and_cards(frame, header_chunks[1], app);
}

/// Draw columns and cards layout.
fn draw_columns_and_cards(frame: &mut Frame, area: Rect, app: &App) {
    if let Some(board) = &app.board {
        let column_count = board.columns.len();
        let column_width = 100 / column_count.max(1);

        let constraints: Vec<Constraint> = board
            .columns
            .iter()
            .map(|_| Constraint::Percentage(column_width as u16))
            .collect();

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints::<&[Constraint]>(constraints.as_ref())
            .split(area);

        for (i, column) in board.columns.iter().enumerate() {
            draw_column(frame, chunks[i], app, &column.id, i);
        }
    }
}

/// Draw a single column with its cards.
fn draw_column(frame: &mut Frame, area: Rect, app: &App, column_id: &str, index: usize) {
    let board = app.board.as_ref().unwrap();
    let column = board.columns.iter().find(|c| c.id == column_id).unwrap();

    let cards: Vec<_> = board
        .cards
        .iter()
        .filter(|c| c.column_id == column_id)
        .collect();

    let is_focused = app.selected_column == index;

    // Create list items for cards
    let items: Vec<ListItem> = cards
        .iter()
        .enumerate()
        .map(|(i, card)| {
            let assignee = card
                .assignee
                .as_ref()
                .map(|a| format!(" [@{}]", a))
                .unwrap_or_default();

            let is_selected =
                app.selected_card == Some(i) && is_focused && app.focus == Focus::Cards;

            let style = if is_selected {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };

            let title = if card.title.len() > 25 {
                format!("{}...{}", &card.title[..25], assignee)
            } else {
                format!("{}{}", card.title, assignee)
            };

            ListItem::new(Span::styled(title, style))
        })
        .collect();

    // Column block
    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" {} ({}) ", column.name, cards.len()))
        .border_style(if is_focused && app.focus == Focus::Columns {
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        });

    let list = List::new(items).block(block).highlight_style(
        Style::default()
            .add_modifier(Modifier::BOLD)
            .bg(Color::DarkGray),
    );

    frame.render_widget(list, area);
}

/// Draw card detail view.
fn draw_card_detail_view(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(10),
            Constraint::Length(3),
        ])
        .split(area);

    // Header
    let header = Paragraph::new(" Card Details ")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);
    frame.render_widget(header, chunks[0]);

    // Card content
    if let Some(card_id) = app.selected_card_id() {
        if let Some(board) = &app.board {
            if let Some(card) = board.get_card(&card_id) {
                let column = board.columns.iter().find(|c| c.id == card.column_id);

                let mut text = vec![
                    Line::from(format!("ID: {}", card.id)),
                    Line::from(""),
                    Line::from(format!("Title: {}", card.title)),
                    Line::from(""),
                ];

                if let Some(desc) = &card.description {
                    text.push(Line::from(format!("Description: {}", desc)));
                    text.push(Line::from(""));
                }

                if let Some(assignee) = &card.assignee {
                    text.push(Line::from(format!("Assignee: {}", assignee)));
                    text.push(Line::from(""));
                }

                if let Some(column) = column {
                    text.push(Line::from(format!("Column: {}", column.name)));
                    text.push(Line::from(""));
                }

                text.push(Line::from(format!(
                    "Created: {}",
                    card.created_at.format("%Y-%m-%d %H:%M")
                )));
                text.push(Line::from(format!(
                    "Updated: {}",
                    card.updated_at.format("%Y-%m-%d %H:%M")
                )));

                let paragraph = Paragraph::new(text).wrap(Wrap { trim: true });
                frame.render_widget(paragraph, chunks[1]);
            }
        }
    }
}

/// Draw create card view.
fn draw_create_card_view(frame: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(area);

    // Header
    let header = Paragraph::new(" Create Card ")
        .style(
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);
    frame.render_widget(header, chunks[0]);

    // Input fields
    let title_block = Block::default()
        .borders(Borders::ALL)
        .title(" Title (required) ");

    let title_paragraph = Paragraph::new(app.input.as_str())
        .block(title_block)
        .wrap(Wrap { trim: false });
    frame.render_widget(title_paragraph, chunks[1]);

    // Footer handled by main draw function
}

/// Draw edit card view.
fn draw_edit_card_view(frame: &mut Frame, app: &App, area: Rect) {
    draw_create_card_view(frame, app, area);
}

/// Draw confirmation dialog for deletion.
fn draw_confirm_delete_view(frame: &mut Frame, app: &App, area: Rect) {
    let area = centered_rect(area, 40, 10);

    frame.render_widget(Clear, area);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Confirm Delete ")
        .style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD));

    let text = if let Some(card_id) = app.selected_card_id() {
        format!("Are you sure you want to delete {}?", card_id)
    } else {
        "Are you sure you want to delete this card?".to_string()
    };

    let paragraph = Paragraph::new(Text::from(vec![
        Line::from(""),
        Line::from(text),
        Line::from(""),
        Line::from(""),
        Line::from(" y: confirm | n: cancel "),
    ]))
    .block(block)
    .alignment(Alignment::Center);

    frame.render_widget(paragraph, area);
}

/// Draw error message overlay.
fn draw_error_message(frame: &mut Frame, message: &str) {
    let area = centered_rect(frame.size(), 60, 6);

    frame.render_widget(Clear, area);

    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Error ")
        .style(Style::default().fg(Color::Red));

    let paragraph = Paragraph::new(message)
        .block(block)
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, area);
}

/// Helper to create a centered rectangle.
fn centered_rect(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
