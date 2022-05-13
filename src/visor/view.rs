use super::Component;

pub struct TextView {
    contents: String,
}

impl TextView {
    pub fn new(contents: String) -> Self {
        Self { contents }
    }
}

impl Component for TextView {
    fn render(&self) -> String {
        self.contents.clone()
    }

    fn declare_dimensions(&self) -> (u16, u16) {
        let _lines = self.contents.lines().count();
        let _length = self.contents.lines().fold(0, |acc, next| {
            if acc < next.len() {
                return next.len();
            }
            acc
        });

        //(length as u16, lines as u16)
        (30, 30)
    }
}

pub struct Panel<C: Component> {
    name: String,
    dimensions: (u16, u16),
    component: C,
}

impl<C: Component> Component for Panel<C> {
    fn render(&self) -> String {
        self.component.render()
    }

    fn declare_dimensions(&self) -> (u16, u16) {
        let orig = self.component.declare_dimensions();
        (orig.0 + 4, orig.1 + 4)
    }

    fn handle(&mut self, e: &super::UserInput) -> super::UserEventHandled {
        self.component.handle(e)
    }
}
