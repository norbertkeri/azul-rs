use std::cmp::min;
use rand::{prelude::SliceRandom, thread_rng};
use super::Tile;

#[derive(Debug)]
pub struct Bag {
    tiles: Vec<Tile>,
    discards: Vec<Tile>
}

impl Bag {
    pub fn new(tiles: Vec<Tile>, discards: Vec<Tile>) -> Self { Self { tiles, discards } }

    pub fn draw(&mut self, how_many: usize) -> Vec<Tile> {
        let can_draw = min(self.tiles.len(), how_many);
        let drawn = self.tiles.drain(0..can_draw).collect();
        drawn
    }

    pub fn reshuffle(&mut self) {
        self.tiles = self.discards.drain(..).collect();
        self.tiles.shuffle(&mut thread_rng());
    }

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
            discards: vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{bag::Bag, Tile};


    #[test]
    pub fn test_drawing_after_reshuffling() {
        let mut bag = Bag::new(vec![Tile::Yellow, Tile::Red], vec![Tile::Blue]);
        let drawn = bag.draw(3);
        assert_eq!(drawn.len(), 2);
        bag.discard(&drawn);
        bag.reshuffle();
        assert_eq!(bag.draw(3).len(), 3);
    }
}
