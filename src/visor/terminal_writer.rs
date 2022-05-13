use std::io::Write;

use super::Coords;

pub trait TerminalBackend {
    fn clear(&self, sink: &mut impl Write);
    fn move_cursor(&mut self, coords: Coords, sink: &mut impl Write);
    fn write(&self, text: &str, sink: &mut impl Write);
}

#[derive(Default)]
pub struct TermionBackend {}

impl TerminalBackend for TermionBackend {
    fn clear(&self, stdout: &mut impl Write) {
        write!(
            stdout,
            "{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
        )
        .unwrap();
    }

    fn move_cursor(&mut self, coords: Coords, sink: &mut impl Write) {
        write!(sink, "{}", termion::cursor::Goto(coords.0, coords.1),).unwrap();
    }

    fn write(&self, text: &str, sink: &mut impl Write) {
        write!(sink, "{}", text).unwrap();
    }
}

pub struct TerminalWriter<T: TerminalBackend> {
    backend: T,
}

impl<T: TerminalBackend> TerminalWriter<T> {
    pub fn new(backend: T) -> Self {
        Self { backend }
    }

    pub fn move_to(&mut self, coords: Coords, sink: &mut impl Write) {
        self.backend.move_cursor(coords, sink);
    }

    pub fn with_backend(backend: T) -> Self {
        Self { backend }
    }

    pub fn clear(&mut self, sink: &mut impl Write) {
        self.backend.clear(sink);
        self.backend.move_cursor(Coords(1, 1), sink);
    }

    pub fn write(&self, text: &str, sink: &mut impl Write) {
        self.backend.write(text, sink);
    }
}
