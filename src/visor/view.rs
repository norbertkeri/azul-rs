use crate::visor::{renderer::RootedRenderer, Coords};
use derive_builder::Builder;
use std::fmt::Debug;

use super::Component;

pub struct TextView {
    contents: String,
}

impl From<TextView> for Box<dyn Component> {
    fn from(s: TextView) -> Self {
        Box::new(s)
    }
}

impl<S: ToString> From<S> for TextView {
    fn from(s: S) -> Self {
        TextView::new(s.to_string())
    }
}

impl TextView {
    pub fn new(contents: String) -> Self {
        Self { contents }
    }

    pub fn update_contents(&mut self, contents: String) {
        self.contents = contents;
    }
}

impl Component for TextView {
    fn render(&self, writer: &mut RootedRenderer) {
        let mut iter = self.contents.lines().peekable();

        while let Some(line) = iter.next() {
            writer.write(line);
            if iter.peek().is_some() {
                writer.set_cursor_to((0, 1).into());
            }
        }
    }

    fn declare_dimensions(&self) -> (u16, u16) {
        let lines = self.contents.lines().count();
        let length = self.contents.lines().fold(0, |acc, next| {
            if acc < next.len() {
                return next.len();
            }
            acc
        });

        (length as u16, lines as u16)
    }
}

pub enum PanelDimensions {
    Static(u16, u16),
    ShrinkWrap,
}

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct Panel<'a> {
    #[builder(setter(into, strip_option), default)]
    name: Option<String>,
    #[builder(default)]
    padding: u16,
    #[builder(default = "PanelDimensions::ShrinkWrap")]
    dimensions: PanelDimensions,
    component: Box<dyn Component + 'a>,
}

impl<'a> From<Panel<'a>> for Box<dyn Component + 'a> {
    fn from(val: Panel<'a>) -> Self {
        Box::new(val)
    }
}

impl Component for Panel<'_> {
    /**
     * Calculate total length of component, by:
     * Base length + 2*padding, + 2 for borders
     */
    fn declare_dimensions(&self) -> (u16, u16) {
        let title_length: u16 = self
            .name
            .as_ref()
            .map(|x| x.len())
            .map(TryInto::try_into)
            .map(|x| x.unwrap_or(0))
            .unwrap_or(0);

        let mut base = match self.dimensions {
            PanelDimensions::Static(w, h) => (w, h),
            PanelDimensions::ShrinkWrap => self.component.declare_dimensions(),
        };
        if base.0 < title_length + 4 {
            base.0 = title_length + 4;
        }
        let (vertical_padding, horizontal_padding) = (self.padding, self.padding);
        (
            base.0 + (horizontal_padding * 2) + 2,
            base.1 + (vertical_padding * 2) + 2,
        )
    }

    fn handle(&mut self, e: &super::UserInput) -> super::UserEventHandled {
        self.component.handle(e)
    }

    fn render(&self, writer: &mut RootedRenderer) {
        let (w, h) = self.declare_dimensions();
        self.draw_header(writer);

        let (vertical_padding, horizontal_padding) = (self.padding, self.padding);

        for i in 1..=vertical_padding {
            writer.reset_cursor_to_root();
            writer.set_cursor_to(Coords(0, i));
            writer.write("│");
            writer.set_cursor_to(Coords(w - 1, i));
            writer.write("│");
        }

        let subroot = Coords(1 + horizontal_padding, 1 + vertical_padding); // +1 for the border
        let mut rooted = RootedRenderer::subrooted(writer, subroot);
        rooted.reset_cursor_to_root();
        self.component.render(&mut rooted);
        writer.reset_cursor_to_root();
        for i in (1 + vertical_padding)..=(h - 2) {
            let beginning = Coords(0, i);
            writer.set_cursor_to(beginning);
            writer.write("│");
            let end = Coords(w - 1, i);
            writer.set_cursor_to(end);
            writer.write("│");
        }
        writer.set_cursor_to(Coords(0, h - 1));
        writer.write("└");
        writer.write(&"─".repeat((w - 2).into()));
        writer.write("┘");
    }
}

impl Panel<'_> {
    fn draw_header(&self, writer: &mut RootedRenderer) {
        let (w, _h) = self.declare_dimensions();
        writer.write("┌");
        match &self.name {
            Some(name) => {
                // total length - (two || characters with spaces + length of title) - panel borders / 2
                let width = (w - (4 + name.len() as u16) - 2) / 2;
                writer.write(&format!(
                    "{}| {} |{}",
                    "─".repeat(width.into()),
                    name,
                    "─".repeat(width.into())
                ));
                /* If the name of the panel has even characters, and the width of the box is odd (or vice versa, they don't
                 * match), we have to add an extra -, otherwise the top line won't line up with the bottom.
                 * This will make the title slightly off center, nothing we can do here.
                 * I'm going to add the extra - on the right side.
                 */
                if w as usize % 2 != name.len() % 2 {
                    writer.write("─");
                }
            }
            None => writer.write(&"─".repeat((w - 2).into())),
        }
        writer.write("┐");
    }
}
