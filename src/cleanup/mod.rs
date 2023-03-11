use crossterm::terminal;
pub struct Clean_up {}

impl Clean_up {
    // add code here
    pub fn drop(&mut self) {
        terminal::disable_raw_mode().expect("Unable to disable raw mode")
    }
}
