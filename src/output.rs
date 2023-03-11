use crate::content;
use crossterm::cursor;
use crossterm::execute;
use crossterm::queue;
use crossterm::terminal::{self, ClearType};
use std::io::stdout;
use std::io::Write;

pub struct Output {
    win_size: (usize, usize),
    content: content::Content,
}

impl Output {
    pub fn new() -> Self {
        let win_size = terminal::size()
            .map(|(x, y)| (x as usize, y as usize))
            .unwrap();
        Self {
            win_size,
            content: content::Content::new(),
        }
    }

    pub fn clear_screen(&self) -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }
    fn draw_rows(&mut self) {
        let screen_rows = self.win_size.1;
        for _ in 0..screen_rows - 1 {
            self.content.push('\r')
        }
        self.content.push_str("\r\n")
    }

    pub fn refresh_screen(&mut self) -> crossterm::Result<()> {
        queue!(
            self.content,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )?;
        self.draw_rows();
        queue!(self.content, cursor::MoveTo(0, 0))?;
        self.content.flush()
    }
}
