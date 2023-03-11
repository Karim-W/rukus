use crossterm::cursor;
use crossterm::execute;
use crossterm::terminal::{self, ClearType};
use std::io::stdout;
pub struct Output {
    win_size: (usize, usize),
}

impl Output {
    pub fn new() -> Self {
        let win_size = terminal::size()
            .map(|(x, y)| (x as usize, y as usize))
            .unwrap();
        Self { win_size }
    }

    pub fn clear_screen() -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }
    fn draw_rows(&self) {
        let screen_rows = self.win_size.1; /* add this line */
        for _ in 0..screen_rows {
            /* modify */
            println!("~\r");
        }
    }

    pub fn refresh_screen(&self) -> crossterm::Result<()> {
        Self::clear_screen()?;
        self.draw_rows();
        execute!(stdout(), cursor::MoveTo(0, 0))
    }
}
