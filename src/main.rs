use crossterm::terminal;

pub mod cleanup;
pub mod content;
pub mod editor;
pub mod output;
pub mod reader;

fn main() -> crossterm::Result<()> {
    terminal::enable_raw_mode()?; /* modify */
    let mut editor = editor::Editor::new();
    while editor.run()? {}
    editor.exit().expect("this should clear");
    /* end */
    Ok(())
}
