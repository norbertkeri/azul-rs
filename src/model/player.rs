use std::usize;

use crate::visor::Component;

use super::patternline::{PatternLine, PatternLineView};

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
}

pub struct BuildingArea([PatternLine; 5]);

impl BuildingArea {
    pub fn new() -> Self {
        Self([1, 2, 3, 4, 5].map(PatternLine::new_free))
    }

    pub fn get_row(&self, row_number: usize) -> &PatternLine {
        &self.0[row_number]
    }

    pub fn get_rows(&self) -> &[PatternLine] {
        &self.0
    }
}

impl Default for BuildingArea {
    fn default() -> Self {
        Self::new()
    }
}

pub struct BuildingAreaView<'a> {
    buildingarea: &'a BuildingArea,
}

impl<'a> BuildingAreaView<'a> {
    pub fn new(buildingarea: &'a BuildingArea) -> Self {
        Self { buildingarea }
    }
}

impl<'a> Component for BuildingAreaView<'a> {
    fn render(&self, writer: &mut crate::visor::terminal_writer::RootedRenderer) {
        for (i, pl) in self.buildingarea.get_rows().iter().enumerate() {
            PatternLineView::new(pl).render(writer);
            let next = (i + 1) as u16;
            writer.set_cursor_to((0, next).into());
        }
    }

    fn declare_dimensions(&self) -> (u16, u16) {
        (5, 5)
    }
}
