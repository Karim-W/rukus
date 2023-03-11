use crate::content;
use crate::cursors;
use crate::rows;
use crossterm::cursor;
use crossterm::execute;
use crossterm::queue;
use crossterm::terminal::{self, ClearType};
use std::cmp;
use std::io::stdout;
use std::io::Write;

pub struct Output {
    win_size: (usize, usize),
    content: content::Content,
    cursor: cursors::Cursor,
    e_rows: rows::Rows,
}

impl Output {
    pub fn new() -> Self {
        let win_size = terminal::size()
            .map(|(x, y)| (x as usize, y as usize))
            .unwrap();
        Self {
            win_size,
            content: content::Content::new(),
            cursor: cursors::Cursor::new(),
            e_rows: rows::Rows::new(),
        }
    }

    pub fn clear_screen(&self) -> crossterm::Result<()> {
        execute!(stdout(), terminal::Clear(ClearType::All))?;
        execute!(stdout(), cursor::MoveTo(0, 0))
    }
    fn draw_rows(&mut self) {
        let screen_rows = self.win_size.1;
        let screen_columns = self.win_size.0;
        for i in 0..screen_rows {
            if i >= self.e_rows.rows_length() && self.e_rows.rows_length() == 0 {
                /* add this line */
                if i == screen_rows / 2 {
                    let mut welcome = format!("----- rukus v0.0.1 -------");
                    if welcome.len() > screen_columns {
                        welcome.truncate(screen_columns)
                    }
                    let mut padding = (screen_columns - welcome.len()) / 2;
                    if padding != 0 {
                        self.content.push('~');
                        padding -= 1
                    }
                    (0..padding).for_each(|_| self.content.push(' '));
                    self.content.push_str(&welcome);
                } else {
                    self.content.push('~');
                }
                /* add the following*/
            } else {
                let len = cmp::min(self.e_rows.serve(i).len(), screen_columns);
                self.content.push_str(&self.e_rows.serve(i)[..len])
            }
            /* end */
            queue!(self.content, terminal::Clear(ClearType::UntilNewLine)).unwrap();
            if i < screen_rows - 1 {
                self.content.push_str("\r\n");
            }
        }
    }

    pub fn handle_create_line(&mut self) {
        self.e_rows.create_line();
        self.cursor.move_cursor(cursors::CursorDirection::Down);
        self.cursor.x = 0;
    }

    pub fn insert_row(&mut self, chara: char) {
        self.e_rows.push(self.cursor.y, chara.to_string().as_str());
        self.cursor.move_cursor(cursors::CursorDirection::Right);
    }

    pub fn refresh_screen(&mut self) -> crossterm::Result<()> {
        queue!(
            self.content,
            cursor::Hide,
            // terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )?;
        self.draw_rows();
        let nx = self.cursor.x;
        let ny = self.cursor.y;
        queue!(self.content, cursor::MoveTo(nx, ny), cursor::Show)?;
        self.content.flush()
    }

    pub fn move_cursor(&mut self, dir: char) {
        let dir = cursors::CursorDirection::from_char(dir);
        if dir.is_none() {
            return;
        }
        let dir = dir.unwrap();
        match dir {
            cursors::CursorDirection::Up => {
                if self.cursor.y == 0 {
                    return;
                }
                self.cursor.y -= 1;
            }
            cursors::CursorDirection::Down => {
                if self.cursor.y == self.e_rows.rows_length() as u16 - 1 {
                    return;
                }
                self.cursor.y += 1;
            }
            cursors::CursorDirection::Left => {
                if self.cursor.x == 0 {
                    return;
                }
                self.cursor.x -= 1;
            }
            cursors::CursorDirection::Right => {
                if self.cursor.x == self.e_rows.serve(self.cursor.y as usize).len() as u16 {
                    return;
                }
                self.cursor.x += 1;
            }
        }
    }
}
