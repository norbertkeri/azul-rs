use crate::{visor::{Component, view::PanelBuilder, layout::Layout}, model::player::{Player, BuildingAreaView}};

pub struct PlayerView<'a> {
    player: &'a Player
}

impl<'a> PlayerView<'a> {
    pub fn new(player: &'a Player) -> Self { Self { player } }
}

impl<'a> Component for PlayerView<'a> {
    fn render(&self, writer: &mut dyn crate::visor::terminal_writer::TerminalBackend) {
        BuildingAreaView::new(self.player.get_buildingarea()).render(writer);
    }

    fn declare_dimensions(&self) -> (u16, u16) {
        (30, 10)
    }
}

pub struct PlayerAreaView<'a> {
    players: &'a [Player]
}

impl<'a> PlayerAreaView<'a> {
    pub fn new(players: &'a [Player]) -> Self { Self { players } }
}

impl<'a> Component for PlayerAreaView<'a> {
    fn render(&self, writer: &mut dyn crate::visor::terminal_writer::TerminalBackend) {
        let players: Vec<Box<dyn Component>> = self.players.iter().map(|p| {
            let p = PanelBuilder::default()
                .name(p.get_name().to_owned())
                .padding(0)
                .component(Box::new(PlayerView::new(p)))
                .build().unwrap();

            Box::new(p) as Box<_>
        }).collect();
        let panel = PanelBuilder::default()
            .name(String::from("Player area"))
            .component(Box::new(Layout::horizontal(0, players)))
            .build()
            .unwrap();

        // TileAreaView

        panel.render(writer);
    }

    fn declare_dimensions(&self) -> (u16, u16) {
        let length = 30 * self.players.len() as u16;
        (length, 30)
    }
}
