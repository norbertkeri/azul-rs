use crate::{
    model::{AppEvent, AuctionHouse},
    visor::{Component, UserEventHandled, UserInput},
};
use std::{cell::RefCell, rc::Rc};

pub struct AuctionHouseView {
    house: Rc<RefCell<AuctionHouse>>,
}

impl<'a> AuctionHouseView {
    pub fn new(house: Rc<RefCell<AuctionHouse>>) -> Self {
        Self { house }
    }
}

impl<'a> Component for AuctionHouseView {
    fn render(&self) -> String {
        format!(
            "There is {} cards in the AH",
            &self.house.borrow().num_of_cards()
        )
    }

    fn handle(&mut self, event: &UserInput) -> UserEventHandled {
        match event {
            UserInput::Character(_c) => UserEventHandled::AppEvent(AppEvent::CardDrawn),
            UserInput::Direction | UserInput::Confirm | UserInput::Back => UserEventHandled::Noop,
        }
    }

    fn declare_dimensions(&self) -> (u16, u16) {
        (40, 10)
    }
}
