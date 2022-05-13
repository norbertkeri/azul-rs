use std::hash::BuildHasher;

use furnace::visor::{layout::Layout, view::{TextView, PanelBuilder}, Component};

use crate::helpers::expect_component;

fn assert_dimensions(component: &dyn Component, expected: (u16, u16)) {
    let dimensions = component.declare_dimensions();
    assert_eq!(dimensions, expected);
}

#[test]
fn test_layout_dimensions_horizontal() {
    let hello = TextView::new(String::from("Hello"));
    let world = TextView::new(String::from("World"));
    let stuff = [hello, world]
        .into_iter()
        .map(|x| Box::new(x) as Box<_>)
        .collect();

    let layout = Layout::horizontal(0, stuff);
    assert_dimensions(&layout, (10, 1));
}

#[test]
fn test_layout_dimensions_vertical() {
    let hello = TextView::new(String::from("Bye"));
    let world = TextView::new(String::from("World"));
    let stuff = [hello, world]
        .into_iter()
        .map(|x| Box::new(x) as Box<_>)
        .collect();

    let layout = Layout::vertical(0, stuff);
    assert_dimensions(&layout, (5, 2));
}

#[test]
fn test_layout_dimensions_horizontal_linebreak() {
    let hello = TextView::new(String::from("Bye\nWorld"));
    let world = TextView::new(String::from("Come back"));
    let stuff = [hello, world]
        .into_iter()
        .map(|x| Box::new(x) as Box<_>)
        .collect();

    let layout = Layout::horizontal(0, stuff);
    assert_dimensions(&layout, (14, 2));
}

#[test]
fn test_layout_dimensions_vertical_linebreak() {
    let hello = TextView::new(String::from("Bye\nWorld"));
    let world = TextView::new(String::from("Come back"));
    let stuff = [hello, world]
        .into_iter()
        .map(|x| Box::new(x) as Box<_>)
        .collect();

    let layout = Layout::vertical(0, stuff);
    assert_dimensions(&layout, (9, 3));
}

#[test]
fn test_panel_with_border_dimensions() {
    let hello = TextView::new(String::from("Hello"));
    let panel = PanelBuilder::default()
        .component(Box::new(hello))
        .build()
        .unwrap();

    assert_dimensions(&panel, (7, 3));
}
