#![allow(unused_imports)]
use std::io::Write;
use std::{
    cell::RefCell,
    io::{stdin, stdout},
    rc::Rc,
};

use furnace::model::{Factory, Tile};
use furnace::model::view::FactoryView;
use furnace::visor::terminal_writer::TermionBackend;
use furnace::visor::{Component, UserInput, Engine};
use furnace::visor::layout::Layout;
use furnace::visor::view::{TextView, Panel, PanelBuilder};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

fn main() {
    let factories = (0..4).map(|_| {
        FactoryView::new(Rc::new(Factory::new_random()), None)
    });
    let fviews: Vec<Box<dyn Component>> = factories.into_iter()
        .map(|x| Box::new(x) as Box<dyn Component>)
        .collect();

    let x = PanelBuilder::default()
        .name("Factories")
        .padding(1)
        .component(Box::new(Layout::vertical(0, fviews)))
        .build()
        .unwrap();

    let backend = TermionBackend::new(Box::new(stdout()));
    let mut engine = Engine::new(backend, x);

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
