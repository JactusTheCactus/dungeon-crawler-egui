use {
	Align::Center,
	TopBottomSide::Top,
	eframe::{App, Frame, egui},
	egui::{
		Align, CentralPanel, Context, Id, Layout,
		panel::{TopBottomPanel, TopBottomSide},
	},
	std::collections::BTreeMap,
};use eframe::run_native;use eframe::Result;use eframe::NativeOptions;use egui::Grid;use egui::RichText;
fn main() -> Result<()> {
	run_native(
		"Dungeon Crawler",
		NativeOptions::default(),
		Box::new(|_| Ok(Box::new(Game::new()))),
	)
}
#[derive(Debug)]
struct CurMax<T> {
	cur: T,
	max: T,
}
#[derive(Debug)]
struct Stats {
	hp: CurMax<u8>,
	mana: CurMax<u8>,
	atk: u8,
	def: u8,
	gold: f32,
}
#[derive(Default, Debug)]
enum Mode {
	StatsPage,
	#[default]
	Inv,
}
#[derive(Debug, Ord, PartialOrd, PartialEq, Eq, Clone)]
enum Item {
	Sword,
	Shield,
	Bow,
	Arrow,
	Helm,
	Chestpiece,
}
#[derive(Debug)]
struct Game {
	mode: Mode,
	stats: Stats,
	inv: BTreeMap<Item, u8>,
	nearby: BTreeMap<Item, u8>,
}
impl Game {
	fn new() -> Self {
		let mut inv = BTreeMap::<Item, u8>::new();
		for (item, count) in [(Item::Sword, 1), (Item::Shield, 1)] {
			inv.insert(item, count);
		}
		let mut nearby = BTreeMap::<Item, u8>::new();
		for (item, count) in [
			(Item::Sword, 1),
			(Item::Shield, 1),
			(Item::Bow, 1),
			(Item::Arrow, 5),
			(Item::Helm, 1),
			(Item::Chestpiece, 1),
		] {
			nearby.insert(item, count);
		}
		Self {
			mode: Mode::default(),
			stats: Stats {
				hp: CurMax { cur: 100, max: 100 },
				mana: CurMax { cur: 100, max: 100 },
				atk: 5,
				def: 0,
				gold: 0.,
			},
			inv,
			nearby,
		}
	}
}
impl App for Game {
	fn update(&mut self, ctx: &Context, _: &mut Frame) {
		CentralPanel::default().show(ctx, |ui| {
			TopBottomPanel::new(Top, Id::NULL).show_inside(ui, |ui| {
				ui.with_layout(Layout::left_to_right(Center), |ui| {
					if ui.button("Stats").clicked() {
						self.mode = Mode::StatsPage;
					}
					if ui.button("Inventory").clicked() {
						self.mode = Mode::Inv;
					}
				});
			});
			match self.mode {
				Mode::StatsPage => {
					let Stats {
						hp: CurMax {
							cur: hp_cur,
							max: hp_max,
						},
						mana: CurMax {
							cur: mana_cur,
							max: mana_max,
						},
						atk,
						def,
						gold,
					} = &self.stats;
					ui.heading("Stats");
					for i in [
						format!("HP: {hp_cur}/{hp_max}"),
						format!("Mana: {mana_cur}/{mana_max}"),
						format!("Atk: {atk}"),
						format!("Def: {def}"),
						format!("Gold: ${gold:.2}"),
					] {
						ui.label(RichText::new(i));
					}
				}
				Mode::Inv => {
					Grid::new(Id::NULL).show(ui, |ui| {
						ui.heading("Inventory");
						ui.end_row();
						for (name, count) in &mut self.inv {
							if *count != 0 {
								if ui.button("Drop").clicked() {
									*count = count.saturating_sub(1);
								}
								for i in [format!("{count}"), "×".into(), format!("{name:?}")] {
									ui.label(RichText::new(i));
								}
								ui.end_row();
							}
						}
						ui.end_row();
						ui.heading("Nearby Items");
						ui.end_row();
						for (item, add) in &self.nearby {
							if ui.button("Pick up").clicked() {
								if let Some(count) = self.inv.get(item) {
									self.inv.insert(item.clone(), count.saturating_add(*add));
								} else {
									self.inv.insert(item.clone(), 1);
								}
							}
							for i in [format!("{add}"), "×".into(), format!("{item:?}")] {
								ui.label(RichText::new(i));
							}
							ui.end_row();
						}
					});
				}
			}
		});
	}
}
