# Thoughts?
```tree
/
├── curmax.rs
├── game/
│   ├── add.rs
│   ├── drop.rs
│   ├── mod.rs
│   ├── new.rs
│   └── update/
│       ├── event.rs
│       ├── inv.rs
│       ├── mod.rs
│       └── stats.rs
├── item.rs
├── lib.rs
├── main.rs
├── mode.rs
└── stats.rs
```
## `curmax.rs`
```rs
use {
	num_traits::ops::saturating::{SaturatingAdd, SaturatingMul, SaturatingSub},
	std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};
#[derive(Debug, Default, Copy, Clone)]
pub struct CurMax<T> {
	pub cur: T,
	pub max: T,
}
impl<T: Copy + Ord> CurMax<T> {
	pub fn new(cur: T, max: T) -> Self {
		Self {
			cur: cur.min(max),
			max,
		}
	}
	pub fn set(&mut self, value: T) {
		self.cur = value.min(self.max);
	}
	pub fn get(&self) -> (T, T) {
		(self.cur, self.max)
	}
}
impl<
	T: Copy
		+ Ord
		+ Add
		+ AddAssign
		+ Sub
		+ SubAssign
		+ Mul
		+ MulAssign
		+ Div<Output = T>
		+ DivAssign
		+ SaturatingAdd
		+ SaturatingSub
		+ SaturatingMul,
> CurMax<T>
{
	pub fn add(&self, n: T) -> Self {
		CurMax::new(self.cur.saturating_add(&n).min(self.max), self.max)
	}
	pub fn add_assign(&mut self, n: T) {
		self.cur = self.add(n).cur;
	}
	pub fn sub(&self, n: T) -> Self {
		CurMax::new(self.cur.saturating_sub(&n), self.max)
	}
	pub fn sub_assign(&mut self, n: T) {
		self.cur = self.sub(n).cur;
	}
	pub fn mul(&self, n: T) -> Self {
		CurMax::new(self.cur.saturating_mul(&n).min(self.max), self.max)
	}
	pub fn mul_assign(&mut self, n: T) {
		self.cur = self.mul(n).cur;
	}
	pub fn div(&self, n: T) -> Self {
		CurMax::new(self.cur / n, self.max)
	}
	pub fn div_assign(&mut self, n: T) {
		self.cur = self.div(n).cur;
	}
}
```
## `game/add.rs`
```rs
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
```
## `game/drop.rs`
```rs
use crate::game::{Game, Item};
pub fn drop_item(game: &mut Game, item: Item) {
	if let Some(count) = game.inv.get_mut(&item) {
		*count = count.saturating_sub(1);
		if *count == 0 {
			game.inv.remove(&item);
		}
	}
}
```
## `game/mod.rs`
```rs
use {
	crate::{
		game::{
			add::add_item,
			drop::drop_item,
			new::new,
			update::{event::Event, update},
		},
		item::Item,
		mode::Mode,
		stats::Stats,
	},
	eframe::{App, Frame, egui::Context},
	std::collections::BTreeMap,
};
pub mod add;
pub mod drop;
pub mod new;
pub mod update;
#[derive(Debug, Default)]
pub struct Game {
	pub mode: Mode,
	pub stats: Stats,
	pub inv: BTreeMap<Item, u8>,
	pub nearby: BTreeMap<Item, u8>,
	pub events: Vec<Event>,
}
impl Game {
	pub fn new() -> Self {
		new()
	}
	pub fn add_item(&mut self, item: Item, add: u8) {
		add_item(self, item, add);
	}
	pub fn drop_item(&mut self, item: Item) {
		drop_item(self, item);
	}
}
impl App for Game {
	fn update(&mut self, ctx: &Context, frame: &mut Frame) {
		update(self, ctx, frame);
	}
}
```
## `game/new.rs`
```rs
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
```
## `game/update/event.rs`
```rs
use crate::item::Item;
#[derive(Debug, Clone)]
pub enum Event {
	PickUp(Item, u8),
	Drop(Item),
}
```
## `game/update/inv.rs`
```rs
use {
	crate::{game::Game, item::Item},
	eframe::egui::{Grid, Ui},
};
pub fn inv(game: &mut Game, ui: &mut Ui) {
	Grid::new("inventory").show(ui, |ui| {
		ui.heading("Inventory");
		ui.end_row();
		let items: Vec<(Item, u8)> = game.inv.iter().map(|(i, c)| (*i, *c)).collect();
		for (item, count) in items {
			ui.label(format!("{count} × {item:?}"));
			if ui.button("Drop").clicked() {
				game.drop_item(item);
			}
			ui.end_row();
		}
		ui.end_row();
		ui.heading("Nearby Items");
		ui.end_row();
		for (item, add) in game.nearby.clone() {
			ui.label(format!("{add} × {item:?}"));
			if ui.button("Pick up").clicked() {
				game.add_item(item, add);
			}
			ui.end_row();
		}
	});
}
```
## `game/update/mod.rs`
```rs
use {
	crate::{
		game::{
			Game,
			update::{inv::inv, stats::stats},
		},
		mode::Mode::{Inv, Stats},
	},
	eframe::{
		Frame,
		egui::{
			Align::Center,
			CentralPanel, Context, Layout,
			panel::{TopBottomPanel, TopBottomSide::Top},
		},
	},
};
pub mod event;
pub mod inv;
pub mod stats;
pub fn update(game: &mut Game, ctx: &Context, _frame: &mut Frame) {
	CentralPanel::default().show(ctx, |ui| {
		TopBottomPanel::new(Top, "nav").show_inside(ui, |ui| {
			ui.with_layout(Layout::left_to_right(Center), |ui| {
				for (label, page) in [("Stats", Stats), ("Inventory", Inv)] {
					if ui.button(label).clicked() {
						game.mode = page;
					}
				}
			});
		});
		match game.mode {
			Stats => stats(game, ui),
			Inv => inv(game, ui),
		}
	});
}
```
## `game/update/stats.rs`
```rs
use {
	crate::{game::Game, stats::Stats},
	eframe::egui::Ui,
};
pub fn stats(game: &mut Game, ui: &mut Ui) {
	let Stats {
		hp,
		mana,
		atk,
		def,
		gold,
	} = game.stats;
	let ((hp_cur, hp_max), (mana_cur, mana_max)) = (hp.get(), mana.get());
	ui.heading("Stats");
	ui.label(format!("HP: {hp_cur}/{hp_max}"));
	ui.label(format!("Mana: {mana_cur}/{mana_max}"));
	ui.label(format!("Atk: {atk}"));
	ui.label(format!("Def: {def}"));
	ui.label(format!("Gold: ${gold:.2}"));
}
```
## `item.rs`
```rs
#[derive(Debug, Ord, PartialOrd, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Item {
	Sword,
	Shield,
	Bow,
	Arrow,
	Helm,
	Chestpiece,
}
```
## `lib.rs`
```rs
pub mod curmax;
pub mod game;
pub mod item;
pub mod mode;
pub mod stats;
```
## `main.rs`
```rs
use {
	dungeon_crawler::game::Game,
	eframe::{NativeOptions as Options, Result, run_native as run},
};
fn main() -> Result<()> {
	run(
		"Dungeon Crawler",
		Options::default(),
		Box::new(|_| Ok(Box::new(Game::new()))),
	)
}
```
## `mode.rs`
```rs
#[derive(Default, Debug)]
pub enum Mode {
	#[default]
	Stats,
	Inv,
}
```
## `stats.rs`
```rs
use crate::curmax::CurMax;
#[derive(Debug, Default)]
pub struct Stats {
	pub hp: CurMax<u8>,
	pub mana: CurMax<u8>,
	pub atk: u8,
	pub def: u8,
	pub gold: f32,
}
```
