use std::io::Write;

use super::Coords;

pub struct RootedRenderer<'a> {
    writer: &'a mut dyn TerminalBackend,
    root: Coords,
    drawn_area: (u16, u16),
}

impl<'a> RootedRenderer<'a> {
    pub fn subrooted(root: &'a mut RootedRenderer, shift_root: Coords) -> Self {
        Self::new(root.writer, root.root + shift_root)
    }

    pub fn get_root(&self) -> Coords {
        self.root
    }

    pub fn new(writer: &'a mut dyn TerminalBackend, root: Coords) -> Self {
        Self {
            writer,
            root,
            drawn_area: (0, 0),
        }
    }

    pub fn get_drawn_area(&self) -> (u16, u16) {
        self.drawn_area
    }

    pub fn write(&mut self, s: &str) {
        self.writer.write(s);
    }

    pub fn set_cursor_to(&mut self, coords: Coords) {
        let new_coords = self.root + coords;
        self.writer.set_cursor_to(new_coords);
    }

    pub fn reset_cursor(&mut self) {
        self.writer.set_cursor_to(self.root);
    }
}

pub trait TerminalBackend {
    fn clear(&mut self);
    fn set_cursor_to(&mut self, coords: Coords);
    fn write(&mut self, text: &str);
    fn flush(&mut self);
}

pub trait DebuggableTerminalBackend: TerminalBackend {
    fn get_contents(&self) -> String;
}

pub struct TermionBackend {
    sink: Box<dyn Write>,
}

impl TermionBackend {
    pub fn new(sink: Box<dyn Write>) -> Self {
        Self { sink }
    }
}

impl TerminalBackend for TermionBackend {
    fn flush(&mut self) {
        self.sink.flush().unwrap();
    }

    fn clear(&mut self) {
        write!(
            self.sink,
            "{}{}",
            termion::clear::All,
            termion::cursor::Goto(1, 1),
        )
        .unwrap();
    }

    fn set_cursor_to(&mut self, coords: Coords) {
        write!(self.sink, "{}", termion::cursor::Goto(coords.0, coords.1)).unwrap();
    }

    fn write(&mut self, text: &str) {
        write!(self.sink, "{}", text).unwrap();
    }
}

pub struct TestBackend {
    cursor: Coords,
    screen: Vec<String>,
}

impl Default for TestBackend {
    fn default() -> Self {
        Self {
            cursor: (1, 1).into(),
            screen: Default::default(),
        }
    }
}

impl TerminalBackend for TestBackend {
    fn clear(&mut self) {
        self.screen = vec![];
        self.cursor = (1, 1).into();
    }

    fn set_cursor_to(&mut self, coords: Coords) {
        self.cursor = coords;
    }

    fn write(&mut self, text: &str) {
        let (x, y): (usize, usize) = (self.cursor.0.into(), self.cursor.1.into());
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
                let casted: (u16, u16) = (
                    (x + text.chars().count()).try_into().unwrap(),
                    y.try_into().unwrap(),
                );
                self.cursor = casted.into();
            }
        }
    }

    fn flush(&mut self) {}
}

impl DebuggableTerminalBackend for TestBackend {
    fn get_contents(&self) -> String {
        self.screen.join("\n").trim_end().to_string()
    }
}
