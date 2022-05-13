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
enum Slot {
    Filled(Tile),
    Free(Tile),
}

const AREA_LENGTH: usize = 5;

pub struct TilingArea {
    slots: [[Slot; AREA_LENGTH]; AREA_LENGTH],
}

impl Default for TilingArea {
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

pub struct TilingAreaView<'a> {
    tiling_area: &'a TilingArea,
}

impl<'a> TilingAreaView<'a> {
    pub fn new(tiling_area: &'a TilingArea) -> Self {
        Self { tiling_area }
    }
}

impl<'a> Component for TilingAreaView<'a> {
    fn render(&self, writer: &mut renderer::RootedRenderer) {
        for (i, row) in self.tiling_area.slots.iter().enumerate() {
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
    tiling_area: TilingArea,
}

impl BuildingArea {
    pub fn new() -> Self {
        let in_progress = [1, 2, 3, 4, 5].map(PatternLine::new_free);
        let tiling_area = Default::default();
        Self {
            in_progress,
            tiling_area,
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
            .filter(move |(_i, p)| p.can_accept(tile, how_many))
            .collect()
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
                Box::new(TilingAreaView::new(&self.buildingarea.tiling_area)),
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
