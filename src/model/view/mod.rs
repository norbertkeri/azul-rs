use self::player::PlayerAreaView;

use super::{AppEvent, CommonAreaView, Direction, Factory, FactoryId, Game, Tile};
use crate::visor::renderer::RootedRenderer;
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
    fn render(&self, writer: &mut RootedRenderer) {
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
            _ => false,
        }
    }
}

impl<'a> From<FactoryView<'a>> for Box<dyn Component + 'a> {
    fn from(s: FactoryView<'a>) -> Self {
        Box::new(s)
    }
}

impl Component for FactoryView<'_> {
    fn render(&self, writer: &mut RootedRenderer) {
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
        writer.write("\n");
    }

    fn declare_dimensions(&self) -> (u16, u16) {
        if self.has_selected_tile() {
            return (8, 1);
        }
        (5, 1)
    }

    fn handle(&mut self, _event: &UserInput) -> UserEventHandled {
        UserEventHandled::Noop
    }
}

pub enum FactoryAreaState {
    Passive,
    SelectFactory(FactoryId), // TODO name this as currently_selected?
    SelectTile { factory_id: FactoryId, tile: Tile },
}

struct FactoryAreaView<'a> {
    factories: &'a [Factory],
    state: FactoryAreaState,
}

impl<'a, const N: usize> From<&'a Game<N>> for FactoryAreaView<'a> {
    fn from(game: &'a Game<N>) -> Self {
        let factory_state = match game.state {
            GameState::PickSource { current_source, .. } => match current_source {
                super::TileSource::Factory(factory_id) => {
                    FactoryAreaState::SelectFactory(factory_id)
                }
                super::TileSource::CommonArea => FactoryAreaState::Passive,
            },
            GameState::PickTileFromSource {
                selected_tile,
                current_source,
                ..
            }
            | GameState::PickRowToPutTiles {
                tile: selected_tile,
                source: current_source,
                ..
            } => match current_source {
                super::TileSource::Factory(factory_id) => FactoryAreaState::SelectTile {
                    factory_id,
                    tile: selected_tile,
                },
                super::TileSource::CommonArea => FactoryAreaState::Passive,
            },
        };
        let factories = game.get_factories();
        Self {
            state: factory_state,
            factories,
        }
    }
}

impl Component for FactoryAreaView<'_> {
    fn render(&self, writer: &mut RootedRenderer) {
        let factory_views: Vec<_> = self
            .factories
            .iter()
            .enumerate()
            .map(|(i, f)| {
                let (is_selected, selected_tile): (bool, Option<Tile>) = match self.state {
                    FactoryAreaState::SelectFactory(selected) => (selected == i, None),
                    FactoryAreaState::SelectTile { factory_id, tile } if factory_id == i => {
                        (true, Some(tile))
                    }
                    FactoryAreaState::SelectTile { .. } | FactoryAreaState::Passive => {
                        (false, None)
                    }
                };
                let view = FactoryView::new(f, selected_tile, is_selected);
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
        (18, 8)
    }
}

pub struct GameView<const N: usize> {
    pub game: Rc<RefCell<Game<N>>>,
}

impl<const N: usize> Component for GameView<N> {
    fn render(&self, writer: &mut RootedRenderer) {
        let game: &Game<N> = &self.game.as_ref().borrow();
        let player_area: PlayerAreaView = game.into();
        let factory_area: FactoryAreaView = game.into();
        let common_area = CommonAreaView::new(&game.common_area, None);

        let gameview = PanelBuilder::default()
            .component(Box::new(Layout::vertical(
                1,
                vec![
                    Box::new(player_area),
                    Box::new(Layout::horizontal(
                        1,
                        vec![Box::new(factory_area), Box::new(common_area)],
                    )),
                ],
            )))
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
                GameState::PickSource { .. } => match c {
                    'j' => UserEventHandled::AppEvent(AppEvent::Select(Direction::Next)),
                    'k' => UserEventHandled::AppEvent(AppEvent::Select(Direction::Prev)),
                    _ => UserEventHandled::Noop,
                },
                GameState::PickTileFromSource { .. } => match c {
                    'j' => UserEventHandled::AppEvent(AppEvent::Select(Direction::Next)),
                    'k' => UserEventHandled::AppEvent(AppEvent::Select(Direction::Prev)),
                    _ => UserEventHandled::Noop,
                },
                GameState::PickRowToPutTiles { .. } => match c {
                    'j' => UserEventHandled::AppEvent(AppEvent::Select(Direction::Next)),
                    'k' => UserEventHandled::AppEvent(AppEvent::Select(Direction::Prev)),
                    _ => UserEventHandled::Noop,
                },
            },
            UserInput::Confirm => match game.state {
                GameState::PickSource {
                    player_id: _,
                    current_source,
                } => {
                    let tile = self.game.borrow().find_first_tile_in_source(current_source);

                    UserEventHandled::AppEvent(AppEvent::TransitionToPickTileFromFactory {
                        source: current_source,
                        tile,
                    })
                }
                GameState::PickTileFromSource {
                    player_id,
                    current_source,
                    selected_tile,
                } => UserEventHandled::AppEvent(AppEvent::TransitionToPickRow {
                    player_id,
                    tile: selected_tile,
                    source: current_source,
                }),
                GameState::PickRowToPutTiles {
                    player_id,
                    source,
                    tile,
                    selected_row_id,
                } => UserEventHandled::AppEvent(AppEvent::PlaceTiles {
                    player_id,
                    source,
                    tile,
                    row_id: selected_row_id,
                }),
            },
            UserInput::Back => UserEventHandled::Noop,
        }
    }
}
