use crate::game::{Game, Item};
pub fn drop_item(game: &mut Game, item: Item) {
	if let Some(count) = game.inv.get_mut(&item) {
		*count = count.saturating_sub(1);
		if *count == 0 {
			game.inv.remove(&item);
		}
	}
}
