use std::{cell::RefCell, rc::Rc};

use crate::visor::{terminal_writer::TerminalBackend, Component};

use super::{Card, Resources};

pub struct Player {
    name: String,
    cards: Vec<Card>,
    resources: Resources,
}

impl Player {
    pub fn new(name: String, cards: Vec<Card>, resources: Resources) -> Self {
        Self {
            name,
            cards,
            resources,
        }
    }

    pub fn with_name(name: String) -> Self {
        Self {
            name,
            cards: Default::default(),
            resources: Default::default(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

pub struct PlayerView {
    player: Rc<RefCell<Player>>,
}

impl PlayerView {
    pub fn new(player: Rc<RefCell<Player>>) -> Self {
        Self { player }
    }
}

impl Component for PlayerView {
    fn render(&self, writer: &mut dyn TerminalBackend) {
        writer.write(&format!(
            "Player area for {}\nBut could be for anyone",
            self.player.borrow().get_name()
        ));
    }

    fn declare_dimensions(&self) -> (u16, u16) {
        (32, 16)
    }
}
