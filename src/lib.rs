#![doc = include_str!("../README.md")]
#![forbid(unsafe_code)]

/// Reusable terminal UI elements.
pub mod components;
/// Small responsive layout helpers.
pub mod layout;
/// Project Mambo colors and styles.
pub mod theme;

pub use components::{
    EmptyState, Footer, Header, Help, KeyHint, Panel, Shell, Status, StatusLine, selection_list,
};
pub use layout::responsive_columns;
pub use theme::Theme;

/// The exact ratatui version used by this crate.
///
/// Applications may import ratatui types through this re-export to avoid a
/// version mismatch.
pub use ratatui;
