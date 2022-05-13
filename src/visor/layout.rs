use std::cmp::max;

use super::{terminal_writer::RootedRenderer, Component, Coords, UserEventHandled};

pub type Components<'a> = Vec<Box<dyn Component + 'a>>;

pub struct Layout<'a> {
    direction: Direction,
    padding: u8,
    components: Components<'a>,
}

impl<'a> From<Layout<'a>> for Box<dyn Component + 'a> {
    fn from(s: Layout<'a>) -> Self {
        Box::new(s)
    }
}

impl<'a> Layout<'a> {
    pub fn new(direction: Direction, padding: u8, components: Components<'a>) -> Self {
        Self {
            direction,
            padding,
            components,
        }
    }

    pub fn horizontal(padding: u8, components: Components<'a>) -> Self {
        Self {
            direction: Direction::Horizontal,
            padding,
            components,
        }
    }

    pub fn vertical(padding: u8, components: Components<'a>) -> Self {
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

impl<'a> Component for Layout<'a> {
    fn render(&self, renderer: &mut RootedRenderer) {
        let mut dims = Coords(0, 0);
        for component in self.components.iter() {
            let mut rooted = RootedRenderer::subrooted(renderer, dims);
            rooted.reset_cursor();
            component.render(&mut rooted);
            let dimensions = component.declare_dimensions();
            let move_root_by: Coords = match self.direction {
                Direction::Horizontal => (dimensions.0, 0).into(),
                Direction::Vertical => (0, dimensions.1).into(),
            };
            dims = dims + move_root_by;
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
        UserEventHandled::Noop
    }
}
