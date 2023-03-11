use crossterm::terminal;

pub mod cleanup;
pub mod editor;

fn main() -> crossterm::Result<()> {
    terminal::enable_raw_mode()?; /* modify */
    let editor = editor::Editor::new();
    while editor.run()? {}
    /* end */
    Ok(())
}
