#![warn(clippy::pedantic)]
use std::io::Write;
use std::{
    cell::RefCell,
    io::{stdin, stdout},
    rc::Rc,
};

use azulrs::model::player::Player;
use azulrs::model::view::GameView;
use azulrs::model::Game;
use azulrs::visor::backend::TermionBackend;
use azulrs::visor::{Engine, UserInput};
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
    write!(stdout, "{}", termion::cursor::Hide).unwrap();

    engine.render();
    for c in stdin.keys() {
        stdout.flush().unwrap();

        let input = match c.unwrap() {
            termion::event::Key::Backspace | termion::event::Key::Esc => UserInput::Back,
            termion::event::Key::Char(c) => UserInput::from_char(c),
            _ => UserInput::Noop,
        };

        let is_over = game.borrow_mut().handle(input);
        if is_over {
            break;
        }
        engine.render();
    }
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
