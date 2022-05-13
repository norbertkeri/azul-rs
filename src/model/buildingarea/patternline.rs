use super::Tile;
use crate::visor::{renderer::RootedRenderer, Component};

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
        if length < taken {
            panic!("Cannot create a taken patternline that has more taken blocks ({}), than its length ({})", taken, length);
        }
        Self::Taken {
            tile,
            length,
            taken,
        }
    }

    pub(super) fn can_accept(&self, what: Tile) -> bool {
        match self {
            PatternLine::Free { .. } => true,
            PatternLine::Taken { tile, .. } if tile != &what => false,
            PatternLine::Taken {
                tile: _,
                length,
                taken,
            } => length > taken,
        }
    }

    pub(super) fn accept(&mut self, what: Tile, how_many: usize) -> Result<usize, String> {
        match *self {
            PatternLine::Free { length } => {
                let can_take = std::cmp::min(length, how_many);
                let remainder: usize = how_many - can_take;
                *self = PatternLine::Taken {
                    tile: what,
                    length,
                    taken: can_take,
                };
                Ok(remainder)
            }
            PatternLine::Taken {
                tile,
                length,
                ref mut taken,
            } if tile == what => {
                let can_take = std::cmp::min(length - *taken, how_many);
                *taken += can_take;
                let remainder: usize = how_many - can_take;
                Ok(remainder)
            }
            PatternLine::Taken { tile, .. } => Err(format!(
                "This line is taken by color {}, cannot accept {}",
                tile, what
            )),
        }
    }

    pub fn length(&self) -> usize {
        match *self {
            PatternLine::Free { length } => length,
            PatternLine::Taken { length, .. } => length,
        }
    }
}

#[derive(Debug)]
pub struct PatternLineView<'a> {
    line: &'a PatternLine,
    selected: bool,
}

impl<'a> PatternLineView<'a> {
    pub fn new(line: &'a PatternLine, selected: bool) -> Self {
        Self { line, selected }
    }
}

impl<'a> From<PatternLineView<'a>> for Box<dyn Component + 'a> {
    fn from(val: PatternLineView<'a>) -> Self {
        Box::new(val)
    }
}

impl<'a> Component for PatternLineView<'a> {
    fn render(&self, writer: &mut RootedRenderer) {
        let mut output = match *self.line {
            PatternLine::Free { length } => {
                format!("{: >5}", "☐".repeat(length))
            }
            PatternLine::Taken {
                tile,
                length,
                taken,
            } => {
                let mut output = String::from(&"☐".repeat(length - taken));
                output.push_str(&tile.to_string().repeat(taken));
                output
            }
        };
        if self.selected {
            output = format!("-> {}", output);
        }
        writer.write(&format!("{: >8}", output));
    }

    fn declare_dimensions(&self) -> (u16, u16) {
        (self.line.length().try_into().unwrap(), 1)
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

    #[test_case("YY--", (Tile::Yellow, true); "can accept to full")]
    #[test_case("YY--", (Tile::Yellow, true); "can accept less than full")]
    #[test_case("YY--", (Tile::Red, false); "can't accept different color")]
    #[test_case("YYYY", (Tile::Yellow, false); "can't accept if full")]
    fn test_patternline_can_accept(pattern: &str, (tile, expected): (Tile, bool)) {
        let pline: PatternLine = pattern.parse().unwrap();
        assert_eq!(pline.can_accept(tile), expected);
    }

    #[test_case("----", (Tile::Yellow, 2), 0, PatternLine::Taken { tile: Tile::Yellow, length: 4, taken: 2 }; "accept if free")]
    #[test_case("YY--", (Tile::Yellow, 1), 0, PatternLine::Taken { tile: Tile::Yellow, length: 4, taken: 3 }; "accept more if taken")]
    #[test_case("YY--", (Tile::Yellow, 3), 1, PatternLine::Taken { tile: Tile::Yellow, length: 4, taken: 4 }; "accept too many")]
    fn test_patternline_accept(
        pattern: &str,
        (tile_to_accept, how_many): (Tile, usize),
        expected_remainder: usize,
        expected_state: PatternLine,
    ) {
        let mut pline: PatternLine = pattern.parse().unwrap();
        let remainder = pline.accept(tile_to_accept, how_many);
        pretty_assertions::assert_eq!(remainder, Ok(expected_remainder));
        pretty_assertions::assert_eq!(pline, expected_state);
    }
}
