use crate::visor::{Coords, terminal_writer::RootedRenderer};

use super::{terminal_writer::TerminalBackend, Component};

pub struct TextView {
    contents: String,
}

impl<S: ToString> From<S> for TextView {
    fn from(s: S) -> Self {
        TextView::new(s.to_string())
    }
}

impl TextView {
    pub fn new(contents: String) -> Self {
        Self { contents }
    }
}

impl Component for TextView {
    fn render(&self, writer: &mut dyn TerminalBackend) {
        let mut iter = self.contents.lines().peekable();

        while let Some(line) = iter.next() {
            writer.write(line);
            if iter.peek().is_some() {
                writer.move_root((0, 1).into());
                writer.reset_cursor();
            }
        }
    }

    fn declare_dimensions(&self) -> (u16, u16) {
        let lines = self.contents.lines().count();
        let length = self.contents.lines().fold(0, |acc, next| {
            if acc < next.len() {
                return next.len();
            }
            acc
        });

        (length as u16, lines as u16)
    }
}

pub enum PanelDimensions {
    Static(u16, u16),
    ShrinkWrap,
}

pub struct Panel {
    name: String,
    padding: u8,
    dimensions: PanelDimensions,
    component: Box<dyn Component>,
}

impl Panel {
    pub fn new(name: String, padding: u8, dimensions: PanelDimensions, component: Box<dyn Component>) -> Self {
        if let PanelDimensions::Static(w, h) = dimensions {
            let (c_w, c_h) = component.declare_dimensions();
            if c_w < w || c_h < h {
                panic!("You cannot make a panel with a static size ({}x{}), that is smaller than the enclosing component {}x{}", w, h, c_w, c_h);
            }
        }
        Self {
            name,
            padding,
            dimensions,
            component,
        }
    }
}

impl Component for Panel {
    fn declare_dimensions(&self) -> (u16, u16) {
        let base = match self.dimensions {
            PanelDimensions::Static(w, h) => (w, h),
            PanelDimensions::ShrinkWrap => self.component.declare_dimensions(),
        };
        (base.0 + 4, base.1 + 4)
    }

    fn handle(&mut self, e: &super::UserInput) -> super::UserEventHandled {
        self.component.handle(e)
    }

    fn render(&self, writer: &mut dyn TerminalBackend) {
        let padding = 1;
        let (w, h) = self.declare_dimensions();
        writer.write("┌");
        writer.write(&"─".repeat((w - 2).into()));
        writer.write("┐");
        let subroot = writer.get_root() + Coords(1 + padding, 1);
        let mut rooted = RootedRenderer::new(writer, subroot);
        rooted.reset_cursor();
        self.component.render(&mut rooted);
        for i in 2..(h-2) {
            writer.set_cursor_to(Coords(1, i));
            writer.write("│");
            writer.set_cursor_to(Coords(w, i));
            writer.write("│");
        }
        writer.set_cursor_to(Coords(1, h-2));
        writer.write("└");
        writer.write(&"─".repeat((w - 2).into()));
        writer.write("┘");
    }
}
