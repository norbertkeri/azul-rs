#![allow(unused_imports)]
use std::io::Write;
use std::{
    cell::RefCell,
    io::{stdin, stdout},
    rc::Rc,
};

use furnace::model::player::Player;
use furnace::model::view::{FactoryView, GameView};
use furnace::model::{Factory, Game, Tile};
use furnace::visor::backend::TermionBackend;
use furnace::visor::layout::Layout;
use furnace::visor::view::{Panel, PanelBuilder, TextView};
use furnace::visor::{Component, Engine, UserInput};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let players = [
        Player::default_with_name("Alice".into()),
        Player::default_with_name("Bob".into()),
    ];
    let game = Rc::new(RefCell::new(Game::for_players(players)));
    let game_view = GameView { game: game.clone() };
    let backend = TermionBackend::new(Box::new(stdout()));
    let mut engine = Engine::new(backend, Box::new(game_view) as Box<_>);

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    //let mut stdout = stdout();
    write!(stdout, "{}", termion::cursor::Hide).unwrap();

    engine.render();
    for c in stdin.keys() {
        stdout.flush().unwrap();

        match c.unwrap() {
            termion::event::Key::Backspace => {
                engine.trigger(UserInput::Back);
            }
            termion::event::Key::Char(c) => {
                let result = match c {
                    'q' => {
                        break;
                    }
                    '\n' => engine.trigger(UserInput::Confirm),
                    direction @ ('j' | 'k') => engine.trigger(UserInput::Character(direction)),
                    _ => None,
                };
                if let Some(appevent) = result {
                    game.borrow_mut().handle(appevent);
                }
            }
            _ => {}
        }
        engine.render();
    }
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
