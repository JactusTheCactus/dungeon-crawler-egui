use {
	crate::{
		curmax::CurMax,
		game::Game,
		item::Item::{Arrow, Bow, Chestpiece, Helm, Shield, Sword},
		mode::Mode,
		stats::Stats,
	},
	std::collections::BTreeMap,
};
pub fn new() -> Game {
	Game {
		mode: Mode::default(),
		stats: Stats {
			hp: CurMax::new(100, 100),
			mana: CurMax::new(100, 100),
			atk: 5,
			def: 0,
			gold: 0.,
		},
		inv: BTreeMap::from([(Sword, 1), (Shield, 1)]),
		nearby: BTreeMap::from([
			(Sword, 1),
			(Shield, 1),
			(Bow, 1),
			(Arrow, 5),
			(Helm, 1),
			(Chestpiece, 1),
		]),
		events: vec![],
	}
}
