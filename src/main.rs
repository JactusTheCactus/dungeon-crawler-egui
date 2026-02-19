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
