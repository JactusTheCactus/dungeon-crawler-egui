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
