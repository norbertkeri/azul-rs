use furnace::visor::{
    terminal_writer::{RootedRenderer, TestBackend},
    Coords,
};

#[test]
fn test_rooted_renderer_calculates_drawn_area() {
    let mut renderer = TestBackend::default();
    let _rooted = RootedRenderer::new(&mut renderer, Coords::from((0, 0)));
}
