//! Domain layer containing core business logic and entities.
//!
//! This module defines the fundamental concepts of the kanban board:
//! - Cards: Individual tasks with properties
//! - Columns: Status categories that contain cards
//! - Boards: Collections of columns representing a project

pub mod board;
pub mod card;
pub mod column;

pub use board::Board;
pub use card::Card;
pub use column::Column;
