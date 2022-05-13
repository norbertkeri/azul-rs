use crate::visor::view::PanelBuilder;
use crate::{
    model::player::Player,
    visor::{view::TextView, Component},
};
use rand::{distributions::Standard, prelude::Distribution};
use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use self::patternline::PatternLine;

pub mod bag;
pub mod patternline;
pub mod player;
pub mod view;

pub enum AppEvent {
    Select(Direction),
    TransitionToPickTileFromFactory {
        factory_id: usize,
        tile: Tile,
    },
    TransitionToPickRow {
        player_id: usize,
        factory_id: usize,
        tile: Tile,
    },
    PlaceTiles {
        player_id: usize,
        factory_id: usize,
        tile: Tile,
        row_id: usize,
    },
}

pub enum Slot {
    Empty,
    Tile(Tile),
}

pub trait Scrollable<K, T>
where
    K: PartialEq,
{
    fn scroll(&self, pivot: usize, direction: Direction) -> Option<K>;
}

impl<K, T> Scrollable<K, T> for Vec<(K, T)>
where
    K: PartialEq<usize> + PartialEq<K> + Copy,
{
    fn scroll(&self, pivot: usize, direction: Direction) -> Option<K> {
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

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Tile {
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
            c => Err(format!("Invalid tile character: {}", c)),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Pickable {
    FirstPlayerToken,
    Tile(Tile),
}

impl ToString for Pickable {
    fn to_string(&self) -> String {
        match self {
            Pickable::FirstPlayerToken => String::from("1"),
            Pickable::Tile(t) => t.to_string(),
        }
    }
}

pub struct CommonArea(Vec<Pickable>);

impl Default for CommonArea {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl CommonArea {
    pub fn inspect(&self) -> &[Pickable] {
        &self.0
    }
    pub fn new(mut pickables: Vec<Pickable>) -> Self {
        let mut initial = vec![Pickable::FirstPlayerToken];
        initial.append(&mut pickables);
        Self(initial)
    }

    pub fn add(&mut self, tiles: &[Tile]) {
        let mut tiles = tiles.iter().copied().map(Pickable::Tile).collect();
        self.0.append(&mut tiles);
        self.0.sort();
    }
}

struct CommonAreaView<'a> {
    common_area: &'a CommonArea,
    selected: Option<Pickable>,
}

impl<'a> CommonAreaView<'a> {
    fn new(common_area: &'a CommonArea, selected: Option<Pickable>) -> Self {
        Self {
            common_area,
            selected,
        }
    }
}

impl<'a> Component for CommonAreaView<'a> {
    fn render(&self, writer: &mut crate::visor::renderer::RootedRenderer) {
        let output: String = self
            .common_area
            .inspect()
            .iter()
            .map(|t| t.to_string())
            .collect();

        let panel = PanelBuilder::default()
            .name("Common area")
            .padding(0)
            .component(Box::new(TextView::from(output)))
            .build()
            .unwrap();
        panel.render(writer);
    }

    fn declare_dimensions(&self) -> (u16, u16) {
        (12, 1)
    }

    fn handle(&mut self, _event: &crate::visor::UserInput) -> crate::visor::UserEventHandled {
        crate::visor::UserEventHandled::Noop
    }
}

pub struct Factory(Option<[Tile; 4]>);

impl Factory {
    pub fn new(mut tiles: [Tile; 4]) -> Self {
        tiles.sort();
        Self(Some(tiles))
    }

    pub fn new_empty() -> Self {
        Self(None)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    fn distinct_tiles(&self) -> Vec<Tile> {
        if let Some(tiles) = self.0 {
            let mut result: Vec<Tile> = Vec::with_capacity(4);
            for t in tiles.iter() {
                if !result.contains(t) {
                    result.push(*t);
                }
            }
            return result;
        }
        vec![]
    }

    pub fn find_adjacent_tile(&self, tile: Tile, direction: Direction) -> Tile {
        let distinct_tiles = self.distinct_tiles();
        if distinct_tiles.len() == 1 {
            return tile;
        }
        let i = distinct_tiles
            .iter()
            .position(|maybe_t| maybe_t == &tile)
            .expect("This tile is not in the factory?");
        match (i, direction) {
            (0, Direction::Prev) => *distinct_tiles.last().unwrap(),
            (i, Direction::Next) if i == distinct_tiles.len() - 1 => {
                *distinct_tiles.first().unwrap()
            }
            (i, Direction::Prev) => distinct_tiles[i - 1],
            (i, Direction::Next) => distinct_tiles[i + 1],
        }
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

    pub fn count_tile(&self, tile: Tile) -> usize {
        match self.get_tiles() {
            Some(tiles) => tiles.iter().filter(|&&t| t == tile).count(),
            None => 0,
        }
    }

    pub fn pick(
        &mut self,
        picked_tile: Tile,
        common_area: &mut CommonArea,
        pattern_line: &mut PatternLine,
    ) -> Result<(), String> {
        let tiles = self
            .0
            .as_ref()
            .ok_or_else(|| String::from("You tried picking an empty factory"))?;
        if !tiles.contains(&picked_tile) {
            return Err(format!(
                "You tried picking tile {} from a factory that does not have it",
                picked_tile
            ));
        }
        let (picked, non_picked): (Vec<Tile>, Vec<Tile>) =
            tiles.iter().partition(|&tile| tile == &picked_tile);
        pattern_line.accept(picked_tile, picked.len())?;
        common_area.add(&non_picked);
        self.0 = None;
        Ok(())
    }
}

impl Factory {
    pub fn new_random() -> Self {
        let tiles: [Tile; 4] = [
            rand::random(),
            rand::random(),
            rand::random(),
            rand::random(),
        ];
        Self::new(tiles)
    }
}

pub enum Direction {
    Next,
    Prev,
}

pub struct Game<const N: usize> {
    players: [Player; N],
    factories: [Factory; 4],
    state: GameState,
    pub common_area: CommonArea,
}

impl<const N: usize> Game<N> {
    pub fn get_players(&self) -> &[Player] {
        &self.players
    }

    pub fn find_pickable_factories(&self) -> Vec<(usize, &Factory)> {
        self.factories
            .iter()
            .enumerate()
            .filter(|(_, f)| !f.is_empty())
            .collect()
    }

    pub fn find_first_tile_in_factory(&self, factory_id: usize) -> Tile {
        self.factories[factory_id].find_first_tile().unwrap() // TODO unwrap
    }

    pub fn for_players(players: [Player; N]) -> Self {
        let factories = Self::generate_factories();
        Game {
            players,
            factories,
            state: GameState::PickFactory {
                player_id: 0,
                current_factory: 0,
            },
            common_area: CommonArea::default(),
        }
    }

    pub fn get_factories(&self) -> &[Factory] {
        &self.factories
    }

    fn generate_factories() -> [Factory; 4] {
        [0, 1, 2, 3].map(|_| Factory::new_random())
    }

    fn find_adjacent_selectable_row(
        &self,
        tile: Tile,
        how_many: usize,
        player_id: usize,
        current_row: usize,
        direction: Direction,
    ) -> Option<usize> {
        let barea = self.get_players()[player_id].get_buildingarea();
        let rows = barea.get_rows_that_can_accept(tile, how_many);
        rows.scroll(current_row, direction)
    }

    pub fn handle(&mut self, e: AppEvent) {
        let new_e = match e {
            AppEvent::Select(dir) => match self.state {
                GameState::PickFactory {
                    player_id,
                    current_factory,
                } => {
                    let pickable_factories = self.find_pickable_factories();
                    let next_factory_id = pickable_factories
                        .scroll(current_factory, dir)
                        .unwrap_or(current_factory);
                    GameState::PickFactory {
                        player_id,
                        current_factory: next_factory_id,
                    }
                }
                GameState::PickTileFromFactory {
                    player_id,
                    factory_id,
                    selected_tile,
                } => {
                    let next_tile =
                        self.factories[factory_id].find_adjacent_tile(selected_tile, dir);
                    GameState::PickTileFromFactory {
                        player_id,
                        factory_id,
                        selected_tile: next_tile,
                    }
                }
                GameState::PickRowToPutTiles {
                    player_id,
                    factory_id,
                    tile,
                    selected_row_id,
                } => {
                    let factory = &self.get_factories()[factory_id];
                    let count = factory.count_tile(tile);
                    let selected_row_id = self
                        .find_adjacent_selectable_row(tile, count, player_id, selected_row_id, dir)
                        .unwrap_or(selected_row_id);
                    GameState::PickRowToPutTiles {
                        factory_id,
                        player_id,
                        tile,
                        selected_row_id,
                    }
                }
            },
            AppEvent::TransitionToPickTileFromFactory {
                factory_id: _,
                tile,
            } => match self.state {
                GameState::PickFactory {
                    player_id,
                    current_factory,
                } => GameState::PickTileFromFactory {
                    player_id,
                    factory_id: current_factory,
                    selected_tile: tile,
                },
                GameState::PickTileFromFactory { .. } => panic!("Cannot happen"),
                GameState::PickRowToPutTiles { .. } => panic!("Cannot happen"),
            },
            AppEvent::TransitionToPickRow {
                player_id,
                factory_id,
                tile,
            } => {
                let factory = &self.get_factories()[factory_id];
                let how_many = factory.count_tile(tile);
                let buildingarea = self.get_players()[player_id].get_buildingarea();
                let rows = buildingarea.get_rows_that_can_accept(tile, how_many);
                let first_that_can_fit = rows.first();
                assert!(first_that_can_fit.is_some());
                let (selected_row_id, _) = *first_that_can_fit.unwrap();
                GameState::PickRowToPutTiles {
                    player_id,
                    factory_id,
                    tile,
                    selected_row_id,
                }
            }
            AppEvent::PlaceTiles {
                player_id,
                factory_id,
                tile,
                row_id,
            } => {
                let factory = &mut self.factories[factory_id];
                let buildingarea = self.players[player_id].get_buildingarea_mut();
                let row = buildingarea.get_row_mut(row_id);
                factory.pick(tile, &mut self.common_area, row).unwrap();
                let pickable_factories = self.find_pickable_factories();
                let next_factory = pickable_factories.first();
                if next_factory.is_none() {
                    panic!("Ran out of factories");
                }
                GameState::PickFactory {
                    player_id: 1 - player_id,
                    current_factory: next_factory.unwrap().0,
                }
            }
        };

        self.state = new_e;
    }
}

pub enum GameState {
    PickFactory {
        player_id: usize,
        current_factory: usize,
    },
    PickTileFromFactory {
        player_id: usize,
        factory_id: usize,
        selected_tile: Tile,
    },
    PickRowToPutTiles {
        player_id: usize,
        factory_id: usize,
        tile: Tile,
        selected_row_id: usize,
    },
}

#[cfg(test)]
mod tests {
    use crate::model::{Factory, Tile};

    #[test]
    fn test_count_tile() {
        let factory = Factory::new([Tile::Yellow, Tile::Yellow, Tile::Green, Tile::Red]);
        assert_eq!(factory.count_tile(Tile::Yellow), 2);
    }
}
