use crate::{
    model::{buildingarea::BuildingAreaView, player::Player, Game, GameState},
    visor::{layout::Layout, renderer::RootedRenderer, view::PanelBuilder, Component},
};

pub struct PlayerView<'a> {
    player: &'a Player,
    selected_building_row: Option<usize>,
}

impl<'a> PlayerView<'a> {
    pub fn new(player: &'a Player, selected_building_row: Option<usize>) -> Self {
        Self {
            player,
            selected_building_row,
        }
    }
}

impl<'a> Component for PlayerView<'a> {
    fn render(&self, writer: &mut RootedRenderer) {
        BuildingAreaView::new(self.player.get_buildingarea(), self.selected_building_row)
            .render(writer);
    }

    fn declare_dimensions(&self) -> (u16, u16) {
        (20, 7)
    }
}

pub struct PlayerAreaView<'a> {
    players: &'a [Player],
    current_player_id: usize,
    selected_building_row: Option<usize>,
}

impl<'a, const N: usize> From<&'a Game<N>> for PlayerAreaView<'a> {
    fn from(game: &'a Game<N>) -> Self {
        let players = game.get_players();
        match game.state {
            GameState::PickSource | GameState::PickTileFromSource { .. } => {
                PlayerAreaView::new(players, game.player_id, None)
            }
            GameState::PickRowToPutTiles {
                tile: _,
                selected_row_id,
            } => PlayerAreaView::new(players, game.player_id, Some(selected_row_id)),
        }
    }
}

impl<'a> PlayerAreaView<'a> {
    pub fn new(
        players: &'a [Player],
        current_player_id: usize,
        selected_building_row: Option<usize>,
    ) -> Self {
        Self {
            players,
            current_player_id,
            selected_building_row,
        }
    }
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
                    _ => None,
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
