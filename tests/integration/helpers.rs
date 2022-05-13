use azulrs::visor::{
    backend::TestBackend, renderer::RootedRenderer, view::TextView, Component, Coords, Engine,
};

pub fn to_textviews<const N: usize>(data: [&str; N]) -> Vec<Box<dyn Component>> {
    data.iter()
        .map(TextView::from)
        .map(|x| Box::new(x) as Box<_>)
        .collect()
}

#[track_caller]
pub fn expect_component<'a, T: Into<Box<dyn Component + 'a>>>(component: T, expected: &str) {
    let backend = TestBackend::default();
    let mut engine = Engine::new(backend, component);
    engine.render();
    let result = engine.get_contents();
    if result != expected {
        println!("=====\nExpected:\n{}\n=====\nGot:\n{}", expected, result);
        panic!("Rendered outputs don't match");
    }
}

#[track_caller]
pub fn assert_dimensions(component: &dyn Component, expected: (u16, u16)) {
    let mut backend = TestBackend::default();
    let mut writer = RootedRenderer::default_with_writer(&mut backend);
    let node_id = writer.render_into_layer("dimension_assert", Coords::new(0, 0), component);
    let dimensions = writer.get_drawn_area(node_id);
    assert_eq!(dimensions, expected);
}
