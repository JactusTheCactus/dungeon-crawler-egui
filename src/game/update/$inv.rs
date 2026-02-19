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
