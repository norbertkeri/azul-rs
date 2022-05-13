use std::rc::Rc;

use crate::visor::Component;

use super::{Tile, Factory};

pub struct TileView {
    pub tile: Tile,
    pub selected: bool
}

impl From<TileView> for Box<dyn Component> {
    fn from(s: TileView) -> Self {
        Box::new(s)
    }
}

impl TileView {
    pub fn new(tile: Tile, selected: bool) -> Self { Self { tile, selected } }
}

impl Component for TileView {
    fn render(&self, writer: &mut dyn crate::visor::terminal_writer::TerminalBackend) {
        let s = if self.selected {
            format!("|{}|", self.tile.to_string())
        } else {
            self.tile.to_string()
        };
        writer.write(&s);
    }

    fn declare_dimensions(&self) -> (u16, u16) {
        if self.selected {
            (3, 1)
        } else {
            (1, 1)
        }
    }

    fn handle(&mut self, _event: &crate::visor::UserInput) -> crate::visor::UserEventHandled {
        crate::visor::UserEventHandled::Noop
    }
}

pub struct FactoryView {
    factory: Rc<Factory>,
    selected_tile: Option<Tile>
}

impl FactoryView {
    pub fn new(factory: Rc<Factory>, selected_tile: Option<Tile>) -> Self { Self { factory, selected_tile } }
}

impl From<FactoryView> for Box<dyn Component> {
    fn from(s: FactoryView) -> Self {
        Box::new(s)
    }
}

impl Component for FactoryView {
    fn render(&self, writer: &mut dyn crate::visor::terminal_writer::TerminalBackend) {
        let f = self.factory.as_ref();
        let mut tiles = f.0;
        tiles.sort();
        let mut began_selection = false;
        let mut iter = tiles.into_iter().peekable();
        while let Some(t) = iter.next() {
            if !began_selection && matches!(self.selected_tile, Some(selected_tile) if selected_tile == t) {
                writer.write("|");
                began_selection = true;
            }
            TileView::new(t, false).render(writer);
            if began_selection {
                let selected_tile = self.selected_tile.unwrap();
                let render_closing = match iter.peek() {
                    Some(next_tile) if next_tile != &selected_tile => true,
                    None => true,
                    _ => false
                };
                if render_closing {
                    writer.write("|");
                    began_selection = false;
                }
            }
        }
    }

    fn declare_dimensions(&self) -> (u16, u16) {
        if self.has_selected_tile() {
            return (6, 1);
        }
        (4, 1)
    }

    fn handle(&mut self, _event: &crate::visor::UserInput) -> crate::visor::UserEventHandled {
        crate::visor::UserEventHandled::Noop
    }
}

impl FactoryView {
    fn has_selected_tile(&self) -> bool {
        matches!(self.selected_tile, Some(tile) if self.factory.as_ref().0.contains(&tile))
    }
}
