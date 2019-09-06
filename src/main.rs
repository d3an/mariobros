use amethyst::{
	core::transform::TransformBundle,
	input::{InputBundle, StringBindings},
	prelude::*,
	renderer::{
		plugins::{RenderFlat2D, RenderToWindow},
		types::DefaultBackend,
		RenderingBundle,
	},
	utils::application_root_dir,
	ui::{RenderUi, UiBundle},
};

mod mariobros;

use crate::mariobros::MarioBros;

fn main() -> amethyst::Result<()> {
	// Starts command line logger
	amethyst::start_logger(Default::default());

	// Set up file structure
	let app_root = application_root_dir()?;
	let display_config_path = app_root.join("config").join("display.ron");
	let assets_dir = app_root.join("assets");

	let game_data = GameDataBuilder::default()
		.with_bundle(
			RenderingBundle::<DefaultBackend>::new()
				// The RenderToWindow plugin provides all the scaffolding for opening a window and drawing on it
				.with_plugin(
					RenderToWindow::from_config_path(display_config_path)
						.with_clear([0.0, 0.0, 0.0, 1.0]),
				)
				// RenderFlat2D plugin is used to render entities with a `SpriteRender` component.
				.with_plugin(RenderFlat2D::default()),
		)?;

	// Initialize game
	let mut game = Application::new(assets_dir, MarioBros, game_data)?;

	// Start the game loop
	game.run();

	Ok(())
}


