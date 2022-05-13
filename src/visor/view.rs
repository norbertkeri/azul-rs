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

        let mut i = 1;
        while let Some(line) = iter.next() {
            writer.write(line);
            if iter.peek().is_some() {
                writer.reset_cursor_to_root();
                writer.set_cursor_to((1, i + 1).into());
            }
            i += 1;
        }
    }
}

#[derive(Builder)]
#[builder(pattern = "owned")]
pub struct Panel<'a> {
    #[builder(setter(into, strip_option), default)]
    name: Option<String>,
    #[builder(default)]
    padding: u16,
    component: Box<dyn Component + 'a>,
}

impl<'a> From<Panel<'a>> for Box<dyn Component + 'a> {
    fn from(val: Panel<'a>) -> Self {
        Box::new(val)
    }
}

impl Component for Panel<'_> {
    fn handle(&mut self, e: &super::UserInput) -> super::UserEventHandled {
        self.component.handle(e)
    }

    fn render<'a>(&'a self, writer: &'a mut RootedRenderer) {
        let (vertical_padding, horizontal_padding) = (self.padding, self.padding);

        let subroot = Coords(1 + horizontal_padding, 1 + vertical_padding); // +1 for the border
        let node_id = writer.render_into_layer("panel-content", subroot, self.component.as_ref());
        let (content_w, content_h) = writer.get_drawn_area(node_id);
        let (mut full_w, full_h) = (
            content_w + 2 + horizontal_padding * 2,
            content_h + 2 + vertical_padding * 2,
        );

        full_w = self.draw_header(writer, full_w);

        for i in 2..full_h {
            writer.set_cursor_to(Coords(1, i));
            writer.write("│");
            writer.set_cursor_to(Coords(full_w, i));
            writer.write("│");
        }

        writer.set_cursor_to(Coords(1, full_h));
        writer.write("└");
        writer.write(&"─".repeat((full_w - 2).into()));
        writer.write("┘");
    }
}

impl Panel<'_> {
    fn draw_header(&self, writer: &mut RootedRenderer, total_width: u16) -> u16 {
        let mut to_write = String::from("┌");
        match &self.name {
            Some(name) => {
                // (total_length - length of title - two || characters - two spaces - two corners) / 2
                let total_width_i: i16 = total_width.try_into().unwrap();
                let name_len: i16 = name.len().try_into().unwrap();
                let dash_length = (total_width_i - 2 - 4 - name_len) / 2;
                let actual = dash_length.try_into().unwrap_or(0);
                to_write.push_str(&format!(
                    "{}| {} |{}",
                    "─".repeat(actual),
                    name,
                    "─".repeat(actual)
                ));
                /* If the name of the panel has even characters, and the width of the box is odd (or vice versa, they don't
                 * match), we have to add an extra -, otherwise the top line won't line up with the bottom.
                 * This will make the title slightly off center, nothing we can do here.
                 * I'm going to add the extra - on the right side.
                 */
                if total_width as usize % 2 != name.len() % 2 {
                    to_write.push('─');
                }
                //actual_panel_width
            }
            None => {
                let draw_amount = match total_width {
                    0..=2 => 0,
                    n => n - 2,
                } as usize;
                to_write.push_str(&"─".repeat(draw_amount));
                //total_width
            }
        };
        to_write.push('┐');
        writer.write(&to_write);
        to_write.chars().count() as u16
    }
}
