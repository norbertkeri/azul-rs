use std::rc::Rc;

use furnace::model::{view::{TileView, FactoryView}, Tile, Factory};

use crate::helpers::expect_component;

#[test]
fn test_tileview() {
    let view = TileView {
        tile: Tile::Yellow,
        selected: false
    };

    expect_component(view, "Y");
}

#[test]
fn test_tileview_selected() {
    let view = TileView {
        tile: Tile::Yellow,
        selected: true
    };

    expect_component(view, "|Y|");
}

#[test]
fn test_factoryview_sorts() {
    let factory = Factory::new([Tile::Yellow, Tile::Green, Tile::Yellow, Tile::White]);
    let view = FactoryView::new(Rc::new(factory), None, false);

    expect_component(view, "GWYY");
}

#[test]
fn test_factoryview_selected() {
    let factory = Factory::new([Tile::Yellow, Tile::Green, Tile::Yellow, Tile::White]);
    let view = FactoryView::new(Rc::new(factory), Some(Tile::Yellow), false);

    expect_component(view, "GW|YY|");
}
