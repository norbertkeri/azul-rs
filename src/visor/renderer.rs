use indextree::{Arena, NodeId};
use std::cmp::max;

use super::{backend::TerminalBackend, Component, Coords};

#[derive(Debug, PartialEq)]
struct Layer {
    name: String,
    root: Coords,
    drawn_area: (u16, u16),
    cursor: Coords,
}

impl Layer {
    pub fn default_with_name<T: ToString>(name: T) -> Self {
        Self {
            name: name.to_string(),
            root: Coords::new(0, 0),
            drawn_area: Default::default(),
            cursor: Coords::default(),
        }
    }

    pub fn with_name_and_root<T: ToString>(name: T, root: Coords) -> Self {
        let mut me = Self::default_with_name(name);
        me.root = root;
        me
    }
}

pub struct RootedRenderer<'a> {
    components: Arena<Layer>,
    writer: &'a mut dyn TerminalBackend,
    active_layer_id: NodeId,
}

impl<'a> RootedRenderer<'a> {
    pub fn push_layer<T: ToString>(&mut self, name: T, root: Coords) -> NodeId {
        let new_node = Layer::with_name_and_root(name, root);
        let new_node_id = self.components.new_node(new_node);
        self.active_layer_id
            .append(new_node_id, &mut self.components);
        self.active_layer_id = new_node_id;
        self.reset_cursor_to_root();
        new_node_id
    }

    pub fn render_into_layer<T: ToString>(
        &mut self,
        name: T,
        root: Coords,
        component: &dyn Component,
    ) -> NodeId {
        let node_id = self.push_layer(name, root);
        component.render(self);
        self.pop_layer();
        node_id
    }

    fn active_layer(&self) -> &Layer {
        self.components.get(self.active_layer_id).unwrap().get()
    }

    fn get_absolute_cursor(&self) -> Coords {
        let absolute_root = self.active_layer_id.ancestors(&self.components).fold(
            Coords::new(0, 0),
            |acc, next| {
                let node = self.components.get(next).unwrap();
                acc + node.get().root
            },
        );
        absolute_root + self.active_layer().cursor
    }

    pub fn pop_layer(&mut self) {
        self.active_layer_id = self
            .active_layer_id
            .ancestors(&self.components)
            .nth(1)
            .unwrap();
        self.reset_cursor_to_root();
    }

    pub fn get_drawn_area_for_active_layer(&self) -> (u16, u16) {
        self.get_drawn_area(self.active_layer_id)
    }

    fn calculate_total_area(&self, node_id: NodeId, roots_above: (u16, u16)) -> (u16, u16) {
        let myself = self.components.get(node_id).unwrap().get();
        let my_area = (
            myself.drawn_area.0 + roots_above.0,
            myself.drawn_area.1 + roots_above.1,
        );

        let result = node_id
            .children(&self.components)
            .fold(my_area, |acc, next_id| {
                let next_node = self.components.get(next_id).unwrap().get();
                let next_roll = (
                    next_node.root.0 + roots_above.0,
                    next_node.root.1 + roots_above.1,
                );
                let next_area = self.calculate_total_area(next_id, next_roll);
                (max(acc.0, next_area.0), max(acc.1, next_area.1))
            });

        result
    }

    pub fn get_drawn_area(&self, node_id: NodeId) -> (u16, u16) {
        self.calculate_total_area(node_id, (0, 0))
    }

    pub fn default_with_writer(writer: &'a mut dyn TerminalBackend) -> Self {
        let mut nodes = Arena::new();
        let root = nodes.new_node(Layer::default_with_name("root"));

        Self {
            writer,
            components: nodes,
            active_layer_id: root,
        }
    }

    pub fn write(&mut self, s: &str) {
        if s.is_empty() {
            return;
        }
        let active_node = self.get_active_layer_mut();
        let Coords(x, y) = &mut active_node.cursor;
        let lines_drawn = s.lines().count() as u16 - 1;
        let longest_line = s
            .lines()
            .map(|x| x.chars().count())
            .reduce(|longest, next| max(next, longest))
            .unwrap() as u16;

        let top_drawn_area = &mut active_node.drawn_area;

        *top_drawn_area = (
            max(top_drawn_area.0, longest_line + *x - 1),
            max(top_drawn_area.1, lines_drawn + *y),
        );

        if let Some(last_line) = s.lines().last() {
            *y += lines_drawn;
            let new_x: u16 = last_line.chars().count().try_into().unwrap();
            *x += new_x;
        }

        self.writer.write(s);
    }

    fn get_active_layer_mut(&mut self) -> &mut Layer {
        self.components
            .get_mut(self.active_layer_id)
            .unwrap()
            .get_mut()
    }

    fn get_active_layer(&self) -> &Layer {
        self.components.get(self.active_layer_id).unwrap().get()
    }

    pub fn set_cursor_to(&mut self, coords: Coords) {
        assert!(coords.0 > 0 && coords.1 > 0);
        let node = self.get_active_layer_mut();
        node.cursor = coords;
        self.writer.set_cursor_to(self.get_absolute_cursor());
    }

    pub fn move_cursor(&mut self, coords: Coords) {
        let cursor = self.get_active_layer().cursor;
        self.set_cursor_to(coords + cursor);
    }

    pub fn reset_cursor_to_root(&mut self) {
        let node = self.get_active_layer_mut();
        node.cursor = Default::default();
        self.writer.set_cursor_to(self.get_absolute_cursor());
    }
}

#[cfg(test)]
mod tests {
    use crate::visor::{backend::TestBackend, renderer::RootedRenderer, Coords};
    use test_case::test_case;

    #[test_case("", (0, 0))]
    #[test_case("x", (1, 1))]
    #[test_case("Hello", (5, 1))]
    #[test_case("Hello\nbye", (5, 2))]
    #[test_case("Hello\nlongworld\nbye", (9, 3))]
    pub fn test_drawn_area_raw(text: &str, expected_area: (u16, u16)) {
        let mut backend = TestBackend::default();
        let mut writer = RootedRenderer::default_with_writer(&mut backend);
        writer.write(text);
        assert_eq!(writer.get_drawn_area_for_active_layer(), expected_area);
    }

    #[test]
    pub fn test_move_cursor() {
        let mut backend = TestBackend::default();
        let mut writer = RootedRenderer::default_with_writer(&mut backend);
        writer.move_cursor(Coords::new(2, 2));
        writer.write("Hello");
        assert_eq!(writer.get_drawn_area_for_active_layer(), (7, 3));
    }

    #[test]
    pub fn test_subrenders() {
        let mut backend = TestBackend::default();
        let mut writer = RootedRenderer::default_with_writer(&mut backend);
        writer.push_layer("t1", Coords::new(3, 0));
        writer.push_layer("t2", Coords::new(0, 3));
        assert_eq!(writer.get_absolute_cursor(), Coords::new(4, 4));
    }
}
