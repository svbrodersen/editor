use crossterm::cursor::MoveTo;
use crossterm::execute;
use crossterm::terminal::{Clear, ClearType, disable_raw_mode, enable_raw_mode, size};
use std::io::Error;
use std::io::stdout;

pub struct Terminal {}

impl Terminal {
    pub fn terminate() -> Result<(), Error> {
        disable_raw_mode()
    }
    pub fn clear_screen() -> Result<(), Error> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All))
    }
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(0, 0)
    }
    pub fn move_cursor_to(x: u16, y: u16) -> Result<(), Error> {
        execute!(stdout(), MoveTo(x, y))
    }
    pub fn size() -> Result<(u16, u16), Error> {
        size()
    }
}
