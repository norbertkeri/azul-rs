use super::Component;

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
