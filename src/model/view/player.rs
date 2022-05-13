use crate::{
    model::{player::{BuildingAreaView, Player}, Game, GameState},
    visor::{layout::Layout, view::PanelBuilder, Component, renderer::RootedRenderer},
};

pub struct PlayerView<'a> {
    player: &'a Player,
    selected_building_row: Option<usize>
}

impl<'a> PlayerView<'a> {
    pub fn new(player: &'a Player, selected_building_row: Option<usize>) -> Self {
        Self { player, selected_building_row }
    }
}

impl<'a> Component for PlayerView<'a> {
    fn render(&self, writer: &mut RootedRenderer) {
        BuildingAreaView::new(self.player.get_buildingarea(), self.selected_building_row).render(writer);
    }

    fn declare_dimensions(&self) -> (u16, u16) {
        (20, 8)
    }
}

struct PlayerAreaViewSelection {
    player_id: usize,
    building_row_id: usize
}

pub struct PlayerAreaView<'a> {
    players: &'a [Player],
    current_player_id: usize,
    selected_building_row: Option<usize>
}

impl<'a, const N: usize> From<&'a Game<N>> for PlayerAreaView<'a> {
    fn from(game: &'a Game<N>) -> Self {
        let players = game.get_players();
        match game.state {
            GameState::PickFactory { player_id, current_factory: _ } => {
                PlayerAreaView::new(players, player_id, None)
            },
            GameState::PickTileFromFactory { player_id, factory_id: _, selected_tile: _ } => {
                PlayerAreaView::new(players, player_id, None)
            },
            GameState::PickRowToPutTiles { player_id, factory_id: _, tile: _, selected_row_id } => {
                PlayerAreaView::new(players, player_id, Some(selected_row_id))
            }
        }
    }
}

impl<'a> PlayerAreaView<'a> {
    pub fn new(players: &'a [Player], current_player_id: usize, selected_building_row: Option<usize>) -> Self { Self { players, current_player_id, selected_building_row } }
}


impl<'a> Component for PlayerAreaView<'a> {
    fn render(&self, writer: &mut RootedRenderer) {
        let players: Vec<Box<dyn Component>> = self
            .players
            .iter()
            .enumerate()
            .map(|(i, player)| {
                let is_active_player = i == self.current_player_id;
                let selected_row = match (is_active_player, self.selected_building_row) {
                    (true, Some(row_number)) => Some(row_number),
                    _ => None
                };
                let p = PanelBuilder::default()
                    .name(player.get_name().to_owned())
                    .padding(0)
                    .component(Box::new(PlayerView::new(player, selected_row)))
                    .build()
                    .unwrap();

                Box::new(p) as Box<_>
            })
            .collect();
        let panel = PanelBuilder::default()
            .name(String::from("Player area"))
            .component(Box::new(Layout::horizontal(0, players)))
            .build()
            .unwrap();

        // TileAreaView

        panel.render(writer);
    }

    fn declare_dimensions(&self) -> (u16, u16) {
        let length = 23 * self.players.len() as u16;
        (length, 12)
    }
}
