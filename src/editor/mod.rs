use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
pub mod output;
pub mod reader;

pub struct Editor {
    r: reader::Reader,
    output: output::Output,
}

impl Editor {
    pub fn new() -> Self {
        Self {
            r: reader::Reader::new(),
            output: output::Output::new(),
        }
    }
    pub fn process_keypress(&self) -> crossterm::Result<bool> {
        match self.r.read_key()? {
            KeyEvent {
                code: KeyCode::Char('c'),
                modifiers: KeyModifiers::CONTROL,
                kind: _,
                state: _,
            } => return Ok(false),
            _ => {}
        }
        Ok(true)
    }

    pub fn run(&self) -> crossterm::Result<bool> {
        self.output.refresh_screen()?;
        self.process_keypress()
    }
}
