pub mod cli;
pub mod formatter;
pub mod markdown_renderer;
pub mod scrollable_textarea;
pub mod title_popup;
pub mod title_select_popup;
pub mod ui;
pub mod ui_handler;
pub mod utils;

pub use formatter::{format_json, format_markdown};
pub use markdown_renderer::MarkdownRenderer;
pub use scrollable_textarea::ScrollableTextArea;
pub use title_popup::TitlePopup;
pub use title_select_popup::TitleSelectPopup;
pub use utils::{load_textareas, save_textareas};

use dirs::home_dir;
use std::path::PathBuf;

pub fn get_save_file_path() -> PathBuf {
    home_dir().unwrap_or_default().join("thoth_notes.md")
}

pub const ORANGE: ratatui::style::Color = ratatui::style::Color::Rgb(255, 165, 0);
