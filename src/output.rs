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
        let screen_columns = self.win_size.0;
        let screen_rows = self.win_size.1;
        let mut welcome = format!("----- v0.0.1 -----");
        if welcome.len() > screen_columns {
            welcome.truncate(screen_columns)
        }
        self.content.push_str(&welcome);
        for _ in 1..screen_rows - 1 {
            println!("~\r");
            // self.content.push('~');
            // self.content.push('\r');
        }
        queue!(self.content, terminal::Clear(ClearType::UntilNewLine)).unwrap();
        self.content.push_str("\r\n")
    }

    pub fn refresh_screen(&mut self) -> crossterm::Result<()> {
        queue!(
            self.content,
            cursor::Hide,
            // terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )?;
        self.draw_rows();
        queue!(self.content, cursor::MoveTo(0, 0), cursor::Show)?;
        self.content.flush()
    }
}
