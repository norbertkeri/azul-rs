#![allow(dead_code)]
use std::{cell::RefCell, rc::Rc};

use self::player::Player;

pub mod player;
pub mod auction_house;

enum CardEffect {
    Income(Resources),
}

enum CardState {
    Basic,
    Upgraded,
}

impl Default for CardState {
    fn default() -> Self {
        Self::Basic
    }
}

pub struct Card {
    state: CardState,
    effects_basic: Vec<CardEffect>,
    effects_upgraded: Vec<CardEffect>,
}

impl Default for Card {
    fn default() -> Self {
        Self {
            state: Default::default(),
            effects_basic: Default::default(),
            effects_upgraded: Default::default(),
        }
    }
}

pub struct Resources {
    oil: u8,
    steel: u8,
    coal: u8,
}

impl Default for Resources {
    fn default() -> Self {
        Self {
            oil: Default::default(),
            steel: Default::default(),
            coal: Default::default(),
        }
    }
}

pub struct Game {
    auction_house: Rc<RefCell<AuctionHouse>>,
    players: Vec<Rc<RefCell<Player>>>,
}

impl Game {
    pub fn new(
        auction_house: Rc<RefCell<AuctionHouse>>,
        players: Vec<Rc<RefCell<Player>>>,
    ) -> Self {
        Self {
            auction_house,
            players,
        }
    }

    pub fn handle_app_event(&mut self, event: AppEvent) {
        match event {
            AppEvent::TokenPlaced => todo!(),
            AppEvent::CardDrawn => {
                self.auction_house.borrow_mut().add_card(Card::default());
            }
        }
    }
}

pub struct AuctionHouse {
    cards: Vec<Card>,
}

impl AuctionHouse {
    pub fn num_of_cards(&self) -> usize {
        self.cards.len()
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card)
    }
}

impl Default for AuctionHouse {
    fn default() -> Self {
        Self {
            cards: Default::default(),
        }
    }
}

pub enum AppEvent {
    TokenPlaced,
    CardDrawn,
}
