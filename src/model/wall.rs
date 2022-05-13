use crate::visor::{renderer, Component};

use super::{player::Slot, Tile};

const AREA_LENGTH: usize = 5;

pub struct Wall {
    slots: [[Slot; AREA_LENGTH]; AREA_LENGTH],
    points: u16,
}

pub enum FillResult {
    PointsGained(u16),
    PointsGainedAndGameOver(u16),
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

    pub fn fill_slot(&mut self, row: usize, col: usize) -> FillResult {
        match self.slots[row][col] {
            Slot::Filled(_) => panic!("You are trying to fill an already filled slot"),
            Slot::Free(tile) => {
                self.slots[row][col] = Slot::Filled(tile);
                let mut adjacents_in_row = 0;
                let mut adjacents_in_col = 0;
                // <-
                if col > 0 {
                    for i in (0..=col - 1).rev() {
                        if self.is_filled((row, i)) {
                            adjacents_in_row += 1;
                        } else {
                            break;
                        }
                    }
                }
                // ->
                if col < AREA_LENGTH {
                    for i in (col + 1)..AREA_LENGTH {
                        if self.is_filled((row, i)) {
                            adjacents_in_row += 1;
                        } else {
                            break;
                        }
                    }
                }
                // ^
                if row > 0 {
                    for i in (0..=row - 1).rev() {
                        if self.is_filled((i, col)) {
                            adjacents_in_col += 1;
                        } else {
                            break;
                        }
                    }
                }
                // v
                if row < AREA_LENGTH {
                    for i in (row + 1)..AREA_LENGTH {
                        if self.is_filled((i, col)) {
                            adjacents_in_col += 1;
                        } else {
                            break;
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
                self.points += gained;
                if over {
                    FillResult::PointsGainedAndGameOver(gained)
                } else {
                    FillResult::PointsGained(gained)
                }
            }
        }
    }

    pub fn count_points(&self) -> u16 {
        self.points
    }

    fn is_filled(&self, (row, col): (usize, usize)) -> bool {
        match self.slots[row][col] {
            Slot::Filled(_) => true,
            Slot::Free(_) => false,
        }
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
            writer.set_cursor_to((0, i as u16 + 1).into());
        }
    }

    fn declare_dimensions(&self) -> (u16, u16) {
        (5, 5)
    }
}

mod tests {
    use std::collections::BTreeMap;

    use test_case::test_case;

    use super::{Wall, AREA_LENGTH};

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
                wall.fill_slot(row, col);
            }
            wall
        }
    }

    #[allow(dead_code)]
    fn expect_points(wall: &Wall, expected_points: u16) {
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
    // TODO diagonal
    fn test_counting_score(input: &str, expected: u16) {
        let wall = Wall::from_string(input.trim());
        expect_points(&wall, expected);
    }
}
