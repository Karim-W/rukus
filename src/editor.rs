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
            KeyEvent {
                code: KeyCode::Char(val @ ('w' | 'a' | 's' | 'd')),
                modifiers: KeyModifiers::NONE,
                kind: _,
                state: _,
            } => self.output.move_cursor(val),
            KeyEvent {
                code: KeyCode::Char(val),
                modifiers: KeyModifiers::NONE,
                kind: _,
                state: _,
            } => self.output.insert_row(val),
            KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::NONE,
                kind: _,
                state: _,
            } => self.output.handle_create_line(),
            // KeyEvent {
            //     code: KeyCode::Char(val),
            //     modifiers: KeyModifiers::NONE,
            //     kind: _,
            //     state: _,
            // } => println!("Key pressed: {}", val),
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
