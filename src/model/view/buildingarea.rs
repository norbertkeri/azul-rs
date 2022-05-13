use crate::visor::Component;


pub struct BuildingAreaView {
}

impl<'a> Component<'a> for BuildingAreaView {
    fn render<'b: 'a>(&self, writer: &'b mut dyn crate::visor::terminal_writer::TerminalBackend) {
    }

    fn declare_dimensions(&self) -> (u16, u16) {
        (5,5)
    }

    fn handle(&mut self, _event: &crate::visor::UserInput) -> crate::visor::UserEventHandled {
        crate::visor::UserEventHandled::Noop
    }
}
