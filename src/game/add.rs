use {
	crate::{game::Game, item::Item},
	std::collections::btree_map::Entry::{Occupied, Vacant},
};
pub fn add_item(game: &mut Game, item: Item, add: u8) {
	match game.inv.entry(item) {
		Occupied(mut e) => {
			*e.get_mut() = e.get().saturating_add(add);
		}
		Vacant(e) => {
			e.insert(add);
		}
	}
}
