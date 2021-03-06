use azulrs::model::buildingarea::patternline::{PatternLine, PatternLineView};
use test_case::test_case;

use azulrs::model::{
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

#[test_case("    ââYY", PatternLine::new_taken(Tile::Yellow, 4, 2); "can render a taken patternline")]
#[test_case("      ââ", PatternLine::new_free(2); "can render a free patternline")]
fn test_patternline_rendering(
    expected: &str,
    patternline: azulrs::model::buildingarea::patternline::PatternLine,
) {
    let view = PatternLineView::new(&patternline, false);
    expect_component(view, expected);
}
