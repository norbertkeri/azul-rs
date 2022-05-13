use self::player::PlayerAreaView;

use super::{AppEvent, Factory, Game, Tile};
use crate::visor::view::PanelBuilder;
use crate::{
    model::GameState,
    visor::{layout::Layout, Component, UserEventHandled, UserInput},
};
use std::{cell::RefCell, rc::Rc};

pub mod player;

#[derive(Debug)]
pub struct TileView {
    pub tile: Tile,
    pub selected: bool,
}

impl From<TileView> for Box<dyn Component> {
    fn from(s: TileView) -> Self {
        Box::new(s)
    }
}

impl TileView {
    pub fn new(tile: Tile, selected: bool) -> Self {
        Self { tile, selected }
    }
}

impl Component for TileView {
    fn render(&self, writer: &mut dyn crate::visor::terminal_writer::TerminalBackend) {
        let s = if self.selected {
            format!("|{}|", self.tile)
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

pub struct FactoryView<'a> {
    factory: &'a Factory,
    selected_tile: Option<Tile>,
    is_selected: bool,
}

impl<'a> FactoryView<'a> {
    pub fn new(factory: &'a Factory, selected_tile: Option<Tile>, is_selected: bool) -> Self {
        Self {
            factory,
            selected_tile,
            is_selected,
        }
    }

    fn has_selected_tile(&self) -> bool {
        match (self.factory.get_tiles(), self.selected_tile) {
            (Some(tiles), Some(selected_tile)) => tiles.contains(&selected_tile),
            _ => false
        }
        //matches!(self.selected_tile, Some(tile) if self.factory.0.contains(&tile))
    }
}

impl<'a> From<FactoryView<'a>> for Box<dyn Component + 'a> {
    fn from(s: FactoryView<'a>) -> Self {
        Box::new(s)
    }
}

impl Component for FactoryView<'_> {
    fn render(&self, writer: &mut dyn crate::visor::terminal_writer::TerminalBackend) {
        let mut began_selection = false;
        if let Some(tiles) = self.factory.get_tiles() {
            let mut iter = tiles.iter().peekable();
            if self.is_selected {
                writer.write("--> ");
            }
            while let Some(t) = iter.next() {
                if !began_selection
                    && matches!(self.selected_tile, Some(selected_tile) if &selected_tile == t)
                {
                    writer.write("|");
                    began_selection = true;
                }
                TileView::new(*t, false).render(writer);
                if began_selection {
                    let selected_tile = self.selected_tile.unwrap();
                    let render_closing = match iter.peek() {
                        Some(next_tile) if *next_tile != &selected_tile => true,
                        None => true,
                        _ => false,
                    };
                    if render_closing {
                        writer.write("|");
                        began_selection = false;
                    }
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
    SelectTile { factory_id: usize, tile: Tile },
}

struct FactoryAreaView {
    factories: Vec<Rc<Factory>>,
    state: FactoryAreaState,
}

impl Component for FactoryAreaView {
    fn render(&self, writer: &mut dyn crate::visor::terminal_writer::TerminalBackend) {
        let factory_views: Vec<_> = self
            .factories
            .iter()
            .enumerate()
            .map(|(i, f)| {
                let (is_selected, selected_tile): (bool, Option<Tile>) = match self.state {
                    FactoryAreaState::SelectFactory(selected) => (i == selected, None),
                    FactoryAreaState::SelectTile { factory_id, tile } if factory_id == i => {
                        (true, Some(tile))
                    }
                    FactoryAreaState::SelectTile { .. } | FactoryAreaState::Passive => {
                        (false, None)
                    }
                };
                let view = FactoryView::new(f.as_ref(), selected_tile, is_selected);
                Box::new(view) as Box<dyn Component>
            })
            .collect();

        let panel = PanelBuilder::default()
            .name("Factories")
            .padding(1)
            .component(Box::new(Layout::vertical(0, factory_views)))
            .build()
            .unwrap();

        panel.render(writer);
    }

    fn declare_dimensions(&self) -> (u16, u16) {
        (6, 8)
    }
}

pub struct GameView<const N: usize> {
    pub game: Rc<RefCell<Game<N>>>,
}

impl<const N: usize> Component for GameView<N> {
    fn render(&self, writer: &mut dyn crate::visor::terminal_writer::TerminalBackend) {
        let game = self.game.as_ref().borrow();
        let factory_state = match game.state {
            GameState::PickFactory {
                current_factory, ..
            } => FactoryAreaState::SelectFactory(current_factory),
            GameState::PickTileFromFactory {
                selected_tile,
                factory_id,
                ..
            } => FactoryAreaState::SelectTile {
                factory_id,
                tile: selected_tile,
            },
        };
        let factories: Vec<_> = game.get_factories().to_vec();
        let factory_area = FactoryAreaView {
            state: factory_state,
            factories,
        };
        let player_area = PlayerAreaView::new(game.get_players());

        let gameview = PanelBuilder::default()
            .component(Box::new(Layout::vertical(1, vec![Box::new(player_area), Box::new(factory_area)])))
            .build()
            .unwrap();
        gameview.render(writer);
    }

    fn declare_dimensions(&self) -> (u16, u16) {
        panic!("Never called");
    }

    fn handle(&mut self, e: &UserInput) -> UserEventHandled {
        let game = self.game.as_ref().borrow();
        match e {
            UserInput::Character(c) => match game.state {
                GameState::PickFactory { .. } => match c {
                    'j' => UserEventHandled::AppEvent(AppEvent::SelectNext),
                    'k' => UserEventHandled::AppEvent(AppEvent::SelectPrev),
                    _ => UserEventHandled::Noop,
                },
                GameState::PickTileFromFactory { .. } => match c {
                    'j' => UserEventHandled::AppEvent(AppEvent::SelectNext),
                    'k' => UserEventHandled::AppEvent(AppEvent::SelectPrev),
                    _ => UserEventHandled::Noop,
                },
            },
            UserInput::Confirm => match game.state {
                GameState::PickFactory {
                    player_id: _,
                    current_factory,
                } => {
                    let tile = self
                        .game
                        .borrow()
                        .find_first_tile_in_factory(current_factory);
                    UserEventHandled::AppEvent(AppEvent::TransitionToPickTileFromFactory {
                        factory_id: current_factory,
                        tile,
                    })
                }
                GameState::PickTileFromFactory { .. } => todo!(),
            },
            UserInput::Back => UserEventHandled::Noop,
        }
    }
}
