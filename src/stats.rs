use crate::curmax::CurMax;
#[derive(Debug, Default)]
pub struct Stats {
	pub hp: CurMax<u8>,
	pub mana: CurMax<u8>,
	pub atk: u8,
	pub def: u8,
	pub gold: f32,
}
