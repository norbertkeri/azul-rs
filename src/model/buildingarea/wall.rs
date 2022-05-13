use std::collections::HashMap;

use crate::{
    model::Tile,
    visor::{renderer, Component},
};

use super::{floorline::FloorLine, Slot};

const AREA_LENGTH: usize = 5;

pub struct Wall {
    slots: [[Slot; AREA_LENGTH]; AREA_LENGTH],
    points: u8,
}

#[derive(Debug, PartialEq)]
#[must_use]
pub enum FillResult {
    PointsGained(u8),
    PointsGainedAndGameOver(u8),
}

impl Wall {
    pub fn find_slot_for_tile(&self, what: Tile, row_number: usize) -> &Slot {
        self.slots[row_number]
            .iter()
            .find(|&slot| match slot {
                Slot::Filled(tile) | Slot::Free(tile) => tile == &what,
            })
            .unwrap()
    }

    pub fn reset_floorline(&mut self, floorline: &mut FloorLine) {
        let minus = floorline.calculate_minus_points();
        let total = if self.points <= minus {
            0
        } else {
            self.points - minus
        };
        self.points = total;
        floorline.reset();
    }

    fn find_col_for_tile(&self, row_number: usize, what: Tile) -> usize {
        self.slots[row_number]
            .iter()
            .position(|slot| match slot {
                Slot::Filled(tile) | Slot::Free(tile) => *tile == what,
            })
            .unwrap()
    }

    pub(super) fn fill_slot(&mut self, row: usize, tile: Tile) -> FillResult {
        let col = self.find_col_for_tile(row, tile);
        match self.slots[row][col] {
            Slot::Filled(_) => panic!("You are trying to fill an already filled slot"),
            Slot::Free(tile) => {
                self.slots[row][col] = Slot::Filled(tile);
                let mut adjacents_in_row = 0;
                let mut adjacents_in_col = 0;
                let mut ranges_row: Vec<Box<dyn Iterator<Item = usize>>> = vec![];
                let mut ranges_col: Vec<Box<dyn Iterator<Item = usize>>> = vec![];

                if col > 0 {
                    ranges_row.push(Box::new((0..=col - 1).rev()));
                }
                if col < AREA_LENGTH {
                    ranges_row.push(Box::new((col + 1)..AREA_LENGTH));
                }
                if row > 0 {
                    ranges_col.push(Box::new((0..=row - 1).rev()));
                }
                if row < AREA_LENGTH {
                    ranges_col.push(Box::new((row + 1)..AREA_LENGTH));
                }

                for range in ranges_row {
                    for i in range {
                        match self.at(row, i) {
                            Slot::Filled(_tile) => {
                                adjacents_in_row += 1;
                            }
                            Slot::Free(_) => break,
                        }
                    }
                }
                for range in ranges_col {
                    for i in range {
                        match self.at(i, col) {
                            Slot::Filled(_tile) => {
                                adjacents_in_col += 1;
                            }
                            Slot::Free(_) => break,
                        }
                    }
                }

                let mut over = false;
                let mut gained = 0;
                if adjacents_in_row == 0 && adjacents_in_col == 0 {
                    gained = 1;
                }
                if adjacents_in_row > 0 {
                    gained += 1 + adjacents_in_row;
                }
                if adjacents_in_col > 0 {
                    gained += 1 + adjacents_in_col;
                }
                if adjacents_in_col == 4 {
                    gained += 7;
                }
                if adjacents_in_row == 4 {
                    gained += 2;
                    over = true;
                }
                if self.has_diagonal() {
                    gained += 10;
                }

                self.points += gained;
                if over {
                    FillResult::PointsGainedAndGameOver(gained)
                } else {
                    FillResult::PointsGained(gained)
                }
            }
        }
    }

    fn has_diagonal(&self) -> bool {
        let mut tiles: HashMap<Tile, u8> = Default::default();
        for row in &self.slots {
            for slot in row {
                match slot {
                    Slot::Filled(tile) => {
                        let entry = tiles.entry(*tile).or_default();
                        *entry += 1;
                    }
                    Slot::Free(_) => {}
                }
            }
        }
        tiles.iter().any(|(_tile, count)| *count == 5)
    }

    pub fn count_points(&self) -> u8 {
        self.points
    }

    fn at(&self, row: usize, col: usize) -> &Slot {
        &self.slots[row][col]
    }
}

impl Default for Wall {
    fn default() -> Self {
        let slots: Vec<_> = (0..5)
            .map(|i| {
                let mut row = [
                    Tile::Yellow,
                    Tile::Red,
                    Tile::Blue,
                    Tile::White,
                    Tile::Green,
                ]
                .map(Slot::Free);
                row.rotate_right(i);
                row
            })
            .collect();
        let slots = slots.try_into().unwrap();
        Self { slots, points: 0 }
    }
}

pub struct WallView<'a> {
    wall: &'a Wall,
}

impl<'a> WallView<'a> {
    pub fn new(wall: &'a Wall) -> Self {
        Self { wall }
    }
}

impl<'a> Component for WallView<'a> {
    fn render(&self, writer: &mut renderer::RootedRenderer) {
        for (i, row) in self.wall.slots.iter().enumerate() {
            for t in row.iter() {
                match t {
                    Slot::Filled(_tile) => writer.write("X"),
                    Slot::Free(tile) => writer.write(&tile.to_string()),
                }
            }
            writer.set_cursor_to((1, i as u16 + 2).into());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::model::{
        buildingarea::{wall::FillResult, Slot},
        Tile,
    };

    use super::{Wall, AREA_LENGTH};
    use std::collections::BTreeMap;
    use test_case::test_case;

    impl Wall {
        pub fn from_string(input: &str) -> Wall {
            let count = input.lines().count();
            if count != AREA_LENGTH {
                panic!("You must have exactly {AREA_LENGTH} lines to create a Wall from a string (you had {count})");
            }
            let mut wall = Wall::default();
            let mut steps = BTreeMap::new();
            for (row, line) in input.lines().enumerate() {
                let linelen = line.len();
                if linelen != AREA_LENGTH {
                    panic!("You must have exactly {AREA_LENGTH} characters in each line, to create a Wall from a string (you had {linelen})");
                }
                for (col, c) in line.chars().enumerate() {
                    let digit = match c.to_digit(10) {
                        Some(digit) => digit,
                        None => panic!("You had a non-number {c} at {row}:{col}"),
                    };
                    if digit == 0 {
                        continue;
                    }
                    steps.entry(digit).or_insert((row, col));
                }
            }
            for (_i, (row, col)) in steps {
                let tile = wall.find_tile_for(row, col);
                let _ = wall.fill_slot(row, tile);
            }
            wall
        }

        fn find_tile_for(&self, row: usize, col: usize) -> Tile {
            match self.slots[row][col] {
                Slot::Filled(tile) | Slot::Free(tile) => tile,
            }
        }
    }

    fn expect_points(wall: &Wall, expected_points: u8) {
        pretty_assertions::assert_eq!(wall.count_points(), expected_points);
    }

    #[test_case("
12300
00000
00000
00000
00000
", 6; "sums row link")]
    #[test_case("
10203
00000
00000
00000
00000
", 3; "sums separate tiles in a row")]
    #[test_case("
01300
00200
00000
00000
00000
", 6; "sums vertical and horizontal link")]
    #[test_case("
13500
00200
00400
00000
00000
", 12; "sums longer vertical and horizontal link")]
    #[test_case("
00100
00200
04756
00300
00000
", 16; "sums cross link")]
    #[test_case("
10000
20000
30000
40000
50000
", 22; "sums whole column")]
    #[test_case("
12345
00000
00000
00000
00000
", 17; "sums whole row")]
    #[test_case("
10000
02000
00300
00040
00005
", 15; "sums diagonal")]
    fn test_counting_score(input: &str, expected: u8) {
        let wall = Wall::from_string(input.trim());
        expect_points(&wall, expected);
    }

    #[test]
    fn test_row_ends_game() {
        let input = "
12340
00000
00000
00000
00000
"
        .trim();
        let mut wall = Wall::from_string(input);
        let tile = wall.find_tile_for(0, 4);
        let result = wall.fill_slot(0, tile);
        pretty_assertions::assert_eq!(result, FillResult::PointsGainedAndGameOver(7));
    }
}
