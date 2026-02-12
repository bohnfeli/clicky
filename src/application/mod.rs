//! Application layer containing use cases and application services.
//!
//! This layer coordinates between the domain layer (business logic) and
//! the infrastructure layer (storage, I/O). It implements the use cases
//! that fulfill user requirements.

pub mod board_service;
pub mod card_service;

pub use board_service::{BoardService, BoardServiceError};
pub use card_service::CardService;
