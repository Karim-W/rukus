use crossterm::terminal;
use std::io::{self, Write};
// pub mod cleanup;
pub mod content;
pub mod cursors;
pub mod editor;
pub mod output;
pub mod reader;
pub mod rows;

fn main() -> crossterm::Result<()> {
    terminal::enable_raw_mode()?;
    let mut editor = editor::Editor::new();
    while editor.run()? {}
    editor.exit().expect("this should clear");
    io::stdout().flush().unwrap();
    Ok(())
}
