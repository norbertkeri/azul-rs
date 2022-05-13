use super::{Component, UserEventHandled};

struct Layout<C: Component> {
    direction: Direction,
    padding: u8,
    components: Vec<C>,
}

impl<C: Component> Layout<C> {
    pub fn new(direction: Direction, padding: u8, components: Vec<C>) -> Self {
        Self {
            direction,
            padding,
            components,
        }
    }

    pub fn horizontal(padding: u8, components: Vec<C>) -> Self {
        Self {
            direction: Direction::Horizontal,
            padding,
            components,
        }
    }

    pub fn vertical(padding: u8, components: Vec<C>) -> Self {
        Self {
            direction: Direction::Vertical,
            padding,
            components,
        }
    }
}

enum Direction {
    Horizontal,
    Vertical,
}

impl<C: Component> Component for Layout<C> {
    fn render(&self) -> String {
        for component in self.components.iter() {
            for _line in component.render().lines() {
                //self.writer.write(line, sink);
            }
        }
        self.components
            .iter()
            .map(Component::render)
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn declare_dimensions(&self) -> (u16, u16) {
        self.components.iter().fold((0, 0), |acc, next_component| {
            let next = next_component.declare_dimensions();
            // TODO padding should be added, depending on h/v
            (acc.0 + next.0, acc.1 + next.1)
        })
    }

    fn handle(&mut self, _event: &super::UserInput) -> UserEventHandled {
        super::UserEventHandled::Noop
    }
}
