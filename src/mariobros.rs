use amethyst::{
	assets::{AssetStorage, Loader, Handle},
	core::timing::Time,
	core::transform::Transform,
	ecs::prelude::{Component, DenseVecStorage, Entity},
	prelude::*,
	renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
	ui::{Anchor, TtfFormat, UiText, UiTransform},
};

pub const ARENA_HEIGHT: f32 = 100.0;
pub const ARENA_WIDTH: f32 = 100.0;

pub const PLAYER_HEIGHT: f32 = 25.0;
pub const PLAYER_WIDTH: f32 = 18.0;

pub const BRICK_HEIGHT: f32 = 18.0;

/* 
=================================================================
====================== MarioBros Struct =========================
=================================================================
*/

#[derive(Default)]
pub struct MarioBros;

impl SimpleState for MarioBros {
	fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
		let world = data.world;

		world.register::<Player>();

		initialize_players(world);
		initialize_camera(world);
	}
}

/* 
=================================================================
===================== Mario Player Struct =======================
=================================================================
*/

#[derive(PartialEq, Eq)]
pub enum Character {
	Mario,
	Luigi,
}

pub struct Player {
	pub character: Character,
	pub width: f32,
	pub height: f32,
}

// Player Implementation
impl Player {
	fn new(character: Character) -> Player {
		Player {
			character,
			width: PLAYER_WIDTH,
			height: PLAYER_HEIGHT,
		}
	}
}

// Player Component Implementation
impl Component for Player {
	type Storage = DenseVecStorage<Self>;
}

/* 
=================================================================
===================== Additional Functions ======================
=================================================================
*/

// Initializes world view camera
fn initialize_camera(world: &mut World) {
	// Setup camera in a way that our screen covers whole arena and (0, 0) is in the bottom left.
	let mut transform = Transform::default();
	transform.set_translation_xyz(ARENA_WIDTH * 0.5, ARENA_HEIGHT * 0.5, 1.0);

	world
		.create_entity()
		.with(Camera::standard_2d(ARENA_WIDTH, ARENA_HEIGHT))
		.with(transform)
		.build();
}

// Add Luigi entity if needed
fn initialize_players(world: &mut World) {
	let mut mario_transform = Transform::default();

	// Correctly position the player
	let top = ARENA_HEIGHT - BRICK_HEIGHT;
	mario_transform.set_translation_xyz(ARENA_WIDTH * 0.5 - PLAYER_WIDTH * 0.5, top, 0.0);

	// Create a Mario entity
	world
		.create_entity()
		.with(Paddle::new(Character::Mario))
		.with(mario_transform)
		.build();
}










