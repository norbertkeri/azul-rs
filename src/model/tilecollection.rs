use super::{Direction, Tile};

pub trait HasTileCollection {
    fn get_tilecollection(&self) -> Box<dyn TileCollection>;
}

pub trait TileCollection {
    fn distinct_tiles(&self) -> Vec<Tile>;
    fn find_adjacent_tile(&self, tile: Tile, direction: Direction) -> Tile;
    fn count_tile(&self, tile: Tile) -> usize;
    fn find_first_tile(&self) -> Option<Tile>;
}

impl<T> TileCollection for T
where
    T: HasTileCollection,
{
    fn distinct_tiles(&self) -> Vec<Tile> {
        self.get_tilecollection().distinct_tiles()
    }

    fn find_adjacent_tile(&self, tile: Tile, direction: Direction) -> Tile {
        self.get_tilecollection()
            .find_adjacent_tile(tile, direction)
    }

    fn count_tile(&self, tile: Tile) -> usize {
        self.get_tilecollection().count_tile(tile)
    }

    fn find_first_tile(&self) -> Option<Tile> {
        self.get_tilecollection().find_first_tile()
    }
}

impl TileCollection for Vec<Tile> {
    fn distinct_tiles(&self) -> Vec<Tile> {
        self.as_slice().distinct_tiles()
    }

    fn find_adjacent_tile(&self, tile: Tile, direction: Direction) -> Tile {
        self.as_slice().find_adjacent_tile(tile, direction)
    }

    fn count_tile(&self, tile: Tile) -> usize {
        self.as_slice().count_tile(tile)
    }

    fn find_first_tile(&self) -> Option<Tile> {
        self.as_slice().find_first_tile()
    }
}

impl<const N: usize> TileCollection for [Tile; N] {
    fn distinct_tiles(&self) -> Vec<Tile> {
        self.as_slice().distinct_tiles()
    }

    fn find_adjacent_tile(&self, tile: Tile, direction: Direction) -> Tile {
        self.as_slice().find_adjacent_tile(tile, direction)
    }

    fn count_tile(&self, tile: Tile) -> usize {
        self.as_slice().count_tile(tile)
    }
    fn find_first_tile(&self) -> Option<Tile> {
        self.as_slice().find_first_tile()
    }
}

impl<'a> TileCollection for &'a [Tile] {
    fn distinct_tiles(&self) -> Vec<Tile> {
        let mut result: Vec<Tile> = Vec::with_capacity(4);
        for t in *self {
            if !result.contains(t) && t != &Tile::FirstPlayer {
                result.push(*t);
            }
        }
        result
    }

    fn find_adjacent_tile(&self, tile: Tile, direction: Direction) -> Tile {
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
    fn count_tile(&self, tile: Tile) -> usize {
        self.iter().filter(|&&t| t == tile).count()
    }

    fn find_first_tile(&self) -> Option<Tile> {
        self.iter().find(|&&t| t != Tile::FirstPlayer).copied()
    }
}
