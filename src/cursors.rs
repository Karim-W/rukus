use crossterm::event::KeyCode;
pub struct Cursor {
    pub x: u16,
    pub y: u16,
}

pub enum CursorDirection {
    Up = 'w' as isize,
    Down = 's' as isize,
    Left = 'a' as isize,
    Right = 'd' as isize,
}

impl Cursor {
    pub fn new() -> Cursor {
        Self { x: 0, y: 0 }
    }
    pub fn move_cursor(&mut self, dir: CursorDirection) {
        match dir {
            CursorDirection::Down => {
                self.y += 1;
            }
            CursorDirection::Up => {
                if self.y == 0 {
                    return;
                }
                self.y -= 1;
            }
            CursorDirection::Left => {
                if self.x == 0 {
                    return;
                }
                self.x -= 1;
            }
            CursorDirection::Right => {
                self.x += 1;
            }
        }
    }
}

impl CursorDirection {
    pub fn from_char(chara: char) -> Option<CursorDirection> {
        match chara {
            'w' => Some(CursorDirection::Up),
            's' => Some(CursorDirection::Down),
            'a' => Some(CursorDirection::Left),
            'd' => Some(CursorDirection::Right),
            _ => None,
        }
    }
    pub fn from_key_code(code: KeyCode) -> Option<CursorDirection> {
        match code {
            KeyCode::Char('w') => Some(CursorDirection::Up),
            KeyCode::Char('s') => Some(CursorDirection::Down),
            KeyCode::Char('a') => Some(CursorDirection::Left),
            KeyCode::Char('d') => Some(CursorDirection::Right),
            KeyCode::Up => Some(CursorDirection::Up),
            KeyCode::Down => Some(CursorDirection::Down),
            KeyCode::Left => Some(CursorDirection::Left),
            KeyCode::Right => Some(CursorDirection::Right),
            _ => None,
        }
    }
}
