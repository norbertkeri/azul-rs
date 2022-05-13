use furnace::visor::{view::{Panel, TextView, PanelDimensions}};

use crate::helpers::expect_component;

#[test]
fn test_oneline_panel() {
    let hello = TextView::new(String::from("Hello"));
    let panel = Panel::new("Hello".into(), 0, PanelDimensions::ShrinkWrap, Box::new(hello));
    let expected = r#"
┌───────┐
│ Hello │
└───────┘"#.trim_start();
    expect_component(panel, expected);
}
