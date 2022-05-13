use furnace::model::{Factory, Tile, patternline::PatternLine, CommonArea, Pickable};
use crate::util::eq_lists;

#[test]
fn test_picking_tiles_from_factory() {
    let mut factory = Factory::new([Tile::Yellow, Tile::Yellow, Tile::Green, Tile::Red]);
    let mut patternline = PatternLine::new_free(4);
    let mut common = CommonArea::new(vec![]);
    factory.pick(Tile::Yellow, &mut common, &mut patternline).unwrap();

    assert!(factory.is_empty());
    eq_lists(common.inspect(), &[Pickable::FirstPlayerToken, Pickable::Tile(Tile::Green), Pickable::Tile(Tile::Red)]);
}
