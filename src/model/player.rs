use std::usize;

use super::{Tile, Pickable, bag::Bag};

pub struct Player {
    name: String,
    building_area: BuildingArea,
}

impl Player {
    pub fn new(name: String, building_area: BuildingArea) -> Self {
        Self {
            name,
            building_area,
        }
    }

    pub fn default_with_name(name: String) -> Self {
        Self::new(name, BuildingArea::new())
    }
}

pub struct BuildingArea([PatternLine; 5]);

impl BuildingArea {
    pub fn new() -> Self {
        Self([1, 2, 3, 4, 5].map(PatternLine::new_free))
    }

    pub fn get_row(&self, row_number: usize) -> &PatternLine {
        &self.0[row_number]
    }
}

impl Default for BuildingArea {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, PartialEq)]
pub enum PatternLine {
    Free {
        length: usize,
    },
    Taken {
        tile: Tile,
        length: usize,
        taken: usize,
    },
}

impl PatternLine {
    pub fn new_free(length: usize) -> Self {
        Self::Free { length }
    }

    pub fn new_taken(tile: Tile, length: usize, taken: usize) -> Self {
        Self::Taken {
            tile,
            length,
            taken,
        }
    }

    pub fn can_accept(&self, what: Tile, how_many: usize) -> bool {
        match self {
            PatternLine::Free { length } => how_many <= *length,
            PatternLine::Taken {
                tile,
                length,
                taken,
            } => tile == &what && taken + how_many <= *length,
        }
    }

    pub fn accept(&mut self, what: Tile, how_many: usize) -> Result<usize, String> {
        match self {
            PatternLine::Free { length } => {
                if *length >= how_many {
                    let remainder: usize = (how_many as i8 - *length as i8).try_into().unwrap_or(0);
                    *self = PatternLine::Taken {
                        tile: what,
                        length: *length,
                        taken: how_many,
                    };
                    return Ok(remainder);
                }
                return Err(format!(
                    "Not enough space for {} tiles, I only have room for {}",
                    how_many, length
                ));
            }
            PatternLine::Taken {
                tile,
                length,
                taken,
            } if *tile == what => {
                let can_take = *length - *taken;
                if how_many > *taken {
                    return Err(format!(
                        "Not enough space to hold {}, only have {}",
                        how_many, can_take
                    ));
                }
                *taken += how_many;
                let remainder: usize = (how_many as i8 - can_take as i8).try_into().unwrap_or(0);
                Ok(remainder)
            }
            PatternLine::Taken { tile, .. } => Err(format!(
                "This line is taken by color {}, cannot accept {}",
                tile, what
            )),
        }
    }
}

pub struct FloorLine([Pickable; 6]);

impl FloorLine {

    pub fn reset(&mut self, _bag: &mut Bag) -> u8 {
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::model::Tile;
    use std::str::FromStr;
    use test_case::test_case;

    use super::PatternLine;

    impl FromStr for PatternLine {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let length = s.chars().count();
            if !(1..=5).contains(&length) {
                return Err(String::from(
                    "Invalid length to convert to PatternLine, must be 1 <= len <= 5",
                ));
            }
            let first_tile = s.chars().next().unwrap();
            if first_tile == '-' {
                return Ok(PatternLine::new_free(length));
            }
            let mut tile_count = 0;
            for c in s.chars().filter(|c| c != &'-') {
                if c != first_tile {
                    return Err(format!(
                        "Mixed characters are not allowed, got {}, expected {}",
                        c, first_tile
                    ));
                } else {
                    tile_count += 1;
                }
            }
            Ok(PatternLine::new_taken(
                first_tile.to_string().parse()?,
                length,
                tile_count,
            ))
        }
    }

    #[test_case("-----", PatternLine::new_free(5); "empty patternline gets parsed")]
    #[test_case("YY--", PatternLine::new_taken(Tile::Yellow, 4, 2); "patternline with some tiles get parsed")]
    fn test_patternline_parsing(pattern: &str, expected: PatternLine) {
        let pline: PatternLine = pattern.parse().unwrap();
        pretty_assertions::assert_eq!(pline, expected);
    }

    #[test_case("YY--", (Tile::Yellow, 2, true); "can accept exactly until full")]
    #[test_case("YY--", (Tile::Yellow, 1, true); "can accept less than full")]
    #[test_case("YY--", (Tile::Yellow, 3, false); "can't accept too many")]
    #[test_case("YY--", (Tile::Red, 1, false); "can't accept different color")]
    fn test_patternline_can_accept(pattern: &str, (tile, how_many, expected): (Tile, usize, bool)) {
        let pline: PatternLine = pattern.parse().unwrap();
        assert_eq!(pline.can_accept(tile, how_many), expected);
    }

    #[test]
    fn test_patternline_accept_if_free() {
        let pattern = "----";
        let mut pline: PatternLine = pattern.parse().unwrap();
        let remainder = pline.accept(Tile::Yellow, 2);
        assert_eq!(remainder, Ok(0));
        pretty_assertions::assert_eq!(
            pline,
            PatternLine::Taken {
                tile: Tile::Yellow,
                length: 4,
                taken: 2
            }
        );
    }

    #[test]
    fn test_patternline_accept_more() {
        let pattern = "YY--";
        let mut pline: PatternLine = pattern.parse().unwrap();
        pretty_assertions::assert_eq!(
            pline,
            PatternLine::Taken {
                tile: Tile::Yellow,
                length: 4,
                taken: 2
            }
        );
        let remainder = pline.accept(Tile::Yellow, 1);
        assert_eq!(remainder, Ok(0));
        pretty_assertions::assert_eq!(
            pline,
            PatternLine::Taken {
                tile: Tile::Yellow,
                length: 4,
                taken: 3
            }
        );
    }
}
