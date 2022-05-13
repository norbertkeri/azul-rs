#![allow(dead_code)]

pub mod terminal_writer;
pub mod layout;
mod view;

use std::ops::Add;

use crate::model::AppEvent;

use self::terminal_writer::{TerminalBackend, TerminalWriter};

pub trait Renderable {
    fn render(&self);
}

pub trait Component {
    fn render(&self) -> String;
    fn declare_dimensions(&self) -> (u16, u16);
    fn handle(&mut self, _event: &UserInput) -> UserEventHandled {
        UserEventHandled::Noop
    }
}

#[derive(Copy, Clone)]
pub struct Coords(u16, u16);

impl From<(u16, u16)> for Coords {
    fn from(what: (u16, u16)) -> Self {
        Coords(what.0, what.1)
    }
}

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

pub struct Engine<'a, TerminalWriterBackend: TerminalBackend> {
    writer: TerminalWriter<TerminalWriterBackend>,
    components: Vec<Box<dyn Component + 'a>>,
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

impl<'a, TerminalWriterBackend: TerminalBackend> Engine<'a, TerminalWriterBackend> {
    pub fn with_writer(writer: TerminalWriter<TerminalWriterBackend>) -> Self {
        Self::new(writer, Default::default())
    }

    pub fn new(
        writer: TerminalWriter<TerminalWriterBackend>,
        components: Vec<Box<dyn Component + 'a>>,
    ) -> Self {
        Self { writer, components }
    }

    pub fn render(&mut self, sink: &mut impl std::io::Write) {
        /*
        ┌─┐
        │ │
        └─┘
            */

        self.writer.clear(sink);
        let inner_padding = 1;
        let component_padding = 1;
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

        sink.flush().unwrap();
    }

    pub fn trigger(&mut self, event: UserInput) -> Option<AppEvent> {
        for c in self.components.iter_mut() {
            match c.handle(&event) {
                UserEventHandled::ViewChange | UserEventHandled::Noop => {}
                UserEventHandled::AppEvent(appevent) => {
                    return Some(appevent);
                }
            }
        }

        None
    }

    pub fn add_component(&mut self, component: Box<dyn Component + 'a>) {
        self.components.push(component);
    }
}
