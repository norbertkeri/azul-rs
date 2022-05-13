use std::rc::Rc;
use crate::visor::{Component, layout::Layout, UserInput, UserEventHandled};
use super::{Tile, Factory, Game, AppEvent};
use crate::visor::view::PanelBuilder;

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

    fn handle(&mut self, _event: &UserInput) -> UserEventHandled {
        UserEventHandled::Noop
    }
}

pub struct FactoryView {
    factory: Rc<Factory>,
    selected_tile: Option<Tile>
}

impl FactoryView {
    pub fn new(factory: Rc<Factory>, selected_tile: Option<Tile>) -> Self { Self { factory, selected_tile } }
    fn has_selected_tile(&self) -> bool {
        matches!(self.selected_tile, Some(tile) if self.factory.as_ref().0.contains(&tile))
    }
}

impl From<FactoryView> for Box<dyn Component> {
    fn from(s: FactoryView) -> Self {
        Box::new(s)
    }
}

impl Component for FactoryView {
    fn render(&self, writer: &mut dyn crate::visor::terminal_writer::TerminalBackend) {
        let mut began_selection = false;
        let mut iter = self.factory.get_tiles().iter().peekable();
        while let Some(t) = iter.next() {
            if !began_selection && matches!(self.selected_tile, Some(selected_tile) if &selected_tile == t) {
                writer.write("|");
                began_selection = true;
            }
            TileView::new(*t, false).render(writer);
            if began_selection {
                let selected_tile = self.selected_tile.unwrap();
                let render_closing = match iter.peek() {
                    Some(next_tile) if *next_tile != &selected_tile => true,
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

    fn handle(&mut self, _event: &UserInput) -> UserEventHandled {
        UserEventHandled::Noop
    }
}

pub enum FactoryAreaState {
    Passive,
    SelectFactory(usize), // TODO name this as currently_selected?
    SelectTile(usize) // TODO same here?
}

struct FactoryAreaView {
    factories: Vec<Rc<Factory>>,
    state: FactoryAreaState
}

impl Component for FactoryAreaView {
    fn render(&self, writer: &mut dyn crate::visor::terminal_writer::TerminalBackend) {
        let factory_views: Vec<_> = self.factories.iter().map(|f| {
            let view = FactoryView::new(f.clone(), None);
            Box::new(view) as Box<dyn Component>
        }).collect();

        let panel = PanelBuilder::default()
            .name("Factories")
            .padding(1)
            .component(Box::new(Layout::vertical(0, factory_views)))
            .build()
            .unwrap();

        panel.render(writer);
    }

    fn declare_dimensions(&self) -> (u16, u16) {
        (6, 4)
    }
}

pub struct GameView<const N: usize> {
    pub game: Rc<Game<N>>
}

impl<const N: usize> Component for GameView<N> {
    fn render(&self, writer: &mut dyn crate::visor::terminal_writer::TerminalBackend) {
        let factory_state = match self.game.state {
            crate::model::GameState::PickFactory { current_factory, .. } => {
                FactoryAreaState::SelectFactory(current_factory)
            }
        };
        let factories: Vec<_> = self.game.get_factories().to_vec();
        let factory_area = FactoryAreaView {
            state: factory_state,
            factories
        };
        factory_area.render(writer);
    }

    fn declare_dimensions(&self) -> (u16, u16) {
        (100, 50)
    }

    fn handle(&mut self, e: &UserInput) -> UserEventHandled {
        match e {
            UserInput::Character(_c) => match self.game.state {
                super::GameState::PickFactory { .. } => {
                    UserEventHandled::AppEvent(AppEvent::SelectNext)
                }
            },
            UserInput::Confirm | UserInput::Back => todo!(),
        }
    }

}
