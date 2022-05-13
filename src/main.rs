use std::io::Write;
use std::{
    cell::RefCell,
    io::{stdin, stdout},
    rc::Rc,
};

use model::{player::PlayerView, Game};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::model::auction_house::AuctionHouseView;
use crate::visor::terminal_writer::{TerminalWriter, TermionBackend};
use crate::{
    model::{player::Player, AuctionHouse},
    visor::{Engine, UserInput},
};

mod model;
mod visor;

fn main() {
    let ah = Rc::new(RefCell::new(AuctionHouse::default()));
    let ah_view = AuctionHouseView::new(ah.clone());
    let players = ["Alice", "Bob"]
        .into_iter()
        .map(|name| Rc::new(RefCell::new(Player::with_name(String::from(name)))))
        .collect::<Vec<_>>();

    let p1_view = PlayerView::new(players[0].clone());
    let p2_view = PlayerView::new(players[1].clone());
    let mut game = Game::new(ah, players);

    let mut engine = Engine::with_writer(TerminalWriter::with_backend(TermionBackend::default()));
    engine.add_component(Box::new(p1_view));
    engine.add_component(Box::new(p2_view));
    engine.add_component(Box::new(ah_view));

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    for c in stdin.keys() {
        engine.render(&mut stdout);

        match c.unwrap() {
            termion::event::Key::Backspace => todo!(),
            termion::event::Key::Left => todo!(),
            termion::event::Key::Right => todo!(),
            termion::event::Key::Up => todo!(),
            termion::event::Key::Down => todo!(),
            termion::event::Key::Char(c) => {
                if c == 'q' {
                    break;
                }
                let result = engine.trigger(UserInput::Character(c));
                if let Some(appevent) = result {
                    game.handle_app_event(appevent);
                }
            }
            termion::event::Key::Esc => todo!(),
            _ => {}
        }
    }
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
