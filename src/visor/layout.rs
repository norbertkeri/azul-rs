use std::cmp::max;

use super::{
    terminal_writer::{RootedRenderer, TerminalBackend},
    Component, UserEventHandled,
};

pub type Components = Vec<Box<dyn Component>>;

pub struct Layout {
    direction: Direction,
    padding: u8,
    components: Components,
}

impl Layout {
    pub fn new(direction: Direction, padding: u8, components: Components) -> Self {
        Self {
            direction,
            padding,
            components,
        }
    }

    pub fn horizontal(padding: u8, components: Components) -> Self {
        Self {
            direction: Direction::Horizontal,
            padding,
            components,
        }
    }

    pub fn vertical(padding: u8, components: Components) -> Self {
        Self {
            direction: Direction::Vertical,
            padding,
            components,
        }
    }
}

pub enum Direction {
    Horizontal,
    Vertical,
}

impl Component for Layout {
    fn render(&self, writer: &mut dyn TerminalBackend) {
        for component in self.components.iter() {
            writer.reset_cursor();
            let subroot = writer.get_root();
            let mut rooted = RootedRenderer::new(writer, subroot);
            component.render(&mut rooted);
            let dimensions = component.declare_dimensions();
            let move_root_by = match self.direction {
                Direction::Horizontal => (dimensions.0, 0).into(),
                Direction::Vertical => (0, dimensions.1).into(),
            };
            writer.move_root(move_root_by);
        }
    }

    fn declare_dimensions(&self) -> (u16, u16) {
        self.components.iter().fold((0, 0), |acc, next_component| {
            let next = next_component.declare_dimensions();
            match self.direction {
                Direction::Horizontal => {
                    let length = acc.0 + next.0;
                    let height = max(acc.1, next.1);
                    (length, height)
                }
                Direction::Vertical => {
                    let length = max(acc.0, next.0);
                    let height = acc.1 + next.1;
                    (length, height)
                }
            }
        })
    }

    fn handle(&mut self, _event: &super::UserInput) -> UserEventHandled {
        todo!()
    }
}
