# Thoughts?
Files are prefixed with a sigil to denote the content type.
- `#`: Enum
- `&`: Struct
- `$`: Function
***
```tree
/
├── &curmax.rs
├── game/
│   ├── $add.rs
│   ├── $drop.rs
│   ├── $new.rs
│   ├── &mod.rs
│   └── update/
│       ├── $inv.rs
│       ├── $mod.rs
│       ├── $stats.rs
│       └── #event.rs
├── #item.rs
├── lib.rs
├── main.rs
├── #mode.rs
└── &stats.rs
```
## `&curmax.rs`
```rs
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
```
## `game/$add.rs`
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
## `game/$drop.rs`
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
## `game/$new.rs`
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
## `game/&mod.rs`
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
#[path = "$add.rs"] pub mod add;
#[path = "$drop.rs"] pub mod drop;
#[path = "$new.rs"] pub mod new;
#[path = "update/$mod.rs"] pub mod update;
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
## `game/update/$inv.rs`
```rs
use {
	crate::{
		game::{
			Game,
			update::event::Event::{Drop, PickUp},
		},
		item::Item,
	},
	eframe::egui::{Grid, Ui},
};
pub fn inv(game: &mut Game, ui: &mut Ui) {
	Grid::new("inventory").show(ui, |ui| {
		ui.heading("Inventory");
		ui.end_row();
		let mut events = vec![];
		let items: Vec<(Item, u8)> = game.inv.iter().map(|(i, c)| (*i, *c)).collect();
		for (item, count) in items {
			ui.label(format!("{count} × {item:?}"));
			if ui.button("Drop").clicked() {
				events.push(Drop(item));
			}
			ui.end_row();
		}
		ui.end_row();
		ui.heading("Nearby Items");
		ui.end_row();
		for (item, add) in &game.nearby {
			ui.label(format!("{add} × {item:?}"));
			if ui.button("Pick up").clicked() {
				events.push(PickUp(*item, *add));
			}
			ui.end_row();
		}
		for event in events {
			game.events.push(event);
		}
	});
}
```
## `game/update/$mod.rs`
```rs
use {
	crate::{
		game::{
			Game,
			update::{
				event::Event::{Drop, PickUp},
				inv::inv,
				stats::stats,
			},
		},
		mode::Mode::{Inv, Stats},
	},
	TopBottomSide::Top,
	eframe::{
		Frame,
		egui::{
			Align::Center,
			CentralPanel, Context, Layout,
			panel::{TopBottomPanel, TopBottomSide},
		},
	},
};
#[path = "#event.rs"] pub mod event;
#[path = "$inv.rs"] pub mod inv;
#[path = "$stats.rs"] pub mod stats;
pub fn update(game: &mut Game, ctx: &Context, _frame: &mut Frame) {
	game.events = vec![];
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
		for event in game.events.clone() {
			match event {
				PickUp(item, add) => game.add_item(item, add),
				Drop(item) => game.drop_item(item),
			}
		}
	});
}
```
## `game/update/$stats.rs`
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
## `game/update/#event.rs`
```rs
use crate::item::Item;
#[derive(Debug, Clone)]
pub enum Event {
	PickUp(Item, u8),
	Drop(Item),
}
```
## `#item.rs`
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
#[path = "&curmax.rs"] pub mod curmax;
#[path = "game/&mod.rs"] pub mod game;
#[path = "#item.rs"] pub mod item;
#[path = "#mode.rs"] pub mod mode;
#[path = "&stats.rs"] pub mod stats;
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
## `#mode.rs`
```rs
#[derive(Default, Debug)]
pub enum Mode {
	#[default]
	Stats,
	Inv,
}
```
## `&stats.rs`
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
