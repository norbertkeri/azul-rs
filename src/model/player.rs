use crate::visor::layout::Layout;

use crate::visor::{renderer, Component};

use super::{
    patternline::{PatternLine, PatternLineView},
    Tile,
};

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

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_buildingarea(&self) -> &BuildingArea {
        &self.building_area
    }

    pub fn get_buildingarea_mut(&mut self) -> &mut BuildingArea {
        &mut self.building_area
    }
}

#[derive(Debug)]
pub enum Slot {
    Filled(Tile),
    Free(Tile),
}

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

type InProgress = [PatternLine; 5];

pub struct BuildingArea {
    in_progress: InProgress,
    wall: Wall,
}

impl BuildingArea {
    pub fn new() -> Self {
        let in_progress = [1, 2, 3, 4, 5].map(PatternLine::new_free);
        Self {
            in_progress,
            wall: Default::default(),
        }
    }

    pub fn get_row(&self, row_number: usize) -> &PatternLine {
        &self.in_progress[row_number]
    }

    pub fn get_row_mut(&mut self, row_number: usize) -> &mut PatternLine {
        &mut self.in_progress[row_number]
    }

    pub fn get_rows(&self) -> &[PatternLine] {
        &self.in_progress
    }

    pub fn get_rows_that_can_accept(
        &self,
        tile: Tile,
        how_many: usize,
    ) -> Vec<(usize, &PatternLine)> {
        self.in_progress
            .iter()
            .enumerate()
            .filter(move |&(i, _p)| self.can_accept(tile, how_many, i))
            .collect()
    }

    pub fn can_accept(&self, what: Tile, how_many: usize, row_number: usize) -> bool {
        // Is the tile already filled on the right side?
        match self.wall.find_slot_for_tile(what, row_number) {
            Slot::Filled(_) => false,
            Slot::Free(_) => {
                // Does the patternline have enough room for this color?
                self.in_progress[row_number].can_accept(what, how_many)
            }
        }
    }
}

impl Default for BuildingArea {
    fn default() -> Self {
        Self::new()
    }
}

pub struct BuildingAreaView<'a> {
    buildingarea: &'a BuildingArea,
    selected: Option<usize>,
}

impl<'a> BuildingAreaView<'a> {
    pub fn new(buildingarea: &'a BuildingArea, selected: Option<usize>) -> Self {
        Self {
            buildingarea,
            selected,
        }
    }
}

impl<'a> Component for BuildingAreaView<'a> {
    fn render(&self, writer: &mut renderer::RootedRenderer) {
        let panel = Layout::horizontal(
            0,
            vec![
                Box::new(InProgressView::new(
                    self.selected,
                    self.buildingarea.get_rows(),
                )),
                Box::new(WallView::new(&self.buildingarea.wall)),
            ],
        );
        panel.render(writer);
    }

    fn declare_dimensions(&self) -> (u16, u16) {
        (15, 5)
    }
}

struct InProgressView<'a> {
    selected: Option<usize>,
    in_progress: &'a [PatternLine],
}

impl<'a> InProgressView<'a> {
    fn new(selected: Option<usize>, in_progress: &'a [PatternLine]) -> Self {
        Self {
            selected,
            in_progress,
        }
    }
}

impl<'a> Component for InProgressView<'a> {
    fn render(&self, writer: &mut renderer::RootedRenderer) {
        for (i, pl) in self.in_progress.iter().enumerate() {
            let is_selected = self.selected.map(|x| x == i).unwrap_or(false);
            PatternLineView::new(pl, is_selected).render(writer);
            let next = (i + 1) as u16;
            writer.set_cursor_to((0, next).into());
        }
    }

    fn declare_dimensions(&self) -> (u16, u16) {
        (10, 5)
    }
}
