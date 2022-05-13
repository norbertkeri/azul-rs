use furnace::visor::{terminal_writer::{TestBackend, RootedRenderer}, Coords};

#[test]
fn test_rooted_renderer_calculates_drawn_area() {
    let mut renderer = TestBackend::default();
    let _rooted = RootedRenderer::new(&mut renderer, Coords::from((0, 0)));

}
