use crate::visor::{
    layout::Layout,
    renderer::{self, RootedRenderer},
    view::{PanelBuilder, TextView},
    Component,
};

use self::{
    floorline::{FloorLine, FloorLineView},
    patternline::{PatternLine, PatternLineView},
    wall::{FillResult, Wall, WallView},
};

use super::{player::Player, CommonArea, Factory, Tile, TileSource};

pub mod floorline;
pub mod patternline;
pub mod wall;

#[derive(Debug)]
pub enum Slot {
    Filled(Tile),
    Free(Tile),
}

type InProgress = [PatternLine; 5];

pub struct BuildingArea {
    in_progress: InProgress,
    wall: Wall,
    floorline: FloorLine,
}

impl Default for BuildingArea {
    fn default() -> Self {
        let in_progress = [1, 2, 3, 4, 5].map(PatternLine::new_free);
        Self {
            in_progress,
            wall: Default::default(),
            floorline: Default::default(),
        }
    }
}

#[must_use]
pub enum IsGameOver {
    Yes,
    No,
}

impl From<IsGameOver> for bool {
    fn from(val: IsGameOver) -> Self {
        match val {
            IsGameOver::Yes => true,
            IsGameOver::No => false,
        }
    }
}

impl BuildingArea {
    pub(super) fn move_tiles_to_wall(&mut self) -> IsGameOver {
        let mut is_game_over = IsGameOver::No;
        for (i, pl) in &mut self.in_progress.iter_mut().enumerate() {
            if pl.is_full() {
                let tile = pl.flush();
                if let FillResult::PointsGainedAndGameOver(_) = self.wall.fill_slot(i, tile) {
                    is_game_over = IsGameOver::Yes;
                }
            }
        }
        is_game_over
    }

    pub fn flush_floorline(&mut self) {
        self.wall.reset_floorline(&mut self.floorline);
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

    pub fn get_rows_that_can_accept(&self, tile: Tile) -> Vec<usize> {
        self.in_progress
            .iter()
            .enumerate()
            .filter(move |&(i, _p)| self.can_accept(tile, i))
            .map(|(i, _p)| i)
            .collect()
    }

    pub fn can_accept(&self, what: Tile, row_number: usize) -> bool {
        // Is the tile already filled on the right side?
        match self.wall.find_slot_for_tile(what, row_number) {
            Slot::Filled(_) => false,
            Slot::Free(_) => {
                // Does the patternline have enough room for this color?
                self.in_progress[row_number].can_accept(what)
            }
        }
    }

    pub fn get_floorline(&self) -> &FloorLine {
        &self.floorline
    }

    pub fn pick_factory(
        &mut self,
        factory: &mut Factory,
        common_area: &mut CommonArea,
        row_number: usize,
        picked_tile: Tile,
    ) -> Result<(), String> {
        let tiles = factory
            .0
            .as_ref()
            .ok_or_else(|| String::from("You tried picking an empty factory"))?;

        if !tiles.contains(&picked_tile) {
            return Err(format!(
                "You tried picking tile {} from a factory that does not have it",
                &picked_tile
            ));
        }
        let pattern_line = &mut self.in_progress[row_number];
        let (picked, non_picked): (Vec<Tile>, Vec<Tile>) =
            tiles.iter().partition(|&tile| tile == &picked_tile);
        let remaining = pattern_line.accept(picked_tile, picked.len())?;
        common_area.add(&non_picked);
        factory.0 = None;
        self.floorline.add_tiles(&vec![picked_tile; remaining]);
        Ok(())
    }

    pub fn pick_from_common_area(
        &mut self,
        row_number: usize,
        picked_tile: Tile,
        how_many: usize,
        first_player: bool,
    ) -> Result<(), String> {
        let pattern_line = &mut self.in_progress[row_number];
        let remaining = pattern_line.accept(picked_tile, how_many)?;
        let mut to_floor = vec![picked_tile; remaining];
        if first_player {
            to_floor.push(Tile::FirstPlayer);
        }

        self.floorline.add_tiles(&to_floor);
        Ok(())
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
    fn render(&self, writer: &mut RootedRenderer) {
        let panel = Layout::vertical(
            0,
            vec![
                Box::new(Layout::horizontal(
                    0,
                    vec![
                        Box::new(InProgressView::new(
                            self.selected,
                            self.buildingarea.get_rows(),
                        )),
                        Box::new(WallView::new(&self.buildingarea.wall)),
                    ],
                )),
                Box::new(FloorLineView::new(self.buildingarea.get_floorline())),
            ],
        );
        panel.render(writer);
    }

    fn declare_dimensions(&self) -> (u16, u16) {
        (15, 7)
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

pub struct ScoreView<'a> {
    players: &'a [Player],
}

impl<'a> ScoreView<'a> {
    pub fn new(players: &'a [Player]) -> Self {
        Self { players }
    }
}

impl<'a> Component for ScoreView<'a> {
    fn render(&self, writer: &mut RootedRenderer) {
        let mut text = String::new();
        for p in self.players {
            let bg = p.get_buildingarea();
            let wall = &bg.wall;
            text.push_str(&format!("{}: {}\n", p.get_name(), wall.count_points()));
        }
        let textarea = TextView::from(text);

        let panel = PanelBuilder::default()
            .component(Box::new(textarea) as Box<_>)
            .name("Score")
            .build()
            .unwrap();

        panel.render(writer);
    }

    fn declare_dimensions(&self) -> (u16, u16) {
        let mut height: u16 = self.players.len().try_into().unwrap();
        height += 2;
        (10, height)
    }
}
