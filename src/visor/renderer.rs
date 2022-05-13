use super::{Coords, backend::TerminalBackend};

trait Renderer {
    fn write(&mut self, s: &str);
    fn set_cursor_to(&mut self, coords: Coords);
    fn reset_cursor_to_root(&mut self);
}

pub struct RootedRenderer<'a> {
    writer: &'a mut dyn TerminalBackend,
    root: Coords,
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
        }
    }

    pub fn write(&mut self, s: &str) {
        /*
        let Coords(x, y) = self.cursor;
        let lines = s.lines().count() as u16;
        let longest_line =
            s.lines()
                .map(|x| x.chars().count())
                .reduce(|longest, next| if next > longest { next } else { longest })
                .unwrap() as u16;

        self.drawn_area = (
            max(self.drawn_area.0, (x-1) + longest_line),
            max(self.drawn_area.1, (y-1) + lines),
        );
        */
        self.writer.write(s);
    }

    pub fn set_cursor_to(&mut self, coords: Coords) {
        let new_coords = self.root + coords;
        self.writer.set_cursor_to(new_coords);
    }

    pub fn reset_cursor_to_root(&mut self) {
        self.writer.set_cursor_to(self.root);
    }
}
