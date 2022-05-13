#![allow(dead_code)]

use crate::model::player::Player;
use rand::{distributions::Standard, prelude::Distribution};
use std::{
    fmt::{Debug, Display},
    rc::Rc,
    str::FromStr,
};

use self::patternline::PatternLine;

pub mod bag;
pub mod patternline;
pub mod player;
pub mod view;

pub enum AppEvent {
    SelectNext,
    SelectPrev,
    TransitionToPickTileFromFactory { factory_id: usize, tile: Tile },
    TransitionToPickRow { player_id: usize, factory_id: usize, tile: Tile },
    PlaceTiles { player_id: usize, factory_id: usize, tile: Tile, row_id: usize }
}

pub enum Slot {
    Empty,
    Tile(Tile),
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
}

impl CommonArea {
    pub fn new(mut pickables: Vec<Pickable>) -> Self {
        let mut initial = vec![Pickable::FirstPlayerToken];
        initial.append(&mut pickables);
        Self(initial)
    }

    pub fn add(&mut self, tiles: &[Tile]) {
        let mut tiles = tiles.iter().copied().map(Pickable::Tile).collect();
        self.0.append(&mut tiles);
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

    pub fn find_before(&self, before: Tile) -> Tile {
        let distinct = self.distinct_tiles();
        let i = distinct
            .iter()
            .position(|maybe_t| maybe_t == &before)
            .expect("This tile is not in the factory?");
        if i == 0 {
            return *distinct.last().unwrap();
        }
        distinct[i - 1]
    }

    pub fn find_after(&self, after: Tile) -> Tile {
        let distinct = self.distinct_tiles();
        let i = distinct
            .iter()
            .position(|maybe_t| maybe_t == &after)
            .expect("This tile is not in the factory?");
        if i == distinct.len() - 1 {
            return *distinct.first().unwrap();
        }
        distinct[i + 1]
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
    Prev
}

pub struct Game<const N: usize> {
    players: [Player; N],
    factories: [Factory; 4],
    state: GameState,
    common_area: CommonArea
}

impl<const N: usize> Game<N> {
    pub fn get_players(&self) -> &[Player] {
        &self.players
    }

    pub fn find_first_tile_in_factory(&self, factory_id: usize) -> Tile {
        self.factories[factory_id].find_first_tile().unwrap() // TODO unwrap
    }

    pub fn find_next_tile_in_factory_after(&self, factory_id: usize, after: Tile) -> Tile {
        self.factories[factory_id].find_after(after)
    }

    pub fn find_prev_tile_in_factory_after(&self, factory_id: usize, before: Tile) -> Tile {
        self.factories[factory_id].find_before(before)
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
            common_area: CommonArea::default()
        }
    }

    pub fn get_factories(&self) -> &[Factory] {
        &self.factories
    }

    fn generate_factories() -> [Factory; 4] {
        [0, 1, 2, 3].map(|_| Factory::new_random())
    }

    fn find_adjacent_selectable_row(&self, tile: Tile, how_many: usize, player_id: usize, current_row: usize, direction: Direction) -> Option<usize> {
        let barea = self.get_players()[player_id].get_buildingarea();
        let rows = barea.get_rows_that_can_accept(tile, how_many);
        let current_row_index = rows
            .iter()
            .position(|(i, _)| i == &current_row)
            .expect("The row is not here?");

        match (rows.len(), direction) {
            (count, Direction::Next) if count - 1 == current_row_index => {
                let (first_index, _) = rows.first().unwrap();
                Some(*first_index)
            },
            (_, Direction::Prev) if current_row_index == 0 => {
                let (last_index, _) = rows.last().unwrap();
                Some(*last_index)
            },
            (_, Direction::Next) => {
                rows.get(current_row_index + 1)
                .map(|(next_index, _)| *next_index)
            }
            (_, Direction::Prev) => {
                rows.get(current_row_index - 1)
                .map(|(prev_index, _)| *prev_index)
            }
        }
    }

    pub fn handle(&mut self, e: AppEvent) {
        let new_e = match e {
            AppEvent::SelectNext => match self.state {
                GameState::PickFactory {
                    player_id,
                    current_factory,
                } => GameState::PickFactory {
                    player_id,
                    current_factory: current_factory + 1,
                },
                GameState::PickTileFromFactory {
                    player_id,
                    factory_id,
                    selected_tile,
                } => {
                    let next_tile = self.find_next_tile_in_factory_after(factory_id, selected_tile);
                    GameState::PickTileFromFactory {
                        player_id,
                        factory_id,
                        selected_tile: next_tile,
                    }
                }
                GameState::PickRowToPutTiles { player_id, factory_id, tile, selected_row_id } => {
                    let factory = &self.get_factories()[factory_id];
                    let count = factory.count_tile(tile);
                    let selected_row_id = self.find_adjacent_selectable_row(tile, count, player_id, selected_row_id, Direction::Next).unwrap_or(selected_row_id);
                    GameState::PickRowToPutTiles {
                        factory_id, player_id, tile, selected_row_id
                    }
                }
            },
            AppEvent::SelectPrev => match self.state {
                GameState::PickFactory {
                    player_id,
                    current_factory,
                } => GameState::PickFactory {
                    player_id,
                    current_factory: current_factory - 1,
                },
                GameState::PickTileFromFactory {
                    player_id,
                    factory_id,
                    selected_tile,
                } => {
                    let next_tile = self.find_prev_tile_in_factory_after(factory_id, selected_tile);
                    GameState::PickTileFromFactory {
                        player_id,
                        factory_id,
                        selected_tile: next_tile,
                    }
                }
                GameState::PickRowToPutTiles { player_id, factory_id, tile, selected_row_id } => {
                    let factory = &self.get_factories()[factory_id];
                    let count = factory.count_tile(tile);
                    let selected_row_id = self.find_adjacent_selectable_row(tile, count, player_id, selected_row_id, Direction::Prev).unwrap_or(selected_row_id);
                    GameState::PickRowToPutTiles {
                        factory_id, player_id, tile, selected_row_id
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
                GameState::PickTileFromFactory {
                    player_id: _,
                    factory_id: _,
                    selected_tile: _,
                } => {
                    panic!("Cannot happen");
                }
                GameState::PickRowToPutTiles { player_id, factory_id, tile, selected_row_id } => panic!("Cannot happen")
            },
            AppEvent::TransitionToPickRow { player_id, factory_id, tile } => {
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
                    selected_row_id
                }
            }
            AppEvent::PlaceTiles { player_id, factory_id, tile, row_id } => {
                let factory = &mut self.factories[factory_id];
                let buildingarea = self.players[player_id].get_buildingarea_mut();
                let row = buildingarea.get_row_mut(row_id);
                factory.pick(tile, &mut self.common_area, row).unwrap();
                GameState::PickFactory {
                    player_id: 1 - player_id,
                    current_factory: 0
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
        selected_row_id: usize
    }
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
