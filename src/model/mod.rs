#![allow(dead_code)]

use rand::{distributions::Standard, prelude::Distribution};
use std::fmt::Debug;

pub mod view;

pub struct AppEvent {
}

pub struct Player {
    name: String,
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

pub struct Factory(pub [Tile; 4]);

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

struct Game<const N: usize> {
    players: [Player; N]
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
