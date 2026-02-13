//! Interactive prompt handlers for Clicky CLI.
//!
//! This module provides interactive wizards for all Clicky commands
//! when the --interactive flag is used.

#[cfg(feature = "interactive")]
pub mod create_prompt;
#[cfg(feature = "interactive")]
pub mod delete_prompt;
#[cfg(feature = "interactive")]
pub mod init_prompt;
#[cfg(feature = "interactive")]
pub mod list_prompt;
#[cfg(feature = "interactive")]
pub mod move_prompt;
#[cfg(feature = "interactive")]
pub mod show_prompt;
#[cfg(feature = "interactive")]
pub mod update_prompt;
#[cfg(feature = "interactive")]
pub mod utils;
