use crate::item::Item;
#[derive(Debug, Clone)]
pub enum Event {
	PickUp(Item, u8),
	Drop(Item),
}
