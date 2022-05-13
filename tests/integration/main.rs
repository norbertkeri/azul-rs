use std::io::stdout;

use furnace::visor::{
    terminal_writer::{TerminalWriter, TestBackend},
    view::TextView,
    Engine,
};

mod testview;

#[test]
fn test_me() {
    let hello = TextView::new(String::from("Hello"));
    let world = TextView::new(String::from("world"));
    let backend = TestBackend::default();
    let writer = TerminalWriter::new(backend);
    let mut engine = Engine::new(
        writer,
        vec![Box::new(hello) as Box<_>, Box::new(world) as Box<_>],
    );

    let mut stdout = stdout();

    engine.render(&mut stdout);

    let result = engine.get_contents();

    assert_eq!(result, "Helloworld");
}
