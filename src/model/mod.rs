#![allow(dead_code)]

use rand::{distributions::Standard, prelude::Distribution};
use std::{fmt::Debug, rc::Rc};

pub mod view;

pub enum AppEvent {
    SelectNext,
    SelectPrev,
}

pub struct Player {
    name: String,
}

impl Player {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

pub enum Slot {
    Empty,
    Tile(Tile)
}

pub enum MinusPoints {
    FirstPlayerToken,
    Tile(Tile)
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

impl ToString for Tile {
    fn to_string(&self) -> String {
        match self {
            Tile::Yellow => "Y",
            Tile::Red => "R",
            Tile::Blue => "B",
            Tile::Green => "G",
            Tile::White => "W"
        }.into()
    }
}

pub enum Pickable {
    FirstPlayerToken,
    Tile(Tile)
}

impl ToString for Pickable {
    fn to_string(&self) -> String {
        match self {
            Pickable::FirstPlayerToken => String::from("1"),
            Pickable::Tile(t) => t.to_string()
        }
    }
}

pub struct CommonArea(Vec<Pickable>);

pub struct Factory([Tile; 4]);

impl Factory {
    pub fn new(mut tiles: [Tile; 4]) -> Self {
        tiles.sort();
        Self(tiles)
    }

    pub fn get_tiles(&self) -> &[Tile] {
        &self.0
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
        Self(tiles)
    }
}

pub struct Game<const N: usize> {
    players: [Player; N],
    factories: [Rc<Factory>; 4],
    state: GameState
}

impl<const N: usize> Game<N> {
    pub fn for_players(players: [Player; N]) -> Self {
        let factories = Self::generate_factories();
        Game {
            players,
            factories,
            state: GameState::PickFactory { player_id: 0, current_factory: 0 }
        }
    }

    pub fn get_factories(&self) -> &[Rc<Factory>] {
        &self.factories
    }

    fn generate_factories() -> [Rc<Factory>; 4] {
        [0,1,2,3].map(|_| {
            Rc::new(Factory::new_random())
        })
    }

    pub fn handle(&mut self, e: AppEvent) {
        let new_e = match e {
            AppEvent::SelectNext => {
                match self.state {
                    GameState::PickFactory { player_id, current_factory } => {
                        GameState::PickFactory { player_id, current_factory: current_factory + 1 }
                    },
                }
            },
            AppEvent::SelectPrev => {
                match self.state {
                    GameState::PickFactory { player_id, current_factory } => {
                        GameState::PickFactory { player_id, current_factory: current_factory - 1 }
                    },
                }
            }
        };

        self.state = new_e;
    }

}

pub enum GameState {
    PickFactory { player_id: usize, current_factory: usize }
}

struct BuildingArea([PatternLine; 5]);

struct PatternLine {
    state: PatternState,
    length: u8
}

enum PatternState {
    Free,
    Taken(Tile, u8)
}
