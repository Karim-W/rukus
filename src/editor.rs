use crate::output;
use crate::reader;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
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
    pub fn process_keypress(&mut self) -> crossterm::Result<bool> {
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

    pub fn run(&mut self) -> crossterm::Result<bool> {
        self.output.refresh_screen()?;
        self.process_keypress()
    }

    pub fn exit(&mut self) -> crossterm::Result<()> {
        self.output.clear_screen()
    }
}
