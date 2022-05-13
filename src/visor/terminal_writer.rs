use std::io::Write;

use super::{Coords, Engine};

pub trait TerminalBackend {
    fn clear(&mut self, sink: &mut impl Write);
    fn move_cursor(&mut self, coords: Coords, sink: &mut impl Write);
    fn write(&mut self, text: &str, sink: &mut impl Write);
}

#[derive(Default)]
pub struct TermionBackend {}

impl TerminalBackend for TermionBackend {
    fn clear(&mut self, stdout: &mut impl Write) {
        write!(
            stdout,
            "{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
        )
        .unwrap();
    }

    fn move_cursor(&mut self, coords: Coords, sink: &mut impl Write) {
        write!(sink, "{}", termion::cursor::Goto(coords.0, coords.1)).unwrap();
    }

    fn write(&mut self, text: &str, sink: &mut impl Write) {
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

    pub fn write(&mut self, text: &str, sink: &mut impl Write) {
        self.backend.write(text, sink);
    }
}

pub struct TestBackend {
    cursor: (usize, usize),
    screen: Vec<String>,
}

impl Default for TestBackend {
    fn default() -> Self {
        Self {
            cursor: (1, 1),
            screen: Default::default(),
        }
    }
}

impl TerminalBackend for TestBackend {
    fn clear(&mut self, _sink: &mut impl std::io::Write) {
        self.screen = vec![];
        self.cursor = (1, 1);
    }

    fn move_cursor(&mut self, coords: Coords, _sink: &mut impl std::io::Write) {
        self.cursor = coords.into();
    }

    fn write(&mut self, text: &str, _sink: &mut impl std::io::Write) {
        let (x, y) = self.cursor;
        for i in 0..=y {
            if self.screen.get(i).is_none() {
                self.screen.push(String::new());
            }
            if i == y - 1 {
                let new_width = x - 1 + text.chars().count();
                let mut new_string = format!("{:width$}", &self.screen[i], width = new_width);

                let replace_at = new_string
                    .char_indices()
                    .nth(x - 1)
                    .map(|(pos, ch)| (pos..pos + ch.len_utf8()))
                    .unwrap();

                new_string.replace_range(replace_at, text);
                self.screen[i] = new_string.trim_end().to_string();
                self.cursor = (x + text.chars().count(), y);
            }
        }
    }
}

impl TerminalWriter<TestBackend> {
    pub fn get_contents(&self) -> String {
        self.backend.screen.join("\n").trim_end().to_string()
    }
}

impl<'a> Engine<'a, TestBackend> {
    pub fn get_contents(&self) -> String {
        self.writer.get_contents()
    }
}
