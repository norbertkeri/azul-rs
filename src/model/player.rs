use std::usize;

use super::patternline::PatternLine;

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

pub struct BuildingAreaView<'a> {
    buildingarea: &'a BuildingArea
}
