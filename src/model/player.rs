use super::buildingarea::BuildingArea;

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
        Self::new(name, BuildingArea::default())
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
