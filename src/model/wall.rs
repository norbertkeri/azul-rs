use crate::visor::{renderer, Component};

use super::{player::Slot, Tile};

const AREA_LENGTH: usize = 5;

pub struct Wall {
    slots: [[Slot; AREA_LENGTH]; AREA_LENGTH],
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
        Self { slots }
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
