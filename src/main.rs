#![allow(unused_imports)]
use std::io::Write;
use std::{
    cell::RefCell,
    io::{stdin, stdout},
    rc::Rc,
};

use furnace::model::{player::PlayerView, Game};
use furnace::visor::layout::Layout;
use furnace::visor::view::TextView;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use furnace::model::auction_house::AuctionHouseView;
use furnace::visor::terminal_writer::TermionBackend;
use furnace::{
    model::{player::Player, AuctionHouse},
    visor::{Engine, UserInput},
};

fn main() {
    let hello = TextView::new(String::from("Hello\nWorld"));
    let world = TextView::new(String::from("Bye World"));
    let stuff = [hello, world]
        .into_iter()
        .map(|x| Box::new(x) as Box<_>)
        .collect();

    let layout = Box::new(Layout::horizontal(0, stuff));
    let backend = TermionBackend::new(Box::new(stdout()));
    let mut engine = Engine::new(backend, layout);

    /*
    let ah = Rc::new(RefCell::new(AuctionHouse::default()));
    let ah_view = AuctionHouseView::new(ah.clone());
    let players = ["Alice", "Bob"]
        .into_iter()
        .map(|name| Rc::new(RefCell::new(Player::with_name(String::from(name)))))
        .collect::<Vec<_>>();

    let p1_view = PlayerView::new(players[0].clone());
    let p2_view = PlayerView::new(players[1].clone());
    let mut game = Game::new(ah, players);

    let mut engine = Engine::new(TerminalWriter::new(stdout), Box::new(p1_view));
    engine.add_component(Box::new(p2_view));
    engine.add_component(Box::new(ah_view));
    */

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}", termion::cursor::Hide).unwrap();

    for c in stdin.keys() {
        engine.render();

        match c.unwrap() {
            /*
            termion::event::Key::Backspace => todo!(),
            termion::event::Key::Left => todo!(),
            termion::event::Key::Right => todo!(),
            termion::event::Key::Up => todo!(),
            termion::event::Key::Down => todo!(),
            termion::event::Key::Esc => todo!(),
            */
            termion::event::Key::Char(c) => {
                if c == 'q' {
                    break;
                }
                let result = engine.trigger(UserInput::Character(c));
                if let Some(_appevent) = result {
                    //game.handle_app_event(appevent);
                }
            }
            _ => {}
        }
    }
}
