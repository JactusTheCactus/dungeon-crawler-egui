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
