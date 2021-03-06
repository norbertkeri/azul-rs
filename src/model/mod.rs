use crate::visor::view::PanelBuilder;
use crate::visor::UserInput;
use crate::{
    model::player::Player,
    visor::{view::TextView, Component},
};
use rand::{distributions::Standard, prelude::Distribution};
use std::ops::{Index, IndexMut};
use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use self::bag::Bag;

use self::buildingarea::IsGameOver;
use self::tilecollection::{HasTileCollection, TileCollection};
use self::view::render_pickables;

pub mod bag;
pub mod buildingarea;
pub mod player;
pub mod tilecollection;
pub mod view;

pub enum AppEvent {
    Select(Direction),
    TransitionToPickTileFromFactory { tile: Tile },
    TransitionToPickRow { tile: Tile },
    PlaceTiles { tile: Tile, row_id: usize },
}

pub trait Scrollable<T> {
    fn scroll(&self, pivot: T, direction: Direction) -> Option<T>;
}

impl<T> Scrollable<T> for Vec<T>
where
    T: PartialEq + Copy,
{
    fn scroll(&self, pivot: T, direction: Direction) -> Option<T> {
        let current_row_index = self
            .iter()
            .position(|p| p == &pivot)
            .expect("The pivot is not in the vec?");

        match (self.len(), direction) {
            (count, Direction::Next) if count - 1 == current_row_index => {
                let first_index = self.first().unwrap();
                Some(*first_index)
            }
            (_, Direction::Prev) if current_row_index == 0 => {
                let last_index = self.last().unwrap();
                Some(*last_index)
            }
            (_, Direction::Next) => self.get(current_row_index + 1).copied(),
            (_, Direction::Prev) => self.get(current_row_index - 1).copied(),
        }
    }
}

impl<S, T> Scrollable<S> for Vec<(S, T)>
where
    S: PartialEq<usize> + PartialEq<S> + Copy,
{
    fn scroll(&self, pivot: S, direction: Direction) -> Option<S> {
        let current_row_index = self
            .iter()
            .position(|(i, _)| i == &pivot)
            .expect("The pivot is not in the vec?");

        match (self.len(), direction) {
            (count, Direction::Next) if count - 1 == current_row_index => {
                let (first_index, _) = self.first().unwrap();
                Some(*first_index)
            }
            (_, Direction::Prev) if current_row_index == 0 => {
                let (last_index, _) = self.last().unwrap();
                Some(*last_index)
            }
            (_, Direction::Next) => self
                .get(current_row_index + 1)
                .map(|(next_index, _)| *next_index),
            (_, Direction::Prev) => self
                .get(current_row_index - 1)
                .map(|(prev_index, _)| *prev_index),
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Tile {
    FirstPlayer,
    Blue,
    Green,
    Red,
    White,
    Yellow,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self.to_string();
        write!(f, "{}", s)
    }
}

impl Distribution<Tile> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Tile {
        match rng.gen_range(0..=4) {
            0 => Tile::Yellow,
            1 => Tile::Red,
            2 => Tile::Blue,
            3 => Tile::Green,
            _ => Tile::White,
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Tile::Yellow => "Y",
            Tile::Red => "R",
            Tile::Blue => "B",
            Tile::Green => "G",
            Tile::White => "W",
            Tile::FirstPlayer => "1",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for Tile {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 1 {
            return Err("You can only create tiles from single character strings".into());
        }
        match s.chars().next().unwrap() {
            'Y' => Ok(Tile::Yellow),
            'R' => Ok(Tile::Red),
            'B' => Ok(Tile::Blue),
            'G' => Ok(Tile::Green),
            'W' => Ok(Tile::White),
            '1' => Ok(Tile::FirstPlayer),
            c => Err(format!("Invalid tile character: {}", c)),
        }
    }
}

#[derive(Debug)]
pub struct CommonArea(Vec<Tile>);

impl HasTileCollection for CommonArea {
    fn get_tilecollection(&self) -> Box<dyn TileCollection> {
        Box::new(self.0.clone())
    }
}

impl Default for CommonArea {
    fn default() -> Self {
        let initial = vec![Tile::FirstPlayer];
        Self(initial)
    }
}

impl CommonArea {
    pub fn inspect(&self) -> &[Tile] {
        &self.0
    }

    pub fn add(&mut self, tiles: &[Tile]) {
        let mut tiles = tiles.to_vec();
        self.0.append(&mut tiles);
        self.0.sort();
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn pick_tile(&mut self, tile: Tile) -> (usize, bool) {
        let count = self.count_tile(tile);
        let has_firstplayer = self.0.iter().any(|&t| t == Tile::FirstPlayer);
        let new: Vec<_> = self
            .0
            .iter()
            .copied()
            .filter(|&t| tile != t && t != Tile::FirstPlayer)
            .collect();
        self.0 = new;
        (count, has_firstplayer)
    }
}

enum CommonAreaStateView {
    Selected,
    SelectedWithTile(Tile),
    Passive,
}

struct CommonAreaView<'a> {
    common_area: &'a CommonArea,
    state: CommonAreaStateView,
}

impl<'a> CommonAreaView<'a> {
    fn new(common_area: &'a CommonArea, state: CommonAreaStateView) -> Self {
        Self { common_area, state }
    }
}

impl<'a, const N: usize> From<&'a Game<N>> for CommonAreaView<'a> {
    fn from(game: &'a Game<N>) -> Self {
        let state = match game.state {
            GameState::PickSource => match game.current_source {
                TileSource::Factory(_) => CommonAreaStateView::Passive,
                TileSource::CommonArea => CommonAreaStateView::Selected,
            },
            GameState::PickRowToPutTiles { tile, .. }
            | GameState::PickTileFromSource {
                selected_tile: tile,
                ..
            } => match game.current_source {
                TileSource::Factory(_) => CommonAreaStateView::Passive,
                TileSource::CommonArea => CommonAreaStateView::SelectedWithTile(tile),
            },
        };
        Self::new(&game.common_area, state)
    }
}

impl<'a> Component for CommonAreaView<'a> {
    fn render(&self, writer: &mut crate::visor::renderer::RootedRenderer) {
        let (is_selected, selected_tiles) = match self.state {
            CommonAreaStateView::Selected => (true, vec![]),
            CommonAreaStateView::SelectedWithTile(tile) => (true, vec![tile, Tile::FirstPlayer]),
            CommonAreaStateView::Passive => (false, vec![]),
        };

        let output = render_pickables(is_selected, &self.common_area.0, &selected_tiles);

        let panel = PanelBuilder::default()
            .name("Common")
            .padding(0)
            .component(Box::new(TextView::from(output)))
            .build()
            .unwrap();
        panel.render(writer);
    }

    fn handle(&mut self, _event: &crate::visor::UserInput) -> crate::visor::UserEventHandled {
        crate::visor::UserEventHandled::Noop
    }
}

pub const TILE_PER_FACTORY: usize = 4;
pub struct Factory(Option<[Tile; TILE_PER_FACTORY]>);

impl HasTileCollection for Factory {
    fn get_tilecollection(&self) -> Box<dyn TileCollection> {
        match self.0 {
            Some(tiles) => Box::new(tiles) as Box<_>,
            None => Box::new(vec![]) as Box<_>,
        }
    }
}

impl Factory {
    pub fn new(mut tiles: [Tile; TILE_PER_FACTORY]) -> Self {
        tiles.sort();
        Self(Some(tiles))
    }

    pub fn new_from_bag(bag: &mut Bag) -> Self {
        let mut factory = Self::new_empty();
        bag.fill_factory(&mut factory);
        factory
    }

    pub fn new_empty() -> Self {
        Self(None)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    pub fn find_first_tile(&self) -> Option<Tile> {
        self.0.map(|tiles| tiles[0])
    }

    pub fn find_last_tile(&self) -> Option<Tile> {
        self.0.and_then(|tiles| tiles.last().copied())
    }

    pub fn get_tiles(&self) -> Option<&[Tile]> {
        self.0.as_ref().map(|t| t.as_slice())
    }
}

pub enum Direction {
    Next,
    Prev,
}

pub struct Game<const N: usize> {
    players: [Player; N],
    factories: Vec<Factory>,
    state: GameState,
    bag: Bag,
    is_over: bool,
    pub(crate) common_area: CommonArea,
    current_player_id: usize,
    current_source: TileSource,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
pub struct FactoryId(usize);

impl PartialEq<usize> for FactoryId {
    fn eq(&self, other: &usize) -> bool {
        &self.0 == other
    }
}

impl From<usize> for FactoryId {
    fn from(id: usize) -> Self {
        Self(id)
    }
}

impl Index<FactoryId> for [Factory] {
    type Output = Factory;

    fn index(&self, index: FactoryId) -> &Self::Output {
        &self[index.0]
    }
}

impl IndexMut<FactoryId> for Vec<Factory> {
    fn index_mut(&mut self, index: FactoryId) -> &mut Self::Output {
        &mut self[index.0]
    }
}

impl Index<FactoryId> for Vec<Factory> {
    type Output = Factory;

    fn index(&self, index: FactoryId) -> &Self::Output {
        &self[index.0]
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TileSource {
    Factory(FactoryId),
    CommonArea,
}

impl<const N: usize> Game<N> {
    pub fn get_players(&self) -> &[Player] {
        &self.players
    }

    pub fn find_pickable_sources(&self) -> Vec<TileSource> {
        let mut sources: Vec<_> = self
            .factories
            .iter()
            .enumerate()
            .filter(|(_, f)| !f.is_empty())
            .map(|(id, _)| TileSource::Factory(id.into()))
            .collect();

        if !self.common_area.is_empty() {
            sources.push(TileSource::CommonArea);
        }
        sources
    }

    pub fn for_players(players: [Player; N]) -> Self {
        let mut bag = Bag::default();
        let factories: Vec<_> = [0, 1, 2, 3]
            .iter()
            .map(|_| Factory::new_from_bag(&mut bag))
            .collect();

        Game {
            players,
            factories,
            state: GameState::PickSource,
            is_over: false,
            bag,
            common_area: CommonArea::default(),
            current_player_id: 0,
            current_source: TileSource::Factory(0.into()),
        }
    }

    pub fn get_factories(&self) -> &[Factory] {
        &self.factories
    }

    pub fn count_tiles_in(&self, source: TileSource, tile: Tile) -> usize {
        match source {
            TileSource::Factory(factory_id) => self.factories[factory_id].count_tile(tile),
            TileSource::CommonArea => self.common_area.count_tile(tile),
        }
    }

    fn find_adjacent_selectable_row(
        &self,
        tile: Tile,
        player_id: usize,
        current_row: usize,
        direction: Direction,
    ) -> Option<usize> {
        let barea = self.get_players()[player_id].get_buildingarea();
        let rows = barea.get_rows_that_can_accept(tile);
        rows.scroll(current_row, direction)
    }

    pub fn handle(&mut self, input: UserInput) -> bool {
        let new_state = match input {
            UserInput::Direction(dir) => match self.state {
                GameState::PickSource => {
                    let pickable_factories = self.find_pickable_sources();
                    let next_factory_id = pickable_factories
                        .scroll(self.current_source, dir)
                        .unwrap_or(self.current_source);
                    self.current_source = next_factory_id;
                    Some(GameState::PickSource)
                }
                GameState::PickTileFromSource { selected_tile } => {
                    let next_tile = match self.current_source {
                        TileSource::Factory(factory_id) => {
                            self.factories[factory_id].find_adjacent_tile(selected_tile, dir)
                        }
                        TileSource::CommonArea => {
                            self.common_area.find_adjacent_tile(selected_tile, dir)
                        }
                    };
                    Some(GameState::PickTileFromSource {
                        selected_tile: next_tile,
                    })
                }
                GameState::PickRowToPutTiles {
                    tile,
                    selected_row_id,
                } => {
                    let selected_row_id = self
                        .find_adjacent_selectable_row(
                            tile,
                            self.current_player_id,
                            selected_row_id,
                            dir,
                        )
                        .unwrap_or(selected_row_id);
                    Some(GameState::PickRowToPutTiles {
                        tile,
                        selected_row_id,
                    })
                }
            },
            UserInput::Confirm => match self.state {
                GameState::PickSource => {
                    let source = self.find_source(self.current_source);
                    let tile = source
                        .find_first_tile()
                        .expect("You managed to pick a source that has no tiles?");

                    Some(GameState::PickTileFromSource {
                        selected_tile: tile,
                    })
                }
                GameState::PickTileFromSource {
                    selected_tile: tile,
                } => {
                    let buildingarea =
                        self.get_players()[self.current_player_id].get_buildingarea();
                    let rows = buildingarea.get_rows_that_can_accept(tile);
                    let first_that_can_fit = rows.first();
                    if first_that_can_fit.is_none() {
                        todo!("No rows can fit the tiles, all should go to the floorline");
                    }
                    let selected_row_id = *first_that_can_fit.unwrap();
                    Some(GameState::PickRowToPutTiles {
                        tile,
                        selected_row_id,
                    })
                }
                GameState::PickRowToPutTiles {
                    tile,
                    selected_row_id: row_id,
                } => {
                    self.pick(self.current_source, tile, self.current_player_id, row_id)
                        .unwrap();

                    let pickable_sources = self.find_pickable_sources();
                    let next_source = pickable_sources.first();
                    let next = if let Some(next_source) = next_source {
                        self.current_player_id = (N - 1) - self.current_player_id;
                        self.current_source = *next_source;
                        GameState::PickSource
                    } else {
                        self.advance_round()
                    };
                    Some(next)
                }
            },
            UserInput::Back => match self.state {
                GameState::PickSource => None,
                GameState::PickTileFromSource { selected_tile: _ } => Some(GameState::PickSource),
                GameState::PickRowToPutTiles {
                    tile,
                    selected_row_id: _,
                } => Some(GameState::PickTileFromSource {
                    selected_tile: tile,
                }),
            },
            UserInput::Exit => {
                self.is_over = true;
                None
            }
            UserInput::Character(_) | UserInput::Noop => None,
        };

        if let Some(new_state) = new_state {
            self.state = new_state;
        }
        self.is_over
    }

    fn reset_first_player_token(&mut self) -> usize {
        let player_id = self
            .players
            .iter()
            .enumerate()
            .find(|(_i, player)| player.has_first_player_token())
            .map(|(i, _)| i);
        if player_id.is_none() {
            panic!("Nobody has the first player token at the end of the round?");
        }
        self.common_area.add(&[Tile::FirstPlayer]);
        player_id.unwrap()
    }

    fn advance_round(&mut self) -> GameState {
        let next_player = self.reset_first_player_token();
        self.is_over = self.flush_tiles().into();
        self.refill_factories();
        self.current_player_id = next_player;
        self.current_source = TileSource::Factory(FactoryId(0));
        GameState::PickSource
    }

    fn flush_tiles(&mut self) -> IsGameOver {
        let mut is_game_over = IsGameOver::No;
        for p in self.players.iter_mut() {
            let bg = p.get_buildingarea_mut();
            if let IsGameOver::Yes = bg.move_tiles_to_wall() {
                is_game_over = IsGameOver::Yes;
            }
            bg.flush_floorline();
        }
        is_game_over
    }

    fn refill_factories(&mut self) {
        for f in self.factories.iter_mut() {
            self.bag.fill_factory(f);
        }
    }

    pub fn pick(
        &mut self,
        source: TileSource,
        tile: Tile,
        player_id: usize,
        row_id: usize,
    ) -> Result<(), String> {
        let buildingarea = self.players[player_id].get_buildingarea_mut();
        let remaining = match source {
            TileSource::Factory(factory_id) => {
                let factory = &mut self.factories[factory_id];
                buildingarea.pick_factory(factory, &mut self.common_area, row_id, tile)
            }
            TileSource::CommonArea => {
                let (count, has_firstplayer) = self.common_area.pick_tile(tile);
                buildingarea.pick_from_common_area(row_id, tile, count, has_firstplayer)
            }
        }?;
        self.bag.discard(&remaining);
        Ok(())
    }

    pub fn find_source(&self, source: TileSource) -> Box<dyn TileCollection> {
        match source {
            TileSource::Factory(factory_id) => self.factories[factory_id].get_tilecollection(),
            TileSource::CommonArea => self.common_area.get_tilecollection(),
        }
    }
}

pub enum GameState {
    PickSource,
    PickTileFromSource { selected_tile: Tile },
    PickRowToPutTiles { tile: Tile, selected_row_id: usize },
}

impl GameState {
    pub fn find_selected_tile(&self) -> Option<Tile> {
        match self {
            GameState::PickSource { .. } => None,
            GameState::PickTileFromSource { selected_tile, .. } => Some(*selected_tile),
            GameState::PickRowToPutTiles { tile, .. } => Some(*tile),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{tilecollection::TileCollection, Factory, Tile};

    #[test]
    fn test_count_tile() {
        let factory = Factory::new([Tile::Yellow, Tile::Yellow, Tile::Green, Tile::Red]);
        assert_eq!(factory.count_tile(Tile::Yellow), 2);
    }
}
