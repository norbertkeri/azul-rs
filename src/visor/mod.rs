#![allow(dead_code)]

pub mod layout;
pub mod terminal_writer;
pub mod view;

use self::terminal_writer::{DebuggableTerminalBackend, TerminalBackend};
use crate::model::AppEvent;
use std::ops::Add;

pub trait Component {
    fn render(&self, writer: &mut dyn TerminalBackend);
    fn declare_dimensions(&self) -> (u16, u16);
    fn handle(&mut self, _event: &UserInput) -> UserEventHandled {
        UserEventHandled::Noop
    }
}

#[derive(Copy, Clone, Debug)]
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

pub struct Engine<T> {
    writer: T,
    root_component: Box<dyn Component>,
}

pub enum UserEventHandled {
    ViewChange,
    AppEvent(AppEvent),
    Noop,
}

pub enum UserInput {
    Direction,
    Character(char),
    Confirm,
    Back,
}

impl<T> Engine<T>
where
    T: DebuggableTerminalBackend,
{
    pub fn get_contents(&self) -> String {
        self.writer.get_contents()
    }
}

impl<T> Engine<T>
where
    T: TerminalBackend,
{
    pub fn new(writer: T, root_component: Box<dyn Component>) -> Self {
        Self {
            writer,
            root_component,
        }
    }

    pub fn render(&mut self) {
        /*
        ┌─┐
        │ │
        └─┘
            */

        self.writer.clear();
        self.root_component.render(&mut self.writer);
        self.writer.flush();
        /*
        for line in self.root_component.render().lines() {
            self.writer.write(line);
        }
        */
        /*
        for (i, component) in self.components.iter().enumerate() {
            let (width, height) = component.declare_dimensions();
            let starting_corner = Coords(i as u16 * width + component_padding, 1u16);

            self.writer.move_to(starting_corner, sink);
            self.writer.write("┌", sink);
            self.writer.write(&"─".repeat((width - 2).into()), sink);
            self.writer.write("┐", sink);

            let mut total_lines = 1;
            for line in component.render().lines() {
                let line_start = starting_corner + Coords(0, total_lines);
                self.writer.move_to(line_start, sink);
                self.writer.write("│", sink);
                self.writer.write(&" ".repeat(inner_padding), sink);
                self.writer.write(line, sink);
                self.writer.move_to(line_start + Coords(width - 1, 0), sink);
                self.writer.write("│", sink);
                total_lines += 1;
            }
            for i in total_lines..height - 1 {
                let line_start = starting_corner + Coords(0, i);
                let line_end = line_start + Coords(width - 1, 0);
                self.writer.move_to(line_start, sink);
                self.writer.write("│", sink);
                self.writer.move_to(line_end, sink);
                self.writer.write("│", sink);
            }
            self.writer
                .move_to(starting_corner + Coords(0, height - 1), sink);
            self.writer.write("└", sink);
            self.writer.write(&"─".repeat((width - 2).into()), sink);
            self.writer.write("┘", sink);
        }
        */

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
