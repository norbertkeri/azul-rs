use super::{renderer::RootedRenderer, Component, Coords, UserEventHandled};

pub type Components<'a> = Vec<Box<dyn Component + 'a>>;

pub struct Layout<'a> {
    direction: Direction,
    components: Components<'a>,
}

impl<'a> From<Layout<'a>> for Box<dyn Component + 'a> {
    fn from(s: Layout<'a>) -> Self {
        Box::new(s)
    }
}

impl<'a> Layout<'a> {
    pub fn new(direction: Direction, components: Components<'a>) -> Self {
        Self {
            direction,
            components,
        }
    }

    pub fn horizontal(components: Components<'a>) -> Self {
        Self {
            direction: Direction::Horizontal,
            components,
        }
    }

    pub fn vertical(components: Components<'a>) -> Self {
        Self {
            direction: Direction::Vertical,
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
            let name = match self.direction {
                Direction::Horizontal => "layout-horizontal",
                Direction::Vertical => "layout-vertical",
            };
            let node_id = renderer.render_into_layer(name, dims, component.as_ref());

            let just_rendered_area = renderer.get_drawn_area(node_id);
            let move_root_by: Coords = match self.direction {
                Direction::Horizontal => (just_rendered_area.0, 0).into(),
                Direction::Vertical => (0, just_rendered_area.1).into(),
            };
            dims = move_root_by + dims;
        }
    }

    fn handle(&mut self, _event: &super::UserInput) -> UserEventHandled {
        UserEventHandled::Noop
    }
}
