#![allow(unused_imports)]
use std::io::Write;
use std::{
    cell::RefCell,
    io::{stdin, stdout},
    rc::Rc,
};

use furnace::model::{Factory, Tile, Game, Player};
use furnace::model::view::{FactoryView, GameView};
use furnace::visor::terminal_writer::TermionBackend;
use furnace::visor::{Component, UserInput, Engine};
use furnace::visor::layout::Layout;
use furnace::visor::view::{TextView, Panel, PanelBuilder};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let players = [Player::new("Alice".into()), Player::new("Bob".into())];
    let game = Game::for_players(players);
    let game_view = GameView {
        game: Rc::new(game)
    };
    let backend = TermionBackend::new(Box::new(stdout()));
    let mut engine = Engine::new(backend, Box::new(game_view) as Box<_>);

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}", termion::cursor::Hide).unwrap();

    for c in stdin.keys() {
        engine.render();

        match c.unwrap() {
            termion::event::Key::Backspace => {
                engine.trigger(UserInput::Back);
            },
            /*
            termion::event::Key::Left => todo!(),
            termion::event::Key::Right => todo!(),
            termion::event::Key::Up => todo!(),
            termion::event::Key::Down => todo!(),
            termion::event::Key::Esc => todo!(),
            */
            termion::event::Key::Char(c) => {
                let result = match c {
                    'q' => {
                        write!(stdout, "{}", termion::cursor::Show).unwrap();
                        break;
                    },
                    direction @ ('j' | 'k') => {
                        engine.trigger(UserInput::Character(direction))
                    },
                    _ => None
                };
                if let Some(_appevent) = result {
                    //game.handle_app_event(appevent);
                }
            }
            _ => {}
        }
    }
}
