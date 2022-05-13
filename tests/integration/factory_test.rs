use crate::util::eq_lists;
use azulrs::model::{buildingarea::BuildingArea, CommonArea, Factory, Tile};

#[test]
fn test_picking_tiles_from_factory() {
    let mut bg = BuildingArea::default();

    let mut factory = Factory::new([Tile::Yellow, Tile::Yellow, Tile::Green, Tile::Red]);
    let mut common = CommonArea::default();

    bg.pick_factory(&mut factory, &mut common, 4, Tile::Yellow)
        .unwrap();

    assert!(factory.is_empty());
    eq_lists(
        common.inspect(),
        &[Tile::Green, Tile::Red, Tile::FirstPlayer],
    );
}
