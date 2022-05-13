#![allow(dead_code)]

pub mod layout;
pub mod terminal_writer;
pub mod view;

use self::terminal_writer::{DebuggableTerminalBackend, TerminalBackend, RootedRenderer};
use crate::model::AppEvent;
use std::ops::Add;
use std::fmt::Debug;

pub trait Component {
    fn render(&self, writer: &mut dyn TerminalBackend);
    fn declare_dimensions(&self) -> (u16, u16);
    fn handle(&mut self, _event: &UserInput) -> UserEventHandled {
        UserEventHandled::Noop
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Coords(u16, u16);

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

impl Add for Coords {
    type Output = Coords;

    fn add(self, rhs: Self) -> Self::Output {
        Coords(self.0 + rhs.0, self.1 + rhs.1)
    }
}

pub struct Engine<'a, T> {
    writer: T,
    root_component: Box<dyn Component + 'a>,
}

pub enum UserEventHandled {
    ViewChange,
    AppEvent(AppEvent),
    Noop,
}

pub enum UserInput {
    Character(char),
    Confirm,
    Back,
}

impl<T> Engine<'_, T>
where
    T: DebuggableTerminalBackend,
{
    pub fn get_contents(&self) -> String {
        self.writer.get_contents()
    }
}

impl<'a, T> Engine<'a, T>
where
    T: TerminalBackend,
{
    pub fn new<W: Into<Box<dyn Component + 'a>>>(writer: T, root_component: W) -> Self {
        Self {
            writer,
            root_component: root_component.into(),
        }
    }

    pub fn render(&mut self) {
        self.writer.clear();
        let mut writer = RootedRenderer::new(&mut self.writer, Coords(1, 1));
        self.root_component.render(&mut writer);
        self.writer.flush();

        //sink.flush().unwrap(); // TODO
    }

    pub fn trigger(&mut self, event: UserInput) -> Option<AppEvent> {
        match self.root_component.handle(&event) {
            UserEventHandled::ViewChange | UserEventHandled::Noop => {}
            UserEventHandled::AppEvent(appevent) => {
                return Some(appevent);
            }
        }

        None
    }
}
