use std::borrow::Cow;

use crate::visor::Component;

use super::Tile;

#[derive(Default)]
pub struct FloorLine([Option<Tile>; 7]);

impl FloorLine {
    pub fn calculate_minus_points(&self) -> u8 {
        self.0.iter().enumerate().fold(0, |acc, (i, tile)| {
            acc + tile.map(|_| Self::points_for_slot(i)).unwrap_or(0)
        })
    }

    pub fn reset(&mut self) {
        *self = Default::default();
    }

    pub fn has_first_player_token(&self) -> bool {
        self.0.iter().any(|slot| match slot {
            Some(tile) => *tile == Tile::FirstPlayer,
            None => false,
        })
    }

    fn points_for_slot(slot_id: usize) -> u8 {
        match slot_id {
            0..=1 => 1,
            2..=4 => 2,
            _ => 3,
        }
    }

    #[must_use]
    pub fn add_tiles(&mut self, tiles: &[Tile]) -> Vec<Tile> {
        let mut remaining = tiles.to_vec();

        for slot in self.0.iter_mut() {
            if slot.is_none() {
                if remaining.is_empty() {
                    return remaining;
                } else {
                    let tile = remaining.remove(0);
                    *slot = Some(tile);
                }
            }
        }

        remaining
    }
}

pub struct FloorLineView<'a> {
    floorline: &'a FloorLine,
}

impl<'a> FloorLineView<'a> {
    pub fn new(floorline: &'a FloorLine) -> Self {
        Self { floorline }
    }
}

impl<'a> Component for FloorLineView<'a> {
    fn render(&self, writer: &mut crate::visor::renderer::RootedRenderer) {
        for (i, _slot) in self.floorline.0.iter().enumerate() {
            writer.write(&FloorLine::points_for_slot(i).to_string());
        }
        writer.reset_cursor_to_root();
        writer.write("\n");
        for slot in self.floorline.0.iter() {
            let s = match slot {
                Some(tile) => Cow::from(tile.to_string()),
                None => Cow::from("☐"),
            };
            writer.write(&s);
        }
    }

    fn declare_dimensions(&self) -> (u16, u16) {
        (7, 2)
    }
}

#[cfg(test)]
mod tests {
    use super::FloorLine;
    use crate::model::Tile;
    use test_case::test_case;

    #[test]
    fn test_calculate_minus_points() {
        let mut fl = FloorLine::default();
        fl.add_tiles(&[Tile::Yellow; 7]);
        assert_eq!(fl.calculate_minus_points(), 14);
    }

    #[test_case(&[Tile::Yellow; 7], vec![]; "adding to empty")]
    #[test_case(&[Tile::Yellow; 9], vec![Tile::Yellow, Tile::Yellow]; "adding too much")]
    fn test_adding_tiles(tiles: &[Tile], expected_remainder: Vec<Tile>) {
        let mut fl = FloorLine::default();
        let remaining = fl.add_tiles(tiles);
        assert_eq!(remaining, expected_remainder);
    }

    #[test]
    fn test_correct_tiles_are_returned() {
        let mut fl = FloorLine::default();
        fl.add_tiles(&[Tile::Yellow; 6]);
        let remaining = fl.add_tiles(&[Tile::Red, Tile::Blue, Tile::Green]);
        assert_eq!(remaining, vec![Tile::Blue, Tile::Green]);
    }
}
