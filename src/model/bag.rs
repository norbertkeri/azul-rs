use super::{Factory, Tile, TILE_PER_FACTORY};
use rand::{prelude::SliceRandom, thread_rng};
use std::cmp::min;

#[derive(Debug)]
pub struct Bag {
    tiles: Vec<Tile>,
    discards: Vec<Tile>,
}

impl Bag {
    pub fn new(tiles: Vec<Tile>, discards: Vec<Tile>) -> Self {
        Self { tiles, discards }
    }

    fn draw(&mut self, how_many: usize) -> Vec<Tile> {
        let can_draw = min(self.tiles.len(), how_many);
        let drawn: Vec<_> = self.tiles.drain(0..can_draw).collect();
        drawn
    }

    pub fn fill_factory(&mut self, factory: &mut Factory) {
        let mut new_tiles = match factory.0 {
            Some(_) => panic!("You tried filling a non-empty factory"),
            None => {
                let mut some_tiles = self.draw(TILE_PER_FACTORY);
                if some_tiles.len() == TILE_PER_FACTORY {
                    some_tiles
                } else {
                    self.reshuffle();
                    let mut rest = self.draw(TILE_PER_FACTORY - some_tiles.len());
                    rest.append(&mut some_tiles);
                    rest
                }
            }
        };
        assert!(new_tiles.len() == TILE_PER_FACTORY);
        new_tiles.sort();
        factory.0 = Some(new_tiles.try_into().unwrap());
    }

    fn reshuffle(&mut self) {
        self.tiles = self.discards.drain(..).collect();
        self.tiles.shuffle(&mut thread_rng());
    }

    // restrict visibility on this to some kind of pub(in mod::struct)?
    pub fn discard(&mut self, tiles: &[Tile]) {
        self.discards.extend(tiles);
    }
}

impl Default for Bag {
    fn default() -> Self {
        let mut tiles = Vec::with_capacity(100);
        tiles.append(&mut vec![Tile::Yellow; 20]);
        tiles.append(&mut vec![Tile::Blue; 20]);
        tiles.append(&mut vec![Tile::Red; 20]);
        tiles.append(&mut vec![Tile::White; 20]);
        tiles.append(&mut vec![Tile::Green; 20]);
        tiles.shuffle(&mut thread_rng());
        Bag {
            tiles,
            discards: vec![],
        }
    }
}
