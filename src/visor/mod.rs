#![allow(dead_code)]

pub mod backend;
pub mod inmemory;
pub mod layout;
pub mod renderer;
pub mod view;

use self::backend::{DebuggableTerminalBackend, TerminalBackend};
use self::renderer::RootedRenderer;
use crate::model::{AppEvent, Direction};
use std::fmt::Debug;
use std::ops::Add;

pub trait Component {
    fn render(&self, writer: &mut RootedRenderer);
    fn declare_dimensions(&self) -> (u16, u16);
    fn handle(&mut self, _event: &UserInput) -> UserEventHandled {
        UserEventHandled::Noop
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Coords(u16, u16);

impl Coords {
    pub fn new(x: u16, y: u16) -> Self {
        Coords(x, y)
    }
}

impl From<(u16, u16)> for Coords {
    fn from(a: (u16, u16)) -> Self {
        Self(a.0, a.1)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Move(i16, i16);

impl Default for Coords {
    fn default() -> Self {
        Self(1, 1)
    }
}

impl Add<(u16, u16)> for Coords {
    type Output = Coords;

    fn add(self, rhs: (u16, u16)) -> Self::Output {
        Coords(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Add for Coords {
    type Output = Coords;

    fn add(self, rhs: Self) -> Self::Output {
        Coords(self.0 + rhs.0, self.1 + rhs.1)
    }
}

pub struct Engine<'a, T: TerminalBackend> {
    backend: T,
    root_component: Box<dyn Component + 'a>,
}

pub enum UserEventHandled {
    ViewChange,
    AppEvent(AppEvent),
    Noop,
}

pub enum UserInput {
    Direction(Direction),
    Character(char),
    Confirm,
    Back,
    Exit,
    Noop,
}

impl UserInput {
    pub fn from_char(c: char) -> Self {
        match c {
            'q' => Self::Exit,
            '\n' => Self::Confirm,
            'j' => Self::Direction(Direction::Next),
            'k' => Self::Direction(Direction::Prev),
            _ => Self::Noop,
        }
    }
}

impl<T> Engine<'_, T>
where
    T: DebuggableTerminalBackend,
{
    pub fn get_contents(&self) -> String {
        self.backend.get_contents()
    }
}

impl<'a, T> Engine<'a, T>
where
    T: TerminalBackend,
{
    pub fn new<W: Into<Box<dyn Component + 'a>>>(backend: T, root_component: W) -> Self {
        Self {
            backend,
            root_component: root_component.into(),
        }
    }

    pub fn render(&mut self) {
        self.backend.clear();
        let mut renderer = RootedRenderer::new(&mut self.backend, Coords(1, 1));
        self.root_component.render(&mut renderer);
        self.backend.flush();

        //sink.flush().unwrap(); // TODO
    }
}
