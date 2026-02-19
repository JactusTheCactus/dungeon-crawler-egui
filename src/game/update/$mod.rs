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
