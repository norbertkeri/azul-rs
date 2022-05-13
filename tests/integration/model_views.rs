use test_case::test_case;
use furnace::model::patternline::PatternLine;

use furnace::model::{
    view::{FactoryView, TileView},
    Factory, Tile,
};

use crate::helpers::expect_component;

#[test]
fn test_tileview() {
    let view = TileView {
        tile: Tile::Yellow,
        selected: false,
    };

    expect_component(view, "Y");
}

#[test]
fn test_tileview_selected() {
    let view = TileView {
        tile: Tile::Yellow,
        selected: true,
    };

    expect_component(view, "|Y|");
}

#[test]
fn test_factoryview_sorts() {
    let factory = Factory::new([Tile::Yellow, Tile::Green, Tile::Yellow, Tile::White]);
    let view = FactoryView::new(&factory, None, false);

    expect_component(view, "GWYY");
}

#[test]
fn test_factoryview_selected() {
    let factory = Factory::new([Tile::Yellow, Tile::Green, Tile::Yellow, Tile::White]);
    let view = FactoryView::new(&factory, Some(Tile::Yellow), false);

    expect_component(view, "GW|YY|");
}


#[test_case(" ☐☐YY", PatternLine::new_taken(Tile::Yellow, 4, 2); "can render a taken patternline")]
#[test_case("   ☐☐", PatternLine::new_free(2); "can render a free patternline")]
fn test_patternline_rendering(expected: &str, patternline: furnace::model::patternline::PatternLine) {
    let view = furnace::model::patternline::PatternLineView::new(&patternline);
    expect_component(view, expected);
}
