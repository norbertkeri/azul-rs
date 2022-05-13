use furnace::visor::{Component, view::TextView, terminal_writer::TestBackend, Engine};
use pretty_assertions::assert_eq;

pub fn to_textviews<const N: usize>(data: [&str; N]) -> Vec<Box<dyn Component>> {
    data.iter()
        .map(TextView::from)
        .map(|x| Box::new(x) as Box<_>)
        .collect()
}

pub fn expect_component<T: Component + 'static>(component: T, expected: &str) {
    let backend = TestBackend::default();
    let mut engine = Engine::new(backend, Box::new(component));
    engine.render();
    let result = engine.get_contents();
    assert_eq!(result, expected);
}

