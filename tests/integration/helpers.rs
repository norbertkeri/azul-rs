use furnace::visor::{view::TextView, Component, Engine, backend::TestBackend};

pub fn to_textviews<const N: usize>(data: [&str; N]) -> Vec<Box<dyn Component>> {
    data.iter()
        .map(TextView::from)
        .map(|x| Box::new(x) as Box<_>)
        .collect()
}

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
