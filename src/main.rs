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

        let e = match c.unwrap() {
            termion::event::Key::Backspace => engine.trigger(UserInput::Back),
            termion::event::Key::Char(c) => match c {
                'q' => {
                    break;
                }
                '\n' => engine.trigger(UserInput::Confirm),
                direction @ ('j' | 'k') => engine.trigger(UserInput::Character(direction)),
                _ => None,
            },
            _ => None,
        };
        if let Some(appevent) = e {
            game.borrow_mut().handle(appevent);
        }
        engine.render();
    }
    write!(stdout, "{}", termion::cursor::Show).unwrap();
}
