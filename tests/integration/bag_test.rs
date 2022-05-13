use azulrs::model::{bag::Bag, Factory, Tile};

use crate::util::eq_lists;

#[test]
pub fn test_drawing_reshuffles_if_not_enough_tiles_are_in_bag() {
    let mut bag = Bag::new(vec![Tile::Yellow, Tile::Red, Tile::Red], vec![Tile::Blue]);
    let mut factory = Factory::new_empty();
    bag.fill_factory(&mut factory);
    let tiles = factory.get_tiles().unwrap();
    eq_lists(tiles, &[Tile::Yellow, Tile::Red, Tile::Red, Tile::Blue]);
}
